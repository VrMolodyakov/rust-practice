pub trait Delimeter{
    fn next_delim(&self,s:&str) -> Option<(usize,usize)>;
}

#[derive(Debug)]
pub struct StrSplit<'r,D>{
    remainder:Option<&'r str>,
    delimiter:D,
}

impl <'r,D> StrSplit<'r,D>{
    pub fn new(str:&'r str,delimiter:D) ->Self{
        Self{
            remainder:Some(str),
            delimiter
        }
    }
}

impl<'r,D> Iterator for StrSplit<'r,D> 
where 
    D:Delimeter,
{
    type Item = &'r str;
    fn next(&mut self) ->Option<Self::Item>{
        let remainder = self.remainder.as_mut()?;
        if let Some((start,end)) = self.delimiter.next_delim(remainder){
            let until = &remainder[..start];
            *remainder = &remainder[end..];
            Some(until)
        }else{
            self.remainder.take()
        }
      
    }
 }

impl Delimeter for &str{
    fn next_delim(&self,s:&str) -> Option<(usize,usize)>{
        s.find(self).map(|start| (start,start + self.len()))
    }
}


impl Delimeter for char{
    fn next_delim(&self,s:&str) -> Option<(usize,usize)>{
        s.char_indices()
        .find(|(_,c)| c == self)
        .map( |(start,_)| (start,start + self.len_utf8()))
    }
}



fn until_char(str:&str,c:char) -> &'_ str{
    return StrSplit::new(str,c)
            .next()
            .expect("at least something should be returned")
}

#[test]
fn until_char_test(){
    assert_eq!(until_char("goode", 'e'),"good");
}

#[test]
fn it_work(){
    let str = "a b c d e f g";
    let letters = StrSplit::new(str," ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e", "f", "g"].into_iter()));
}