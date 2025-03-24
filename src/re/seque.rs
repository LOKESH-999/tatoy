use std::{
    mem:: ManuallyDrop,
    ptr::null_mut,
    sync::atomic::{
        AtomicPtr,
        Ordering::{
            AcqRel,
            Acquire,
            Release
        }
    }
};


struct Node<T> {
    data: ManuallyDrop<T>,
    ptr: AtomicPtr<Node<T>>
}


impl<T> Node<T> {
    pub fn new(data:T) -> Box<Node<T>>{
        let node = Box::new(
            Node{
                data:ManuallyDrop::new(data),
                ptr:AtomicPtr::new(null_mut())
            }
        );
        node
    }
    pub fn into_raw(self:Box<Self>)->* mut Node<T>{
        Box::into_raw(self)
    }
}

pub struct SeQue<T>{
    head: AtomicPtr<Node<T>>,
    tail:AtomicPtr<Node<T>>
}


impl<T:Default> SeQue<T>{
    pub fn new()->Self{
        let dummy = Node::new(T::default()).into_raw();
        Self{
            head:AtomicPtr::new(dummy),
            tail:AtomicPtr::new(dummy)
        }
    }
    pub fn enqueue(&self,data:T){
        let new = Node::new(data).into_raw();
        loop {
            unsafe { 
                let ptr = &(*self.tail.load(Acquire)).ptr;
                if let Ok(_) = ptr.compare_exchange(null_mut(), new, AcqRel, Acquire) {
                    self.tail.store(new, Release);
                    break;
                }
            }
        }
    }
    pub fn dequeue(&self)->Option<T>{
        loop {
            let head_ptr = self.head.load(Acquire);
            let head_next_ptr = unsafe { (*head_ptr).ptr.load(Acquire) };
            if head_next_ptr.is_null(){
                return None;
            }
            if let Ok(_) = self.head.compare_exchange(head_ptr, head_next_ptr, AcqRel, Acquire) {
                let res = Some(unsafe { 
                    ManuallyDrop::take(&mut (*head_next_ptr).data)
                });
                _ = unsafe { Box::from_raw(head_ptr) };
                
                return res;
            }
        }
    }
}