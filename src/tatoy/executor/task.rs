use super::super::actor_sync::actor::{Actor,Addr,Context};

pub struct ActorTask<A:Actor>{
    future:Box<dyn Future<Output = ()>>,
    actor:A,
    ctx:Context<A>,
    addr:Addr<A>
}

