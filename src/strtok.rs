//25.34
//1.09.00
pub fn strtok<'a,'b> (s: &'a mut &'b str,delimiter:char) -> &'b str{
    if let Some(i) = s.find(delimiter){
        let prefix = &s[..i];
        let suffix = &s[(i+delimiter.len_utf8())..];
        *s = suffix;
        return prefix;
    }else{
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[test]
fn it_works(){
    let mut s = "hello world";
    let hello = strtok(&mut s, ' ');
    assert_eq!(s,"world"); // overlaping immutable borrow and mutable borrow in strtok
}