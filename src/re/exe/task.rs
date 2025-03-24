use std::{mem::MaybeUninit, pin::Pin, task::Waker};

use super::super::actors::actor::ActorTask;

pub struct Task{
    future:Pin<Box<dyn Future<Output = ()>>>,
    waker:MaybeUninit<Waker>
}