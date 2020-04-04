use crate::bits::Bits;
use crate::universal_coding::{UniversalCode, UniversalCodeIter};
use std::path::Path;

#[derive(Debug)]
pub struct EliasGamma {
    data: Bits,
    index: usize,
}

impl EliasGamma {
    const LAST_U64:u64 = 0x8000000000000000;

    fn number_size(a: u64) -> u32 {
        let mut t = Self::LAST_U64;
        for i in (1_u32..=64).rev() {
            if a & t != 0 {
                return i;
            }
            t >>= 1;
        }
        0
    }

    pub fn read_from_file<X>(path: X) -> Result<Self, String> where X: AsRef<Path> {
        Ok(Self{
            data: Bits::read_from_file(path)?,
            index: 0
        })
    }
}

impl UniversalCode for EliasGamma {
    //type UniIterator = UniversalCodeIter<EliasGamma>;

    fn new() -> Self{
        Self{
            data: Bits::new(),
            index: 0
        }
    }

    fn get(&mut self) -> Option<u64> {
        let mut t = 1_u64;
        let mut res = 0;
        loop{
            match self.data.get(self.index) {
                Some(true) => break,
                Some(false) =>{
                    self.index += 1;
                    t <<= 1;
                }
                None => return None,
            }
        }
        t <<= 1;
        while t > 1 {
            t >>= 1;
            if self.data.get(self.index).unwrap(){
                res += t;
            }
            self.index += 1;
        }
        Some(res - 1)
    }

    fn add(&mut self, mut code: u64) {
        code += 1;
        let n = Self::number_size(code);
        for _ in 1..n {
            self.data.push(false);
        }
        let mut t = 1_u64 << n as u64 - 1;
        while t > 0 {
            if code & t == 0 {
                self.data.push(false);
            } else {
                self.data.push(true);
            }
            t >>= 1;
        }
    }

    fn save_to_file<X>(&self, path: X) -> Result<(), String>
        where X: AsRef<Path>{
        self.data.save_to_file(path)
    }

    fn into_iter(self) -> Box<dyn Iterator<Item=u64>>{
        Box::new(UniversalCodeIter{
            c: self
        })
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn index(&self) -> usize {
        self.index
    }

    fn entropy(&self) -> f64 {
        self.data.entropy()
    }
}

#[cfg(test)]
mod gamma_test {
    use crate::universal_coding::UniversalCode;

    #[test]
    fn gamma_test() {
        let mut c = super::EliasGamma::new();
        c.add(7);
        assert_eq!(c.get(), Some(7));
        c.add(15);
        c.add(31);
        assert_eq!(c.get(), Some(15));
        assert_eq!(c.get(), Some(31));
        assert_eq!(c.get(), None);
        c.add(1323123213123);
        c.add(3312312345324423);
        assert_eq!(c.get(), Some(1323123213123));
        assert_eq!(c.get(), Some(3312312345324423));
    }
}