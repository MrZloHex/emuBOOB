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

    pub fn compile(&mut self) -> Result<Vec<u8>, u8> {
        let machine_code: Vec<u8> = Vec::new();
        //

        // for string semantic analyz
        println!("\nBefore:");
        for asm_str in self.asm_code.iter() {
            println!("{}", asm_str);
        }
        self.delete_comments();
        self.delete_empty_str();
        self.tabs_into_spaces();
        match self.check_for_proc() {
            Ok(_) => (),
            Err(v) => return Err(v)
        }
        self.delete_cpu();
        self.carry_value();
        println!("\nAfter:");
        for asm_str in self.asm_code.iter() {
            println!("{}", asm_str);
        }



        Ok(machine_code)
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
        let mut new_code: Vec<String> = Vec::new();
        for index in 0..self.asm_code.len() {
            if empty_str.contains(&index) {continue}
            else {
                new_code.push(self.asm_code[index].clone());
            }
        }
        // load new data
        for index in 0..new_code.len() {
            self.asm_code[index] = new_code[index].clone();
        }
    } 

    fn tabs_into_spaces(&mut self) {
        for index in 0..self.asm_code.len() {
            self.asm_code[index] = self.asm_code[index].replace("\t", "    ");
        }
    }

    fn check_for_proc(&mut self) -> Result<(), u8> {
        let first_str: String = self.asm_code[0].clone();
        let mut i = 0;
        let mut cpu: bool = false;
        let mut proc: bool = false;
        for word in first_str.split(" ") {
            //println!("{}", word);
            if i == 4 {
                if word == "CPU".to_string() {cpu = true}
                else {cpu = false}
            }
            else if i == 5 {
                if word == "8008".to_string() {proc = true}
                else {proc = false}
            }
            i += 1;
        }
        if cpu && proc {
            return Ok(())
        } else {
            return Err(1)
        }
    }
    fn delete_cpu(&mut self) {
        let mut new_code: Vec<String> = Vec::new();
        for index in 1..self.asm_code.len() {
            new_code.push(self.asm_code[index].clone());
        }
        // load new data
        for index in 0..new_code.len() {
            self.asm_code[index] = new_code[index].clone();
        }
        self.asm_code.pop();
    }

    fn carry_value(&mut self) {
        for index in 0..self.asm_code.len() {
            if [' ', '\t'].contains(&(self.asm_code[index].chars().collect()[0])) {
                continue
            }
            else {
                println!("{}", self.asm_code[index]);
            }
        }
    }
}