pub fn flatten<I>(iter:I) -> Flatten<I>{
    Flatten::new(iter)
}

pub struct Flatten<I>
where 
    I:Iterator
{
    outer:I,
    iner:Option<Iterator::Item>
}

impl<I> Flatten<I> {
    pub fn new(i:I) -> Self{
        Flatten { outer: i }
    }
}

impl<I> Iterator for Flatten<I>
where 
    I: Iterator,
    I::Item : IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item; 
    fn next(&mut self) -> Option<Self::Item> {
        self.outer.next().and_then(|inner| inner.into_iter().next())
    }
}