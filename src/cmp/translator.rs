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
        let mut asm_code = self.read_file();
        let machine_code: Vec<u8> = Vec::new();
        //

        // for string semantic analyz
        for i in 0..asm_code.len() {
            self.delete_comments(&mut asm_code[i]);
        }
        for asm_str in asm_code.iter() {
            println!("{}", asm_str);
        }



        machine_code
    }

    fn delete_comments(&self, asm_str: &mut String) {
        let mut i = 0;
        let str_a: String = (*asm_str).clone().to_string();
        for s in str_a.split(";") {
            if i == 0 {
                *asm_str = s.to_string();
            }
            i += 1; 
        };
    } 
}