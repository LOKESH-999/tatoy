mod tatoy;

fn main() {
    println!("Hello, world!");
    let x = null_mut();
    unsafe {
        *x = 90;
        println!("{:?}",*x);
    }
    
}
use std::ptr::null_mut;

use flume::unbounded;