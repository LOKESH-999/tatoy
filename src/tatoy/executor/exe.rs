use std::sync::mpsc;


type Recv<T> = mpsc::Receiver<T>;

pub struct Executor<T>{
    ready_q:Recv<T>
}