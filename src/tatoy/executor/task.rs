use super::super::actor_sync::actor::{Actor,Addr,Context};
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicUsize;
use std::pin::Pin;
use std::task::Waker;
const IDLE:usize = 0;
const READY:usize = 1;
const WAITING:usize = 2;
const CRITICAL:usize = 3;
const COMPLETED:usize = 4;

pub struct ActorTask<A:Actor>{
    future:Pin<Box<dyn Future<Output = ()>>>,
    actor:A,
    ctx:Context<A>,
    addr:Addr<A>,
    state:AtomicUsize,
    waker:MaybeUninit<Waker>
}

