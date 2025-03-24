// #![allow(unused)]

use core::sync::atomic::{AtomicBool,Ordering::{Acquire,Release}};
use core::hint::spin_loop;
use core::cell::UnsafeCell;
use core::ops::{Deref,DerefMut};
use std::thread;
use std::sync::atomic::Ordering::AcqRel;

pub struct SpinLock<T>{
    data:UnsafeCell<T>,
    locked:AtomicBool,
    #[cfg(panic = "unwind")]
    flag:Flag
}

#[cfg(panic = "unwind")]
struct Flag{
    is_panic:AtomicBool
}

pub struct SpinGuard<'a,T>{
    guard:&'a SpinLock<T>
}
impl<'a,T> Drop for SpinGuard<'a,T>{
    fn drop(&mut self){
        #[cfg(panic = "unwind")]
        {
            if  thread::panicking() {
                self.guard.flag.is_panic.store(true, Release);
            }
        }
        self.guard.locked.store(false, Release)
    }
}
impl<T> SpinLock<T>{
    
    pub const fn new(data:T)->Self{
        SpinLock { 
            data: UnsafeCell::new(data),
            locked: AtomicBool::new(false) ,
            #[cfg(panic = "unwind")]
            flag:Flag{ is_panic: AtomicBool::new(false)}
        }
    }

    #[cfg(panic = "unwind")]
    pub unsafe fn force_lock(&self) -> SpinGuard<'_, T> {
        // Forcibly take the lock and skip poison checks.
        while self.locked.swap(true, AcqRel) {
            spin_loop();
        }
        SpinGuard { guard: &self }
    }

    #[cfg(panic = "unwind")]
    pub fn is_poisoned(&self) -> bool {
        self.flag.is_panic.load(Acquire)
    }

    #[cfg(panic = "unwind")]
    pub unsafe fn reset(&self){
        self.flag.is_panic.store(false, Release);
    }

    pub unsafe fn unlock(&self){
        self.locked.store(false, Release);
    }

    pub fn try_lock(&self)->Option<SpinGuard<'_,T>>{
        #[cfg(panic = "unwind")]
        if self.flag.is_panic.load(Acquire){
            return None;
        }
        if self.locked.swap(true, AcqRel){
            return None;
        }
        Some(SpinGuard { 
            guard:&self
        })
    }

    pub fn lock(&self)->Option<SpinGuard<'_,T>>{
        #[cfg(panic = "unwind")]
        if self.flag.is_panic.load(Acquire){
            return None;
        }
        while self.locked.swap(true,AcqRel) { 
            #[cfg(not(panic = "unwind"))]
            spin_loop();
            #[cfg(panic = "unwind")]
            if self.flag.is_panic.load(Acquire){
                return None;
        }
        }
        Some(SpinGuard { 
                    guard:&self
                })
    }
}
impl<T> Deref for SpinGuard<'_,T>{
    type Target=T;
    fn deref(&self)->&T{
        unsafe {
            & *(self.guard.data.get() )   
        }
    }
}

impl<T> DerefMut for SpinGuard<'_,T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{
            &mut *self.guard.data.get()
        }
    }
}

#[cfg(panic = "unwind")]
impl<T: Default> SpinLock<T> {
    pub unsafe fn recover(&self) -> bool {
        if self.is_poisoned() {
            unsafe {
                self.reset();
                let mut guard = self.force_lock();
                *guard = T::default();
                self.reset();
                true
            }
        } else {
            false
        }
    }
}

unsafe impl<T> Send for SpinLock<T>{}
unsafe impl<T> Sync for SpinLock<T>{}
