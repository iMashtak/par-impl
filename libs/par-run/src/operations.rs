use std::{fmt::Debug, pin::Pin};

use arcstr::ArcStr;
use futures::{FutureExt, StreamExt as _};
use tokio::sync::oneshot;

pub enum Value {
    Unit,
    Continue,
    Label { x: ArcStr },

    Str { x: ArcStr },
    I32 { x: i32 },
    U64 { x: u64 },

    Receiver { r: Receiver },
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit => write!(f, "Unit"),
            Self::Continue => write!(f, "Continue"),
            Self::Label { x } => f.debug_struct("Label").field("x", x).finish(),
            Self::Str { x } => f.debug_struct("Str").field("x", x).finish(),
            Self::I32 { x } => f.debug_struct("I32").field("x", x).finish(),
            Self::U64 { x } => f.debug_struct("U64").field("x", x).finish(),
            Self::Receiver { r: _ } => f.debug_struct("Receiver").finish(),
        }
    }
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
pub type Sender = Pin<Box<dyn Future<Output = kanal::AsyncSender<Value>> + Send + 'static>>;
pub type Receiver = Pin<Box<dyn Future<Output = kanal::AsyncReceiver<Value>> + Send + 'static>>;

fn raw_chan() -> (kanal::AsyncSender<Value>, kanal::AsyncReceiver<Value>) {
    kanal::bounded_async(1024)
}

fn future_chan() -> (Sender, Receiver) {
    let (s, r) = kanal::bounded_async(1024);
    (sender(s), receiver(r))
}

fn sender(s: kanal::AsyncSender<Value>) -> Sender {
    tokio::spawn(async move { s }).map(|x| x.unwrap()).boxed()
}

fn receiver(r: kanal::AsyncReceiver<Value>) -> Receiver {
    tokio::spawn(async move { r }).map(|x| x.unwrap()).boxed()
}

fn loop_chan<T>() -> (LoopSender<T>, LoopReceiver<T>) {
    oneshot::channel()
}

pub fn value(value: Value) -> (Sender, Receiver) {
    let (s, r) = raw_chan();
    let s = tokio::spawn(async move {
        s.send(value).await.unwrap();
        s
    })
    .map(|x| x.unwrap())
    .boxed();
    (s, receiver(r))
}

pub async fn send_value(s: Sender, value: Value) -> Sender {
    let s = s.await;
    s.send(value).await.unwrap();
    sender(s)
}

pub async fn send_one(s: Sender, r: Receiver) -> (Sender, Receiver) {
    let s = s.await;
    let r = r.await;
    let x = r.recv().await.unwrap();
    s.send(x).await.unwrap();
    (sender(s), receiver(r))
}

pub fn recv(r: Receiver) -> (Receiver, Sender, Receiver) {
    let (recv_s, recv_r) = raw_chan();
    let cloned_recv_s = recv_s.clone();
    let r = tokio::spawn(async move {
        let r = r.await;
        let x = r.recv().await.unwrap();
        match x {
            Value::Receiver { r: inner_r } => {
                link(sender(cloned_recv_s), inner_r);
                r
            }
            x => {
                cloned_recv_s.send(x).await.unwrap();
                r
            }
        }
    })
    .map(|x| x.unwrap())
    .boxed();
    (r, sender(recv_s), receiver(recv_r))
}

pub fn link(s: Sender, r: Receiver) {
    tokio::spawn(async move {
        let r = r.await;
        let s = s.await;
        while let Some(x) = r.stream().next().await {
            s.send(x).await.unwrap();
        }
    });
}

pub fn chan<Closure: Send + 'static, Fut: Future<Output = ()> + Send>(
    c: Closure,
    f: impl FnOnce(Sender, Receiver, Closure) -> Fut + Send + 'static,
) -> (Sender, Receiver) {
    let (func_s, func_r) = future_chan();
    let (result_s, result_r) = future_chan();
    tokio::spawn(async move {
        f(result_s, func_r, c).await;
    });
    (func_s, result_r)
}

pub async fn case<Closure: Send + 'static, Fut: Future<Output = Receiver> + Send>(
    r: Receiver,
    c: Closure,
    f: impl FnOnce(ArcStr, Receiver, Closure) -> Fut + Send + 'static,
) -> Receiver {
    let r = r.await;
    let label = r.recv().await.unwrap();
    let mut r = receiver(r);
    match label {
        Value::Label { x } => {
            r = f(x, r, c).await;
        }
        _ => unreachable!(),
    };
    r
}

pub async fn begin<Closure: Send + 'static, Fut: Future<Output = Receiver> + Send>(
    r: Receiver,
    mut c: Closure,
    f: impl Fn(LoopSender<Option<Closure>>, Receiver, Closure) -> Fut + Send + 'static,
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
    let r = r.await;
    while let Some(_) = r.stream().next().await {}
}
