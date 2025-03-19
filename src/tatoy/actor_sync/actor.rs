use std::sync::mpsc::{self, SendError};

pub trait Actor {
    type Msg;
    fn recieve(&mut self,msg:Self::Msg,cx:&mut Context<Self>);
}
pub struct Addr<A:Actor+ ?Sized>{
    sender:mpsc::Sender<A::Msg>
}

pub struct Context<A:Actor+ ?Sized>{
    data:Addr<A>
}

impl<A:Actor+?Sized> Addr<A>{
    pub fn fire(&mut self,data:A::Msg){
        self.sender.send(data);
    }
    pub fn sync_Send(&mut self,data:A::Msg)->Result<(),SendError<A::Msg>>{
        self.sender.send(data)
        // todo!()
    }
}