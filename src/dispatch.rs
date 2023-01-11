// Object safety examples

pub trait Hello{
    fn say(&self);
    fn say_sized (&self) where Self:Sized;
    fn say_without_self() where Self: Sized;

}

impl Hello for &str{

    fn say(&self) {
        println!("hello, {}",self);
    }

    fn say_sized (&self){
        println!("inside sized self")
    }

    fn say_without_self() {
        println!("without self ")
    }
}

pub fn strlen(s : impl AsRef<str>) ->usize{
    return s.as_ref().len()
}

// virtual dispatch table
pub fn box_strlen(s:Box<dyn AsRef<str>>) ->usize{
    return s.as_ref().as_ref().len()
}

pub fn can_say(s:&dyn Hello){
    s.say();
}


pub fn can_size_say(s: &str){
    s.say();
    s.say_sized();
}

pub fn can_without_self_say<T:Hello>(){
    T::say_without_self();
}

//dinamicly sized type 
struct Exml{
    f:bool,
    x:i32,
    t:[i32],
}

//&Exml(*Exml,length l) 

#[test]
fn it_work(){
    assert_eq!(strlen("hello"),5);
    assert_eq!(box_strlen(Box::new("hello")),5);
}

#[test]
fn trait_work(){
    can_size_say(&"mars");
    can_without_self_say::<&str>();
}

