pub trait UniversalCode{
    fn get(&mut self) -> Option<u64>;
    fn add(&mut self, code: u64);
    fn save_to_file(&self, path: String) -> Result<(), String>;
    fn into_iter(self) -> Box<dyn Iterator<Item=u64>>;
    fn len(&self) -> usize;
    fn index(&self) -> usize;
    fn entropy(&self) -> f64;
}

pub trait Creatable{
    fn new() -> Self;
}


pub struct UniversalCodeIter<X: UniversalCode+ ?Sized>{
    pub c: X
}

impl<X: UniversalCode+ ?Sized> Iterator for UniversalCodeIter<X>{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.c.get()
    }
}