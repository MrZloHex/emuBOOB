use std::fs::File;
use std::fs;
use std::io::Read;

const MAX_PROM: usize = 2048; // orig 16384 * 8 
const MAX_DATA: usize = 1024;

pub struct Mem {
    prom: [u8; MAX_PROM],
    data: [u8; MAX_DATA]
}

impl Mem {
    pub fn new() -> Mem {
        Mem {prom: [0; MAX_PROM], data: [0; MAX_DATA]}
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

    fn _load_instr(&mut self) {
        /* EXAMPLE WITH NICE BRANCHING
        self.prom[0x0] = 0b01_111_110; // CAL
        self.prom[0x1] = 0b00_001_101; // 09
        self.prom[0x2] = 0b00_000_000;
        self.prom[0x3] = 0b00_111_100; // CPI
        self.prom[0x4] = 0b00_000_000; // 1
        self.prom[0x5] = 0b01_010_000; // JFS
        self.prom[0x6] = 0b00_001_100; // 0C
        self.prom[0x7] = 0b00_000_000; // 0
        self.prom[0x8] = 0b00_000_100; // ADI
        self.prom[0x9] = 0b00_001_111; // 0F
        self.prom[0xA] = 0b11_111_111; // HLT
        self.prom[0xC] = 0b11_111_111; // HLT

        self.prom[0xD] = 0b00_000_110; // LAI
        self.prom[0xE] = 0b11_000_000; // 1
        self.prom[0xF] = 0b00_111_111; // RET
        */

        // MULTIPLYING
        // PROM
        /*
        self.prom[0x0]  = 0b00_011_110; // LDI
        self.prom[0x1]  = 0b00_001_000; // 8
        self.prom[0x2]  = 0b00_100_110; // LEI
        self.prom[0x3]  = 0b00_000_110; // 6
        self.prom[0x4]  = 0b01_000_110; // CAL
        self.prom[0x5]  = 0b00_001_000; // 8
        self.prom[0x6]  = 0b00_000_000;
        self.prom[0x7]  = 0b11_111_111; // HLT
        self.prom[0x8]  = 0b00_010_110; // LCI
        self.prom[0x9]  = 0b00_000_000; // 1
        self.prom[0xA]  = 0b01_000_110; // CAL
        self.prom[0xB]  = 0b00_010_100; // 14
        self.prom[0xC]  = 0b00_000_000; 
        self.prom[0xD]  = 0b00_010_000; // INC
        self.prom[0xE]  = 0b11_000_010; // LAC
        self.prom[0xF]  = 0b10_111_100; // CPE
        self.prom[0x10] = 0b01_001_000; // JFZ
        self.prom[0x11] = 0b00_001_010; // A
        self.prom[0x12] = 0b00_000_000;
        self.prom[0x13] = 0b00_000_111; // RET
        self.prom[0x14] = 0b11_000_001; // LAB
        self.prom[0x15] = 0b10_000_011; // ADD
        self.prom[0x16] = 0b11_001_000; // LBA
        self.prom[0x17] = 0b00_000_111; // RET*/
        
        /*
        self.prom[0x0] = 0b00_000_110; // RST
        self.prom[0x1] = 0b01010101;
        self.prom[0x2] = 0b00_011_010; // HLT*/
    }
}