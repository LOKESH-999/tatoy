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
        self.lock();
        if self.head == 0{
            if self.len == 0{
                unsafe { self.ptr.write(data) };
            }else {
                self.head = self.size - 1;
            }
        }else {
            self.head -= 1;
            unsafe { self.ptr.add(self.head).write(data) };
        }
        self.len += 1;
        if self.size - self.len == 1{
            self.grow()
        }
        unsafe { self.unlock() };
    }

    pub fn lock(&self){
        while self.state.swap(LOCKED, AcqRel) == LOCKED {
            thread::yield_now();
        }
    }

    unsafe fn unlock(&self){
        self.state.store(UNLOCKED, Release);
    }

    fn grow(&mut self){

    }
}