use std::{cell::UnsafeCell, sync::atomic::{AtomicBool, Ordering}};

const LOCKED:bool = true;
const UNLOCKED:bool = false;


pub struct Mutex<T>{
    locked: AtomicBool,
    v:UnsafeCell<T>
}

impl<T> Mutex<T>{

    pub fn new (t:T) ->Self{
        Self{
            locked:AtomicBool::new(UNLOCKED),
            v:UnsafeCell::new(t)
        }
    }

    fn with_lock<R>(&self,f:impl FnOnce(&mut T) -> R) ->R{
        while self.locked.compare_exchange(UNLOCKED, 
            LOCKED,
             Ordering::Acquire,
             Ordering::Relaxed).is_err(){
                // MESI protocol stay in S when locked
                while self.locked.load(Ordering::Relaxed) == LOCKED{
                    std::thread::yield_now();
                }
                std::thread::yield_now();
             }
        while self.locked.load(Ordering::Relaxed) != UNLOCKED{}
        self.locked.store(LOCKED,Ordering::Release);
        let ret = f(unsafe{&mut *self.v.get()});
        self.locked.store(UNLOCKED,Ordering::Relaxed);
        ret
    }
}