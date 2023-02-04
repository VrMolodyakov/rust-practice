use std::sync::{Arc,Mutex, Condvar};
use std::collections::VecDeque;

//29:04

pub struct Sender<T>{
    shared: Arc<Shared<T>>
}

impl <T> Sender<T> {
    pub fn send(&mut self,t:T){
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
    }
}

impl<T> Clone for Sender<T>{
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders +=1 ;
        drop(inner);
        Sender { 
            shared: Arc::clone(&self.shared) 
        }
    }
}

impl<T> Drop for Sender<T>{
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let last = inner.senders == 0;
        drop(inner);
        if last{
            self.shared.available.notify_one();
        }
        
    }
}

pub struct Receiver<T>{
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>
}


impl <T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T>{
        if let Some(remain) = self.buffer.pop_front(){
            return Some(remain);
        }
        let mut inner = self.shared.inner.lock().unwrap();
        loop{
            match inner.queue.pop_front() {
                Some(t) =>{
                    if !inner.queue.is_empty(){
                        std::mem::swap(&mut inner.queue, &mut self.buffer);
                        return Some(t)
                    }
                },
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

#[derive(Default)]
struct Inner<T>{
    queue : VecDeque<T>,
    senders:usize,
}


struct Shared<T>{
    inner : Mutex<Inner<T>>,
    available:Condvar,
}

fn channel<T>() ->(Sender<T>,Receiver<T>){
    let inner = Inner{
        queue:VecDeque::default(),
        senders:1,
    };
    let shared = Shared{
        inner: Mutex::new(inner),
        available:Condvar::new()
    };
    let inner = Arc::new(shared);
    (
        Sender{
            shared:inner.clone()
        },
        Receiver{
            shared:inner.clone(),
            buffer:VecDeque::default()
        }
    )
}

#[test]
fn ping_pong(){
    let (mut tx,mut rx) = channel();
    tx.send(42);
    assert_eq!(rx.recv(),Some(42));
}


 
