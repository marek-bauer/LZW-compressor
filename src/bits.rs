use std::option::Option::Some;
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;

#[derive(Debug)]
pub struct Bits{
    data: Vec<u8>,
    size: usize
}

impl Bits{
    const BIN: [u8; 8] = [128, 64, 32, 16, 8, 4, 2, 1];

    pub fn new() -> Self{
        Self{
            data: Vec::new(),
            size: 0
        }
    }

    pub fn len(&self) -> usize{
        self.size
    }

    pub fn push(&mut self, c: bool) {
        let sector = self.size % 8;
        if sector == 0 {
            self.data.push(0);
        }
        if c {
            let block = self.size / 8;
            self.data[block] |= Self::BIN[sector]
        }
        self.size += 1;
    }

    /**
        Read one bit of your code
    */
    pub fn get(&self, i: usize) -> Option<bool> {
        if i < self.size{
            return Some(!self.data[i / 8] & Self::BIN[i % 8] == 0);
        }
        None
    }

    pub fn save_to_file<X>(&self, path: X) -> Result<(), String> where X: AsRef<Path> {
        let mut file;
        match File::create(path){
            Ok(f) => file = f,
            Err(_e) => return Err("Unable to open file".parse().unwrap())
        }
        //Save data
        match file.write(self.data.as_ref()){
            Err(_e) => return Err("Unable to save file".parse().unwrap()),
            _ => {}
        }
        //Save number of encoded characters
        match file.sync_all(){
            Err(_e) => return Err("Unable to save file".parse().unwrap()),
            _ => {}
        }
        Ok(())
    }

    pub fn read_from_file<X>(path: X) -> Result<Self, String> where X: AsRef<Path> {
        let mut file;
        match File::open(path){
            Ok(f) => file = f,
            Err(_e) => return Err("Unable to open file".parse().unwrap())
        }
        let mut data = vec![];
        match file.read_to_end(data.as_mut()) {
            Err(_e) => return Err("Unable to read file".parse().unwrap()),
            _ => {}
        }
        Ok(Self {
            size: (data.len() * 8),
            data,
        })
    }

    pub fn entropy(&self) -> f64{
        entropy(&self.data)
    }
}

pub fn entropy(data: &Vec<u8>) -> f64{
    let sum = data.iter().fold(0_u64, |a, b| a+*b as u64);
    let mut temp = [0_u64; 256];
    for d in data{
        temp[*d as usize] += 1;
    }
    temp.iter().fold(0.0, |acc, x| if *x > 0{
        acc - (*x as f64/ sum as f64) * ((*x) as f64 / sum as f64).log2()
    }  else{
        acc
    })
}