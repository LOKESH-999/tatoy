use std::{alloc::{alloc, Layout}, ptr::NonNull, sync::atomic::AtomicUsize};


pub struct Queue<T:?Sized>{
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
}