use std::{fmt::Debug, ptr::null_mut, sync::{atomic::{AtomicBool, AtomicPtr}, Arc}};
use std::sync::atomic::Ordering::{
    Acquire,
    Release,
    AcqRel
};
type NodeType<T> = *mut Node<T>;

struct Node<T>{
    state:AtomicBool,
    pub data:T,
    pub next:NodeType<T>,
    pub prev:NodeType<T>
}
type UbType<T> = AtomicPtr<Node<T>>;

pub struct UbQueue<T>{
    head:UbType<T>,
    tail:UbType<T>
}


impl<T> UbQueue<T>{
    pub fn new()->Self{
        UbQueue{
            head:AtomicPtr::new(null_mut()),
            tail:AtomicPtr::new(null_mut())
        }
    }

    pub fn push_front(&self,data:T){
        let new_head = Box::into_raw(
            Box::new(
                Node{
                    state:AtomicBool::new(false),
                    data,
                    next:null_mut(),
                    prev:null_mut()
                }
            )
        );
        
        loop {
            let old_head = self.head.load(Acquire);
            if !old_head.is_null(){
                unsafe {
                    (*new_head) .next = old_head;
                }
                loop {
                    unsafe {
                        if (*old_head).state.compare_exchange(false, true, AcqRel, Acquire).is_ok(){
                            break;
                        }
                    }
                }
            }
            if let Ok(_) = self.head.compare_exchange(old_head, new_head, AcqRel, Acquire) {
                if old_head.is_null(){
                    self.tail.store(new_head, Release);
                }
                else {
                    unsafe { 
                        (*old_head) .prev = new_head;
                    }
                }
                unsafe {(*old_head).state.store(false, Release);}
                break;
            }
            if !old_head.is_null(){
                unsafe {
                    (*old_head).state.store(false, Release);
                }
            }
        }
    }

    pub fn pop_back(&self)->Option<T>{
        todo!("UB-queue takes lot of cpu cycle to waste so instred of the we use stack for storing waiting actors")
    }
}


// unsafe { 
//     (*new_data) .next = old_head
// }
// if old_head.is_null(){
//     if self.head.compare_exchange_weak(old_head, new_data, AcqRel, Acquire).is_ok(){
//         self.tail.store(new_data, Release);
//         break;
//     }
//     continue;
// }
// if self.head.compare_exchange_weak(old_head, new_data, AcqRel, Acquire).is_ok(){
//     unsafe { 
//         (*old_head) .prev = new_data;
//     }
//     break;
// }