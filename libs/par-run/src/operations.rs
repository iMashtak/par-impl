use arcstr::ArcStr;
use futures::StreamExt as _;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Value {
    Unit,
    Continue,
    Label { x: ArcStr },

    Str { x: ArcStr },
    I32 { x: i32 },
    U64 { x: u64 },

    Receiver { r: Receiver },
}

impl Value {
    pub fn label(label: &'static str) -> Self {
        Self::Label { x: label.into() }
    }

    pub fn into_receiver(self) -> Receiver {
        match self {
            Value::Receiver { r } => r,
            _ => unreachable!(),
        }
    }
}

pub type LoopSender<T> = oneshot::Sender<T>;
pub type LoopReceiver<T> = oneshot::Receiver<T>;
pub type Sender = kanal::AsyncSender<Value>;
pub type Receiver = kanal::AsyncReceiver<Value>;

pub fn raw_chan() -> (Sender, Receiver) {
    kanal::bounded_async(512)
}

fn loop_chan<T>() -> (LoopSender<T>, LoopReceiver<T>) {
    oneshot::channel()
}

pub fn value(value: Value) -> (Sender, Receiver) {
    let (s, r) = raw_chan();
    let c = s.clone();
    tokio::spawn(async move {
        c.send(value).await.unwrap();
    });
    (s, r)
}

pub fn link(s: Sender, r: Receiver) {
    tokio::spawn(async move {
        while let Some(x) = r.stream().next().await {
            s.send(x).await.unwrap();
        }
    });
}

pub fn chan<Closure: Send + 'static, Fut: Future<Output = ()> + Send>(
    c: Closure,
    f: impl FnOnce(Sender, Receiver, Closure) -> Fut + Send + 'static,
) -> (Sender, Receiver) {
    let (func_s, func_r) = raw_chan();
    let (result_s, result_r) = raw_chan();
    tokio::spawn(async move {
        f(result_s, func_r, c).await;
    });
    (func_s, result_r)
}

pub async fn case<Closure, Fut: Future<Output = Receiver> + Send>(
    mut r: Receiver,
    c: Closure,
    f: impl FnOnce(ArcStr, Receiver, Closure) -> Fut + Send + 'static,
) -> Receiver {
    let label = r.recv().await.unwrap();
    match label {
        Value::Label { x: label } => {
            r = f(label, r, c).await;
        }
        x => {
            println!("incorrect label: {:?}", x);
            unreachable!()
        }
    }
    r
}

pub async fn begin<Closure>(
    r: Receiver,
    mut c: Closure,
    f: impl AsyncFn(LoopSender<Option<Closure>>, Receiver, Closure) -> Receiver,
) -> Receiver {
    let mut current = r;
    loop {
        let (loop_s, loop_r) = loop_chan();
        current = f(loop_s, current, c).await;
        let next = loop_r.await.unwrap();
        if let Some(next) = next {
            c = next;
        } else {
            break;
        }
    }
    current
}

pub async fn drain(r: Receiver) {
    while let Some(_) = r.stream().next().await {}
}
