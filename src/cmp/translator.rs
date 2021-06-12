use std::fs::File;
use std::io::{BufRead, BufReader};

use super::dictionary;

pub struct Compile {
    dictionary: dictionary::Dictionary,
    asm_code: Vec<String>
}

impl Compile {
    pub fn new (filename: String) -> Compile {
        Compile{
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
        let mut _machine_code: Vec<u8> = Vec::new();
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
        //self.delete_cpu();n load_instr(&m
        //self.carry_value();
        self.add_zero();
        self.transform_labels();
        self.decompose_labels();
        _machine_code = self.turn_into_opcode();
        println!("\nAfter:");
        let mut c = 0;
        for asm_str in self.asm_code.iter() {
            println!("{}\t{}", c, asm_str);
            c += 1;
        }



        Ok(_machine_code)
    } 

    fn _tabs_into_spaces(&mut self) {
        for index in 0..self.asm_code.len() {
            self.asm_code[index] = self.asm_code[index].replace("\t", "    ");
        }
    }

    fn _check_for_proc(&mut self) -> Result<(), u8> {
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
    fn _delete_cpu(&mut self) {
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
        let mut line_lab: isize = -1;
        let mut ampresand: bool = true;
        while ampresand {
            for index in 0..self.asm_code.len() {
                if self.asm_code[index].chars().nth(0) == Some('&') {
                    line_lab = index.clone() as isize;
                    self.asm_code[index] = self.asm_code[index].replace("&", "%");
                    break
                }
            }
            let mut new_code: Vec<String> = Vec::new();
            if line_lab != -1 {
                for line in 0..self.asm_code.len() {
                    if (line_lab + 1) as usize == line {
                        new_code.push("0".to_string())
                    }
                    new_code.push(self.asm_code[line].clone())
                }
            }

            self.asm_code = new_code;

            // check for delete all ampersands
            for i in 0..self.asm_code.len() {
                if self.asm_code[i].chars().nth(0) == Some('&') {
                    ampresand = true;
                    break
                } else {
                    ampresand = false;
                }
            }
        }
    }

    fn transform_labels(&mut self) {
        let mut line_lab: isize = -1;
        let mut label: String = String::new();
        let mut colon: bool = true;
        while colon {
            for index in 0..self.asm_code.len() {
                if self.asm_code[index].chars().last() == Some(':') {
                    line_lab = index.clone() as isize;
                    label = self.asm_code[index].clone();
                    label.pop();
                    self.asm_code.remove(index);
                    break
                }
            }
            let mut new_code: Vec<String> = Vec::new();
            if line_lab != -1 {
                for line in 0..self.asm_code.len() {
                    let mut _new_str: String = String::new();
                    if line_lab as usize == line {
                        _new_str = format!("@{}@", label);
                    }
                    else {_new_str = "".to_string()}
                    new_code.push(format!("{}{}", _new_str, self.asm_code[line].clone()))
                }
            }

            self.asm_code = new_code;

            // check for delete all ampersands
            for i in 0..self.asm_code.len() {
                if self.asm_code[i].chars().last() == Some(':') {
                    colon = true;
                    break
                } else {
                    colon = false;
                }
            }
        }
    }

    fn decompose_labels(&mut self) {
        let mut label_line: isize = -1;
        let mut label: String = String::new();
        let mut label_f: bool = true;
        while label_f {
            for index in 0..self.asm_code.len() {
                if self.asm_code[index].chars().nth(0) == Some('@') {
                    let split = self.asm_code[index].split("@");
                    let mut c: u8 = 0;
                    for s in split {
                        if c == 1 {label = s.to_string()}
                        c += 1;
                    };
                    label_line = index.clone() as isize;
                    self.asm_code[index] = self.asm_code[index].replace(format!("@{}@", label).as_str(), "");
                    break
                }
            }
            
            if label_line != -1 {
                for line in 0..self.asm_code.len() {
                    if self.asm_code[line].chars().nth(0) == Some('%') {
                        let split = self.asm_code[line].split("%");
                        let mut c: u8 = 0;
                        let mut p_l: String = String::new();
                        for s in split {
                            if c == 1 {p_l = s.to_string()}
                            c += 1;
                        };
                        if p_l == label {
                            self.asm_code[line] = self.asm_code[line].replace(self.asm_code[line].as_str(), label_line.to_string().as_str());
                        }
                    }
                }
            }
            // check for delete all ampersands
            for i in 0..self.asm_code.len() {
                if self.asm_code[i].chars().nth(0) == Some('@') {
                    label_f = true;
                    break
                } else {
                    label_f = false;
                }
            }
        }
    }

    fn turn_into_opcode(&mut self) -> Vec<u8> {
        let mut code: Vec<u8> = Vec::new();
        for index in 0..self.asm_code.len() {
            if ('0'..='9').any(|num| Some(num) == self.asm_code[index].chars().nth(0)) {
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