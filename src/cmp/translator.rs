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
        let mut machine_code: Vec<u8> = Vec::new();
        //

        // for string semantic analyz
        /*println!("\nBefore:");
        for asm_str in self.asm_code.iter() {
            println!("{}", asm_str);
        }*/
        //self.tabs_into_spaces();
        /*match self.check_for_proc() {
            Ok(_) => (),
            Err(v) => return Err(v)
        }*/
        //self.delete_cpu();
        //self.carry_value();
        //self.add_zero();
        machine_code = self.turn_into_opcode();
        println!("\nAfter:");
        for asm_str in machine_code.iter() {
            println!("{:X}", asm_str);
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

    fn add_zero(&mut self) {
        let mut new_line_amount: u8 = 0;
        for index in 0..self.asm_code.len() {
            // each string
            if self.asm_code[index].chars().nth(0).unwrap() == ' ' {
                //only instruction lines
                let mut c = 0;
                for word in self.asm_code[index].split(" ") {
                    if c == 4 {
                        new_line_amount = self.length(word.clone().to_string());

                    }
                    c += 1;
                }
            } else {
                continue
            }
        }
    }

    fn length(&self, instr: String) -> u8 {
        if self.dictionary.get_opcode_length()[0].contains(&instr) {0}
        else if self.dictionary.get_opcode_length()[1].contains(&instr) {1}
        else {2}
    }

    fn turn_into_opcode(&mut self) -> Vec<u8> {
        let mut code: Vec<u8> = Vec::new();
        for index in 0..self.asm_code.len() {
            if ['0','1','2','3','4','5','6','7','8','9'].contains(&(self.asm_code[index].chars().nth(0).unwrap())) {
                code.push(self.number_decode(self.asm_code[index].clone()));
            } else {
                code.push(self.instr_decode(self.asm_code[index].clone()));
            }
        };

        code
    }

    fn instr_decode(&mut self, instr: String) -> u8 {
        match self.dictionary.get_opcode_set().get(&instr) {
            Some(opcode) => *opcode,
            None => 0xC0
        }
    }

    fn number_decode(&mut self, number: String) -> u8 {
        number.parse::<u8>().unwrap()
    }
}