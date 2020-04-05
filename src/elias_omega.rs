use crate::bits::Bits;
use crate::universal_coding::{UniversalCode, UniversalCodeIter, Creatable};
use std::path::Path;

#[derive(Debug)]
pub struct EliasOmega {
    data: Bits,
    index: usize,
}

impl EliasOmega {
    const LAST_U64: u64 = 0x8000000000000000;

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
        Ok(Self {
            data: Bits::read_from_file(path)?,
            index: 0,
        })
    }
}

impl Creatable for EliasOmega {
    fn new() -> Self {
        Self {
            data: Bits::new(),
            index: 0,
        }
    }
}

impl UniversalCode for EliasOmega {
    fn get(&mut self) -> Option<u64> {
        let mut n = 1;
        let mut t;
        loop {
            match self.data.get(self.index) {
                None => return None,
                Some(false) => {
                    self.index += 1;
                    break;
                }
                Some(true) => {
                    t = 1 << n;
                    self.index += 1;
                    n = t;
                    t >>= 1;
                }
            }
            while t > 0 {
                if self.data.get(self.index).unwrap() {
                    n += t;
                }
                self.index += 1;
                t >>= 1;
            }
        }
        Some(n - 1)
    }

    fn add(&mut self, mut code: u64) {
        code += 1;
        let mut buffer = vec![false];
        let mut k = code;
        while k > 1 {
            let size = Self::number_size(k) as u64;
            while k > 0 {
                if k % 2 == 1 {
                    buffer.push(true);
                } else {
                    buffer.push(false);
                }
                k >>= 1;
            }
            k = size - 1;
        }
        for bit in buffer.into_iter().rev() {
            self.data.push(bit);
        }
    }

    fn save_to_file(&self, path: String) -> Result<(), String>{
        self.data.save_to_file(path)
    }

    fn into_iter(self) -> Box<dyn Iterator<Item=u64>> {
        Box::new(UniversalCodeIter {
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
mod omega_test {
    use crate::universal_coding::{UniversalCode, Creatable};

    #[test]
    fn omega_test() {
        let mut c = super::EliasOmega::new();
        c.add(14);
        assert_eq!(c.get(), Some(14));
        c.add(31);
        assert_eq!(c.get(), Some(31));
        assert_eq!(c.get(), None);
        c.add(1323123213123);
        c.add(3312312345324423);
        assert_eq!(c.get(), Some(1323123213123));
        assert_eq!(c.get(), Some(3312312345324423));
        c.add(0);
        assert_eq!(c.get(), Some(0));
    }
}