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
        /*println!("\nBefore:");
        for asm_str in self.asm_code.iter() {
            println!("{}", asm_str);
        }*/
        self.tabs_into_spaces();
        match self.check_for_proc() {
            Ok(_) => (),
            Err(v) => return Err(v)
        }
        self.delete_cpu();
        //self.carry_value();
        println!("\nAfter:");
        for asm_str in self.asm_code.iter() {
            println!("{}", asm_str);
        }



        Ok(machine_code)
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
    /*
    fn carry_value(&mut self) {
        let mut new_code: Vec<String> = Vec::new();
        let mut i = 0;
        for index in 0..self.asm_code.len() {
            if self.asm_code[index].chars().nth(0).unwrap() == ' ' {
                
                let mut new_str = String::new();
                let mut c = 0;
                for word in self.asm_code[index].split(" ") {
                    if c != 5 {
                        new_code.push(self.asm_code[index].clone());
                    }
                    else if c == 5 {
                        new_str = format!("    {}", word).to_string();
                        i += 1;
                        new_code.push(new_str);
                    }
                    c += 1;
                }
            }
            else {
                continue
            }
            i += 1;
        }
        for x in &new_code {
            println!("{}", x)
        }
    }
    */
}