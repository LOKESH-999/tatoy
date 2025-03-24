use std::{sync::atomic:: AtomicUsize, task::Poll};

use super::super::mpsc::Sender;

type SenderType = Sender<Box<dyn Message>>;

pub trait Message {
    fn to_bytes(&self)->&[u8];
}

#[async_trait::async_trait]
pub trait Actor{
    async fn recieve(&self,msg:Box<dyn Message>,ctx:Context);
}


// #[derive(Clone)]
pub struct Addr{
    sender:SenderType
}
impl Addr{
    pub fn send(&self,data:Box<dyn Message> ){
        self.sender.send(data);
    }
}

pub struct Context{
    addr: Addr
}

impl Context {
    pub fn new(addr:Addr)->Self{
        Context { addr }
    }
    pub fn addr(&self)->Addr{
        Addr{
            sender:self.addr.sender.clone()
        }
    }
}

const IDLE:usize = 0;
const READY:usize = 1;
const WAITING:usize = 2;
const CRITICAL:usize = 3;
const COMPLETED:usize = 4;

pub struct ActorTask{
    actor:Box<dyn Actor>,
    ctx:Context,
    state:AtomicUsize,
}
impl ActorTask{
    pub fn new<T:Actor+'static  >(actor:T,ctx:Context)->Self{
        let actor = Box::new(actor) as Box<dyn Actor>;
        let state = AtomicUsize::new(IDLE);
        Self{
            actor,
            ctx,
            state
        }
    }
}

impl Future for ActorTask {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        Poll::Ready(())
    }   
}