pub fn flatten<I>(iter:I) -> Flatten<I::IntoIter>
where 
    I:IntoIterator,
    I::Item : IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<I>
where 
    I:Iterator,
    I::Item : IntoIterator
{
    outer:I,
    front_inner:Option<<I::Item as IntoIterator>::IntoIter>,
    back_inner:Option<<I::Item as IntoIterator>::IntoIter>

}

impl<I> Flatten<I> 
where
    I:Iterator,  
    I::Item : IntoIterator
{
    pub fn new(i:I) -> Self{
        Flatten { outer: i,front_inner:None,back_inner:None }
    }
}

impl<I> Iterator for Flatten<I>
where 
    I: Iterator,
    I::Item : IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item; 
    fn next(&mut self) -> Option<Self::Item> {
        loop{
            if let Some(ref mut front_iter) = self.front_inner{
                if let Some(item) = front_iter.next(){
                    return Some(item)
                }
                self.front_inner = None
            }
            if let Some(next_front_iter) = self.outer.next(){
                self.front_inner = Some(next_front_iter.into_iter());

            }else{
                return self.back_inner.as_mut()?.next();
            }
        }
        
    }
}

impl <I> DoubleEndedIterator for Flatten<I>
where 
I: DoubleEndedIterator,
I::Item : IntoIterator, 
<I::Item as IntoIterator>::IntoIter : DoubleEndedIterator
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop{
            if let Some(ref mut back_iter) = self.back_inner{
                if let Some(item) = back_iter.next_back(){
                    return Some(item)
                }
                self.front_inner = None
            }
            if let Some(next_back_iter) = self.outer.next_back(){
                self.front_inner = Some(next_back_iter.into_iter());

            }else{
                return self.front_inner.as_mut()?.next();
            }
        }
    }
}


// optional and move() (clone trait)
#[test]
fn one(){
    assert_eq!(flatten(std::iter::once(vec!["a"])).count(),1);
}

#[test]
fn two_wide() {
    assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
}