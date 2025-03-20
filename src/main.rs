mod tatoy;

struct ABC{
    str:String
}

use tatoy::actor_sync::actor::*;
impl Actor for ABC{
    type Msg = i32;
    fn recieve(&mut self,msg:Self::Msg,cx:&mut Context<Self>) {
        println!("{}",msg)
    }
}
fn main() {
    println!("Hello, world!");
}
