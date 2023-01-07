pub struct StrSplit<'a>{
    remainder:Option<&'a str>,
    delimiter:&'a str
}

impl <'a> StrSplit<'a>{
    pub fn new(str:&'a str,delimiter:&'a str) ->Self{
        Self{
            remainder:Some(str),
            delimiter
        }
    }
}

impl<'a> Iterator for StrSplit<'a>{
    type Item = &'a str;
    fn next(&mut self) ->Option<Self::Item>{
       if let Some(ref mut remainder) = self.remainder{
            if let Some(next_pos) = remainder.find(self.delimiter){
                let until = &remainder[..next_pos];
                *remainder = &remainder[(next_pos + self.delimiter.len())..];
                Some(until)
            }else{
                self.remainder.take()
            }
       }else{
            None
       }
    }
 }

#[test]
fn it_work(){
    let str = "a b c d e f g";
    let letters = StrSplit::new(str," ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e", "f", "g"].into_iter()));
}