const MAX_PROM: usize = 16; // orig 16384 * 8 
const MAX_DATA: usize = 8;

pub struct Mem {
    prom: [u8; MAX_PROM],
    data: [u8; MAX_DATA]
}

impl Mem {
    pub fn new() -> Mem {
        Mem {prom: [0; MAX_PROM], data: [0; MAX_DATA]}
    }

    pub fn initialize(&mut self) {
        for byte in self.prom.iter_mut() {
            *byte = 0;
        }
        self.load_instr();
    }

    pub fn get_byte_prom(&mut self, addres: usize) -> u8 {
        self.prom[addres]
    }

    pub fn get_byte_data(&mut self, addres: usize) -> u8 {
        self.data[addres]
    }

    pub fn get_length_prom(&mut self) -> usize {
        MAX_PROM
    }

    pub fn get_length_data(&mut self) -> usize {
        MAX_DATA
    }

    fn load_instr(&mut self) {
        self.prom[0x0] = 0b11_001_001; // LBB -> NOP
        self.prom[0x1] = 0b11_000_001; // LAB
        self.prom[0x2] = 0b11_100_000; // LEA
        self.prom[0xA] = 0b11_000_110; // LAL
        self.prom[0xB] = 0b11_011_011; // LDD -> NOP
    }
}