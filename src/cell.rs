use std::cell::UnsafeCell;

//thread local
//allows us to mutate somethung throught shared reference
//fn foo(&self)

//implied by UnsafeCell
// impl<T> !Sync for Cell<T>

//deref get called when smth.MethodA() , smth<T> , Impl T( fn MethodA)
pub struct Cell<T>{
    value:UnsafeCell<T> 
}

impl<T> Cell<T>{
    pub fn new(value:T) -> Self{
        Cell{value:UnsafeCell::new(value)}
    }

    pub fn set(&self , value:T){
        unsafe{*self.value.get() = value;}
    }

    pub fn get(&self) -> T
    where  
        T:Copy
    {
        unsafe{*self.value.get()}
    }
}