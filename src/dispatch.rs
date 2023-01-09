

pub fn strlen(s : impl AsRef<str>) ->usize{
    return s.as_ref().len()
}

// virtual dispatch table
pub fn box_strlen(s:Box<dyn AsRef<str>>) ->usize{
    return s.as_ref().as_ref().len()
}

#[test]
fn it_work(){
    assert_eq!(strlen("hello"),5);
    assert_eq!(box_strlen(Box::new("hello")),5);
}