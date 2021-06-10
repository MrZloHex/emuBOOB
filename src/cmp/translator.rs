use std::fs::File;
use std::io::{BufRead, BufReader};

use super::dictionary;

pub struct Compile {
    filename: String,
    dictionary: dictionary::Dictionary
}

impl Compile {
    pub fn new (filename: String) -> Compile {
        Compile{
            filename: filename,
            dictionary: dictionary::Dictionary::new()
        }
    }

    fn read_file(&self) -> Vec<String> {
        let file = File::open(&self.filename).unwrap();
        let reader = BufReader::new(file);
        let mut data: Vec<String> = Vec::new();
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            data.push(line);
        };
        data
    }

    pub fn compile(&self) -> Vec<u8> {
        let asm_code = self.read_file();
        let machine_code: Vec<u8> = Vec::new();
        //let asm_str = String::new();

        // for string semantic analyz
        for asm_str in asm_code.iter() {
            self.delete_comments(&asm_str);
        }




        machine_code
    }

    fn delete_comments(&self, asm_str: &String) {
        for (i, ch) in asm_str.chars().enumerate() {
            let mut ch_amount: u16 = 0;
            if !([' ', '\t'].contains(&ch)) {
                print!("{}", ch);
                ch_amount += 1;
            }
            if ch == ';' {println!("{}", ch_amount)}
        }
    } 
}