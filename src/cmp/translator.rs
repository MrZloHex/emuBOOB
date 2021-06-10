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

        




        machine_code
    }
}