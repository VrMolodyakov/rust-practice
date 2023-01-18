fn bar<T> (){}


fn call_fn(f:fn()){
    println!("{}",std::mem::size_of_val(&f))
}

fn closure<F>(f:F)
where 
    F:Fn(),
{

}
/* 
impl <F> FnOnce() for F
where F:FnMut(),
{
    fn call(mut self){
        FnMut::call(&mut self)
    }
}

impl <F> FnMut() for F
where F:Fn(),
{
    fn call(mut self){
        Fn::call(&*self);
    }
}
*/

fn take_by_ref(i:&i32){
    println!("{}",i);
}

#[derive(Debug)]
struct foo{
    s:i32,
    n:String
}

fn take_struct_by_ref(f:&foo){
    println!("{:?}",f);
}

#[derive(Debug)]
struct RefHolder<'a> {
    x: &'a i64,
}

impl<'a> RefHolder<'a> {
    fn new(x: &'a i64) -> RefHolder<'a> {
        RefHolder { x }
    }
}


#[test]
fn it_works(){
    //function item 
    let x = bar::<i32>;
    //function pointer 
    call_fn(x);
    //implements Fn,FnOnce,FnMut
    closure(x);

}


#[test]
fn test_mut_by_read_ref(){
    // Create `x`
    let mut x = 10;

    // Make sure `y` is `&mut i64`.
    let y = &mut x;

    // Package the downgraded reference into a struct.
    let z = RefHolder::new(y);
    println!("{:?}",z);
    *y +=1;
    
    println!("{}",y);

}