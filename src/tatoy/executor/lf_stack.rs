use core::sync::atomic::{
    AtomicPtr,
    Ordering::{
        AcqRel,
        Acquire,
        Release,
        Relaxed
    }
};
use std::ptr::null_mut;


type NodeType<T> = *mut Node<T>;

struct Node<T>{
    data:T,
    next:NodeType<T>
}
type UbType<T> = AtomicPtr<Node<T>>;

pub struct UbStack<T>{
    head:UbType<T>
}

impl<T> UbStack<T>{
    pub fn new()->Self{
        Self{
            head:AtomicPtr::new(null_mut())
        }
    }
    pub fn push(&self,data:T){
        let new_head = Box::into_raw(
            Box::new(
                Node{
                    data,
                    next:null_mut()
                }
            )
        );
        loop {
            let old_head = self.head.load(Acquire);
            unsafe {(*new_head).next = old_head };
            if let Ok(_) = self.head.compare_exchange(old_head, new_head, AcqRel, Relaxed){
                return;
            }
        }
    }

    pub fn pop(&self)->Option<T>{
        loop {
            let old_head = self.head.load(Acquire);
            if old_head.is_null(){
                return None;
            }
            let new = unsafe {(*old_head).next};
            if let Ok(_) = self.head.compare_exchange(old_head, new, AcqRel, Relaxed){
                return unsafe { Some(Box::from_raw(old_head).data) };
            }
        }
    }
}