use std::{marker::PhantomData, sync::mpsc::{self, SendError}};

pub trait Actor {
    fn recieve(&mut self,msg:Box<dyn Message>,cx:&mut Context<Self>);
}

pub trait Message {
    fn to_bytes(&self)->Option<&[u8]>{
        None
    }
}
pub struct Addr<A:Actor+ ?Sized>{
    sender:mpsc::Sender<Box<dyn Message>>,
    _p:PhantomData<A>
}

pub struct Context<A:Actor+ ?Sized>{
    data:Addr<A>,
    _p:PhantomData<A>
}

impl<A:Actor+?Sized> Addr<A>{
    pub fn fire(&mut self,data:Box<dyn Message>){
        self.sender.send(data);
    }
    pub fn sync_Send(&mut self,data:Box<dyn Message>)->Result<(),SendError<Box<dyn Message>>>{
        self.sender.send(data)
        // todo!()
    }
}

impl<A> Clone for Addr<A>
where A:Actor + ?Sized{
    fn clone(&self) -> Self {
        Addr{
            sender:self.sender.clone(),
            _p:PhantomData
        }
    }
}