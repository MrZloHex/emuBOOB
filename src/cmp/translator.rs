use std::fs::File;
use std::io::{BufRead, BufReader};

use super::dictionary;

pub struct Compile {
    filename: String,
    dictionary: dictionary::Dictionary,
    asm_code: Vec<String>
}

impl Compile {
    pub fn new (filename: String) -> Compile {
        Compile{
            filename: filename.clone(),
            dictionary: dictionary::Dictionary::new(),
            asm_code: Compile::read_file(filename)
        }
    }

    fn read_file(filename: String) -> Vec<String> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut data: Vec<String> = Vec::new();
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            data.push(line);
        };
        data
    }

    pub fn compile(&mut self) -> Vec<u8> {
        let machine_code: Vec<u8> = Vec::new();
        //

        // for string semantic analyz
        for asm_str in self.asm_code.iter() {
            println!("{}", asm_str);
        }
        self.delete_comments();
        for asm_str in self.asm_code.iter() {
            println!("{}", asm_str);
        }
        self.delete_empty_str();
        for asm_str in self.asm_code.iter() {
            println!("{}", asm_str);
        }



        machine_code
    }

    fn delete_comments(&mut self) {
        for index in 0..self.asm_code.len() {
            let mut i = 0;
            let str_a: String = self.asm_code[index].clone().to_string();
            for s in str_a.split(";") {
                if i == 0 {
                    self.asm_code[index] = s.to_string();
                }   
                i += 1; 
            };
        }
    }

    fn delete_empty_str(&mut self) {
        let mut empty_str: Vec<usize> = Vec::new();
        // detect empty strings
        for index in 0..self.asm_code.len() {
            let mut ch_am = 0;
            for ch in self.asm_code[index].chars() {
                if !([' ', '\t', '\n'].contains(&ch)) {
                    ch_am += 1;
                }
            }
            if ch_am == 0 {
                empty_str.push(index.clone());
            }
        }
        // delete empty strings
        for x in &empty_str {
            self.asm_code.swap_remove(*x);
        }
    } 
}