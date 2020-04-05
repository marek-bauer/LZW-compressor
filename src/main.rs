use lzw_with_universal_coder::elias_gamma::EliasGamma;
use lzw_with_universal_coder::universal_coding::{UniversalCode, Creatable};
use lzw_with_universal_coder::elias_delta::EliasDelta;
use lzw_with_universal_coder::elias_omega::EliasOmega;
use lzw_with_universal_coder::fibonacci::Fibonacci;
use lzw_with_universal_coder::dictionary::Dictionary;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use lzw_with_universal_coder::bits::entropy;


fn print_bar(p: u32) {
    print!("|");
    for _i in 0..p {
        print!("█");
    }
    for _i in p..100 {
        print!(" ");
    }
    println!("| {}%", p);
}

fn encode<X, Y>(data: X) -> Y
    where X: AsRef<[u8]>, Y: UniversalCode + Creatable {
    println!("Coding...");
    let mut prev = vec![];
    let mut res = Y::new();
    let data = data.as_ref();
    let mut dictionary = Dictionary::new();
    let mut percent = 0;
    let mut coded = 0;
    for code in data.iter().map(|a| *a) {
        if (coded * 100) / data.len() >= percent {
            print_bar(percent as u32);
            percent += 1;
        }
        prev.push(code);
        match dictionary.word_position(&prev) {
            None => {
                dictionary.add(prev.clone());
                prev.pop();
                res.add(dictionary.word_position(&prev).unwrap() as u64);
                prev = vec![code];
            }
            _ => {}
        }
        coded += 1;
    }
    match dictionary.word_position(&prev) {
        Some(code) => {
            res.add(code as u64);
            res
        }
        _ => res
    }
}


fn decode<X>(data: &mut X) -> Vec<u8>
    where X: UniversalCode + ?Sized {
    println!("Decoding...");
    let mut prev = vec![];
    let mut res = vec![];
    let mut dictionary = Dictionary::new();
    let mut percent = 0;
    loop {
        let code;
        match data.get() {
            None => return res,
            Some(c) => {
                code = c;
            }
        }
        if (data.index() as f64 * 100.0) / data.len() as f64 > percent as f64 {
            print_bar(percent);
            percent += 1;
        }
        if code >= dictionary.len() as u64 {
            let mut temp = prev.clone();
            temp.push(prev[0]);
            dictionary.add(temp);
        }
        res.append(&mut dictionary[code as usize].clone());
        if prev.len() != 0 {
            let mut temp = prev.clone();
            temp.push(dictionary[code as usize][0]);
            dictionary.add(temp);
        }
        prev = dictionary[code as usize].clone();
    }
}

fn compression_statistics<X: UniversalCode + ?Sized>(before: &Vec<u8>, after: &X) {
    println!("Size before {}B", before.len());
    println!("Size after {}B", after.len() / 8);
    println!("Compression ration {}%", (after.len() / 8) as f32 * 100.0 / before.len() as f32);
    println!("Entropy before {}", entropy(&before));
    println!("Entropy after {}", after.entropy());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    /*let code: char = 'f';
    let operation: char = 'd';
    let path_from: String = "code.code".parse().unwrap();
    let path_to: String = "new.bmp".parse().unwrap();*/
    let code: char;
    let operation: char;
    let path_from: String;
    let path_to: String;
    match args.len() {
        4 => {
            match args[1].as_str() {
                "--encode" => operation = 'e',
                "--decode" => operation = 'd',
                _ => {
                    println!("Wrong arguments please try {} <--encode | --decode> --type <gamma | delta | omega | fibonacci> <file_from> <file_to>", args[0]);
                    return;
                }
            }
            code = 'o';
            path_from = args[2].clone();
            path_to = args[3].clone();
        }
        6 => {
            match args[1].as_str() {
                "--encode" => operation = 'e',
                "--decode" => operation = 'd',
                _ => {
                    println!("Wrong arguments please try {} <--encode | --decode> --type <gamma | delta | omega | fibonacci> <file_from> <file_to>", args[0]);
                    return;
                }
            }
            match args[3].as_str() {
                "gamma" => code = 'g',
                "delta" => code = 'd',
                "omega" => code = 'o',
                "fibonacci" => code = 'f',
                _ => {
                    println!("Wrong arguments please try {} <--encode | --decode> --type <gamma | delta | omega | fibonacci> <file_from> <file_to>", args[0]);
                    return;
                }
            }
            path_from = args[4].clone();
            path_to = args[5].clone();
        }
        _ => {
            println!("Wrong arguments please try {} <--encode | --decode> --type <gamma | delta | omega | fibonacci> <file_from> <file_to>", args[0]);
            return;
        }
    }
    match operation {
        'e' => {
            let mut file;
            match File::open(path_from.clone()) {
                Ok(f) => file = f,
                Err(_error) => {
                    println!("Unable to open file {}", path_from.clone());
                    return;
                }
            }
            let mut data = vec![];
            match file.read_to_end(data.as_mut()) {
                Err(_e) => {
                    println!("Unable to read file {}", path_from);
                    return;
                }
                Ok(_) => {}
            }
            let coded_data: Box<dyn UniversalCode>;
            match code {
                'g' => coded_data = Box::new(encode::<&Vec<u8>, EliasGamma>(&data)),
                'd' => coded_data = Box::new(encode::<&Vec<u8>, EliasDelta>(&data)),
                'o' => coded_data = Box::new(encode::<&Vec<u8>, EliasOmega>(&data)),
                'f' => coded_data = Box::new(encode::<&Vec<u8>, Fibonacci>(&data)),
                _ => {
                    println!("Critical error unknown coding");
                    return;
                }
            }
            match coded_data.save_to_file(path_to) {
                Ok(()) => {
                    compression_statistics(&data, &*coded_data);
                }
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            }
        }
        'd' => {
            let mut coded_data: Box<dyn UniversalCode>;
            match code {
                'g' => {
                    match EliasGamma::read_from_file(path_from.clone()) {
                        Ok(r) => coded_data = Box::new(r),
                        Err(_) => {
                            println!("Unable to read file {}", path_from);
                            return;
                        }
                    }
                }
                'd' => {
                    match EliasDelta::read_from_file(path_from.clone()) {
                        Ok(r) => coded_data = Box::new(r),
                        Err(_) => {
                            println!("Unable to read file {}", path_from);
                            return;
                        }
                    }
                }
                'o' => {
                    match EliasOmega::read_from_file(path_from.clone()) {
                        Ok(r) => coded_data = Box::new(r),
                        Err(_) => {
                            println!("Unable to read file {}", path_from);
                            return;
                        }
                    }
                }
                'f' => {
                    match Fibonacci::read_from_file(path_from.clone()) {
                        Ok(r) => coded_data = Box::new(r),
                        Err(_) => {
                            println!("Unable to read file {}", path_from);
                            return;
                        }
                    }
                }
                _ => {
                    println!("Critical error unknown coding");
                    return;
                }
            }
            let data = decode(&mut *coded_data);
            let mut file;
            match File::create(path_to.clone()) {
                Ok(f) => file = f,
                Err(_error) => {
                    println!("Unable to create file {}", path_to.clone());
                    return;
                }
            }
            match file.write_all(data.as_ref()) {
                Err(_e) => {
                    println!("Unable to write file {}", path_to.clone());
                    return;
                }
                Ok(_) => {}
            }
            match file.sync_all() {
                Err(_e) => {
                    println!("Unable to write file {}", path_to);
                    return;
                }
                Ok(_) => {}
            }
        }
        _ => {}//panic("Wrong operation")
    }
}