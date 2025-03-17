use std::{alloc::{alloc, Layout}, ptr::{copy_nonoverlapping, NonNull}, sync::atomic::AtomicUsize,};
use std::sync::atomic::Ordering::{Acquire,Release,Relaxed};
use std::sync::atomic::AtomicPtr;
use std::alloc::dealloc;
pub struct Queue<T>{
    ptr:NonNull<T>,
    head:AtomicUsize,
    tail:AtomicUsize,
    len:AtomicUsize,
    size:AtomicUsize
}

impl<T> Queue<T>{
    pub fn new()->Self{
        assert_ne!(std::mem::size_of::<T>(),0);
        let size = 2;
        let layout = Layout::array::<T>(size).unwrap();
        let ptr = unsafe {alloc(layout) as *mut T};
        Queue{
            ptr:NonNull::new(ptr).unwrap(),
            head:AtomicUsize::new(0),
            tail:AtomicUsize::new(0),
            len:AtomicUsize::new(0),
            size:AtomicUsize::new(size)
        }
    }
    pub fn push_front(&mut self,data:T){
        todo!("groww check")
    }

    pub fn push_back(&mut self,data:T){}

    pub fn pop_back(&mut self)->Option<T>{
        todo!()
    }
    pub fn pop_front(&mut self)->Option<T>{
        todo!()
    }
}

impl<T> Queue<T>{
    pub fn grow(&mut self){
        let old_ptr = self.ptr.as_ptr();
        let old_size = self.size.load(Acquire);

        let new_size = old_size * 2;
        let new_layout = Layout::array::<T>(new_size).unwrap();
        let new_ptr = unsafe { alloc(new_layout) as *mut T };
        assert_ne!(new_ptr,std::ptr::null_mut(),"allocation err");
        unsafe {
            copy_nonoverlapping(old_ptr, new_ptr , old_size);
        }
        let res = self.size.compare_exchange_weak(old_size, new_size, Release, Relaxed);
        match res {
            Ok(_)=>{
                self.ptr = NonNull::new(new_ptr).unwrap();
                let old_layout = Layout::array::<T>(old_size).unwrap();
                unsafe { dealloc(old_ptr as *mut u8, old_layout) };
            },
            Err(_)=>{
                unsafe { dealloc(new_ptr as *mut u8, new_layout) };
            }
        }
    }
}