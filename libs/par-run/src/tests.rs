use crate::operations::{begin, case, chan, drain, link, raw_chan, recv, send_value, Value};

#[tokio::test]
async fn test_list_builder() {
    println!("start");
    // Init
    let (main_s, main_r) = raw_chan();

    // Program
    let (result_s, result_r) = raw_chan();

    let (list_builder_s, list_builder_r) = chan((), async move |result_s, func_r, _| {
        let (append_s, append_r) = chan((), async move |result_s, func_r, _| {
            let x_r = recv(&func_r).await;
            link(result_s, x_r);
        });
        let func_r = begin(
            func_r,
            (result_s, append_s, append_r),
            async move |loop_s, mut input_r, (result_s, append_s, append_r)| {
                input_r = case(
                    input_r,
                    (result_s, append_s, append_r),
                    async move |label, input_r, (result_s, mut append_s, mut append_r)| {
                        match label.as_str() {
                            "add" => {
                                let x_r = recv(&input_r).await;
                                (append_s, append_r) = chan(
                                    (append_s, append_r, x_r),
                                    async move |result_s, func_r, (append_s, append_r, x_r)| {
                                        let xs_r = recv(&func_r).await;
                                        let (_, inner_r) =
                                            chan((x_r, xs_r), async move |result_s, _, (x_r, xs_r)| {
                                                send_value(&result_s, Value::label("item")).await;
                                                result_s.send(x_r.recv().await.unwrap()).await.unwrap();
                                                link(result_s, xs_r);
                                            });
                                        send_value(&append_s, Value::Receiver { r: inner_r }).await;
                                        link(result_s, append_r);
                                    },
                                );
                                loop_s.send(Some((result_s, append_s, append_r))).unwrap();
                            }
                            "build" => {
                                let (_, inner_r) = chan((), async move |result_s, _, _| {
                                    send_value(&result_s, Value::label("end")).await;
                                    send_value(&result_s, Value::Unit).await;
                                });
                                send_value(&append_s, Value::Receiver { r: inner_r }).await;
                                link(result_s, append_r);
                                loop_s.send(None).unwrap();
                            }
                            _ => unreachable!(),
                        }
                        input_r
                    },
                )
                .await;
                input_r
            },
        );
        drain(func_r).await;
    });

    let print_boxed = || {
        chan((), async move |_, func_r, _| {
            let x = func_r.recv().await.unwrap().await;
            println!("-> {:?}", x);
        })
    };

    println!("before list_builder_s usage");
    for i in 0..10000 {
        send_value(&list_builder_s, Value::label("add")).await;
        send_value(
            &list_builder_s,
            Value::Str {
                x: i.to_string().into(),
            },
        )
        .await;
    }
    send_value(&list_builder_s, Value::label("build")).await;

    println!("before print begin");
    let list_builder_r = begin(
        list_builder_r,
        (print_boxed,),
        async move |loop_s, input_r, (print_boxed,)| {
            let input_r = case(
                input_r,
                (print_boxed,),
                async move |label, input_r, (print_boxed,)| {
                    match label.as_str() {
                        "item" => {
                            let x_r = recv(&input_r).await;
                            let (print_s, print_r) = print_boxed();
                            print_s.send(x_r.recv().await.unwrap()).await.unwrap();
                            if let Err(_) = loop_s.send(Some((print_boxed,))) {
                                unreachable!()
                            }
                            drain(print_r).await;
                        }
                        "end" => {
                            let _ = input_r.recv().await.unwrap();
                            if let Err(_) = loop_s.send(None) {
                                unreachable!()
                            }
                        }
                        _ => unreachable!(),
                    };
                    input_r
                },
            )
            .await;
            input_r
        },
    );

    send_value(&result_s, Value::Unit).await;

    link(main_s, result_r);

    drain(list_builder_r).await;

    // End
    println!("before main");
    let result = main_r.recv().await.unwrap().await;
    println!("main exit: {:?}", result);
}
