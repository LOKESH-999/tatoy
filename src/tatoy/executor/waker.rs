use core::task::{
    Poll,
    Context,
    Waker,
    RawWaker,
    RawWakerVTable
};

use crate::tatoy::actor_sync::actor::Actor;
use crate::tatoy::executor::task::ActorTask;
use std::sync::Arc;
type Input<A:Actor> = Arc<ActorTask<A>>;


unsafe fn clone(data:*const ())->RawWaker{
todo!()
}

unsafe fn wake(data:*const ()){

}

unsafe fn wake_by_ref(data:*const ()){

}

unsafe fn drop(data:*const ()){

}

static VTABLE:RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

