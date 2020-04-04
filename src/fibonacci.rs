use crate::bits::Bits;
use crate::universal_coding::{UniversalCode, UniversalCodeIter};
use std::path::Path;

#[derive(Debug)]
pub struct Fibonacci {
    data: Bits,
    index: usize,
    fib: Vec<u64>
}

impl Fibonacci {
    pub fn read_from_file<X>(path: X) -> Result<Self, String> where X: AsRef<Path> {
        Ok(Self{
            data: Bits::read_from_file(path)?,
            index: 0,
            fib: vec![1,1]
        })
    }

    fn generate_fib_till(&mut self, code: u64){
        while self.fib[self.fib.len() - 1] < code {
            self.fib.push(self.fib[self.fib.len() - 1] + self.fib[self.fib.len() - 2]);
        }
    }

    fn generate_fib(&mut self, n: usize){
        while self.fib.len() <= n {
            self.fib.push(self.fib[self.fib.len() - 1] + self.fib[self.fib.len() - 2]);
        }
    }

    fn get_largest_smaller_fib(&mut self, code: u64) -> usize{
        self.generate_fib_till(code+1);
        let mut i = 1;
        loop{
            if code < self.fib[i] {
                return i-1;
            }
            i += 1;
        }
    }
}

impl UniversalCode for Fibonacci {
    //type UniIterator = UniversalCodeIter<Fibonacci>;

    fn new() -> Self{
        Self{
            data: Bits::new(),
            index: 0,
            fib: vec![1,1]
        }
    }

    fn get(&mut self) -> Option<u64> {
        let mut n = 0;
        let mut res = 0_u64;
        let mut prev = false;
        loop{
            match self.data.get(self.index) {
                None => return None,
                Some(x) =>{
                    self.index += 1;
                    n+=1;
                    if x {
                        if prev{
                            return Some(res - 1);
                        } else {
                            self.generate_fib(n);
                            res += self.fib[n];
                        }
                    }
                    prev = x;
                }
            }
        }
    }

    fn add(&mut self, mut code: u64) {
        code += 1;
        let mut buffer = vec![true, true];
        let mut n = self.get_largest_smaller_fib(code);
        code -= self.fib[n];
        while code > 0 {
            let m = self.get_largest_smaller_fib(code);
            for _ in m..(n-1){
                buffer.push(false);
            }
            buffer.push(true);
            n = m;
            code -= self.fib[n];
        }
        for _ in 1..n{
            buffer.push(false);
        }
        for bit in buffer.into_iter().rev() {
            self.data.push(bit);
        }
    }

    fn save_to_file<X>(&self, path: X) -> Result<(), String>
        where X: AsRef<Path>{
        self.data.save_to_file(path)
    }

    fn into_iter(self) -> Box<dyn Iterator<Item=u64>> {
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
mod fib_test {
    use crate::universal_coding::UniversalCode;

    #[test]
    fn fib_test() {
        let mut c = super::Fibonacci::new();
        c.add(6);
        assert_eq!(c.get(), Some(6));
        c.add(15);
        c.add(31);
        assert_eq!(c.get(), Some(15));
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