use std::{future::Future,pin::Pin,task::{Context,Poll}};

struct Ready<T>{
    value:Option<T>
}


struct MyFuture<F>{
    f:Pin<Box<F>>,
}

impl<F> MyFuture<F>{
    fn get_t(&mut self){
        let x = self.f.as_mut();
    }
}

impl <F:Future + Unpin>Future for MyFuture<F> {
    type Output = F::Output;
    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(& mut self.f).poll(ctx)
    }
}
