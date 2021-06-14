use std::fs;
use std::fs::File;
use std::io::Read;

const MAX_PROM: usize = 2048;
const MAX_DATA: usize = 1024;

pub struct Mem {
    // PROM
    prom: [u8; MAX_PROM],
    // RAM
    data: [u8; MAX_DATA],
}

impl Default for Mem {
    fn default() -> Self {
        Mem::new()
    }
}

impl Mem {
    // Constructor
    fn new() -> Mem {
        Mem {
            prom: [0; MAX_PROM],
            data: [0; MAX_DATA],
        }
    }

    // Load programme from binary file into PROM
    pub fn programme_insert(&mut self, filename: String) {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read_exact(&mut buffer).expect("buffer overflow");
        // Inserting
        self.prom[..buffer.len()].clone_from_slice(&buffer[..]);
    }

    // Get opcode from PROM
    pub fn get_byte_prom(&mut self, addres: usize) -> u8 {
        self.prom[addres]
    }

    // Get size of PROM
    pub fn get_length_prom(&mut self) -> usize {
        MAX_PROM
    }

    // Get byte from RAM
    pub fn get_byte_data(&mut self, addres: usize) -> u8 {
        self.data[addres]
    }

    // Insert byte into RAM
    pub fn put_byte_data(&mut self, addres: usize, value: u8) {
        self.data[addres] = value;
    }

    // Get size of RAM
    pub fn get_length_data(&mut self) -> usize {
        MAX_DATA
    }
}
