use std::fs;
use std::fs::File;
use std::io::Read;

const MAX_PROM: usize = 2048; // orig 16384 * 8
const MAX_DATA: usize = 1024;

pub struct Mem {
    prom: [u8; MAX_PROM],
    data: [u8; MAX_DATA],
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            prom: [0; MAX_PROM],
            data: [0; MAX_DATA],
        }
    }

    pub fn programme_insert(&mut self, filename: String) {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        for i in 0..buffer.len() {
            self.prom[i] = buffer[i];
        }
    }

    pub fn get_byte_prom(&mut self, addres: usize) -> u8 {
        self.prom[addres]
    }

    pub fn get_length_prom(&mut self) -> usize {
        MAX_PROM
    }

    pub fn get_byte_data(&mut self, addres: usize) -> u8 {
        self.data[addres]
    }

    pub fn put_byte_data(&mut self, addres: usize, value: u8) {
        self.data[addres] = value;
    }

    pub fn get_length_data(&mut self) -> usize {
        MAX_DATA
    }
}
