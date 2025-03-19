use std::{alloc::{alloc, Layout}, ptr::NonNull, sync::atomic::AtomicU64, thread};

const UNLOCKED: u64 = 0;
const LOCKED: u64 = 1;
const CRITICAL: u64 = 2;
const GROW:u64 = 3;
use std::sync::atomic::Ordering::{
    AcqRel,
    Release
};
pub struct Queue<T>{
    state:AtomicU64,
    ptr:NonNull<T>,
    head:usize,
    tail:usize,
    len:usize,
    size:usize
}

impl <T> Queue<T> {
    pub fn new()->Self{
        let size = 11;
        let layout = Layout::array::<T>(size).unwrap();
        let ptr = NonNull::new(
            unsafe {
                alloc(layout) as *mut T
            }
        ).unwrap();
        Queue{
            state:AtomicU64::new(0),
            ptr,
            head:0,
            tail:0,
            size,
            len:0
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
    pub fn grow(&self){
        let old_ptr = self.ptr.load(Acquire);
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
                self.ptr.store(new_ptr,Release);
                let old_layout = Layout::array::<T>(old_size).unwrap();
                unsafe { dealloc(old_ptr as *mut u8, old_layout) };
            },
            Err(_)=>{
                unsafe { dealloc(new_ptr as *mut u8, new_layout) };
            }
        }
    }
}