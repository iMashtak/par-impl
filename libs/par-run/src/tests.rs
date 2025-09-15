use crate::operations::{Value, begin, case, chan, drain, link, raw_chan, send};

#[tokio::test]
async fn test_list_builder() {
    println!("start");
    // Init
    let (main_s, main_r) = raw_chan();

    // Program
    let (result_s, result_r) = raw_chan();

    let (list_builder_s, list_builder_r) = chan((), async move |result_s, func_r, _| {
        let (append_s, append_r) = chan((), async move |result_s, func_r, _| {
            let x = func_r.recv().await.unwrap();
            link(result_s, x.await.into_receiver());
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
                                let x = input_r.recv().await.unwrap();
                                (append_s, append_r) = chan(
                                    (append_s, append_r, x),
                                    async move |result_s, func_r, (append_s, append_r, x)| {
                                        let xs = func_r.recv().await.unwrap();
                                        let (_, inner_r) =
                                            chan((x, xs), async move |result_s, _, (x, xs)| {
                                                send(&result_s, Value::label("item")).await;
                                                result_s.send(x).await.unwrap();
                                                link(result_s, xs.await.into_receiver());
                                            });
                                        send(&append_s, Value::Receiver { r: inner_r }).await;
                                        link(result_s, append_r);
                                    },
                                );
                                loop_s.send(Some((result_s, append_s, append_r))).unwrap();
                            }
                            "build" => {
                                let (_, inner_r) = chan((), async move |result_s, _, _| {
                                    send(&result_s, Value::label("end")).await;
                                    send(&result_s, Value::Unit).await;
                                });
                                send(&append_s, Value::Receiver { r: inner_r }).await;
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
    for i in 0..5000 {
        send(&list_builder_s, Value::label("add")).await;
        send(
            &list_builder_s,
            Value::Str {
                x: i.to_string().into(),
            },
        )
        .await;
    }
    send(&list_builder_s, Value::label("build")).await;

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
                            let x = input_r.recv().await.unwrap();
                            let (print_s, print_r) = print_boxed();
                            print_s.send(x).await.unwrap();
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

    send(&result_s, Value::Unit).await;

    link(main_s, result_r);

    drain(list_builder_r).await;

    // End
    println!("before main");
    let result = main_r.recv().await.unwrap().await;
    println!("main exit: {:?}", result);
}
