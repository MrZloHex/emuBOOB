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

    fn load_instr(&mut self) {
        /*
        // PROM
        self.prom[0x0] = 0b11_001_001; // LBB -> NOP
        self.prom[0x1] = 0b11_000_001; // LAB
        self.prom[0x2] = 0b11_100_000; // LEA
        self.prom[0x3] = 0b11_000_110; // LAL
        self.prom[0x4] = 0b11_011_011; // LDD -> NOP
        self.prom[0x5] = 0b00_100_000; // INE
        self.prom[0x6] = 0b11_110_001; // LBL
        self.prom[0x7] = 0b00_001_001; // DCB
        self.prom[0x8] = 0b11_110_101; // LLH
        self.prom[0x9] = 0b11_010_111; // LCM
        self.prom[0xA] = 0b11_110_010; // LLC
        self.prom[0xB] = 0b11_111_001; // LMB
        self.prom[0xC] = 0b11_111_111; // HLT

        // DATA
        self.data[0x0] = 0x05;
        */

        /*
        // PROM
        self.prom[0x0] = 0b00_001_110; // LBI
        self.prom[0x1] = 0b11_111_111; // DATA FF
        self.prom[0x2] = 0b11_111_001; // LMB
        self.prom[0x3] = 0b00_110_110; // LLI
        self.prom[0x4] = 0b00_000_001; // DATA 01
        self.prom[0x5] = 0b00_111_110; // LMI
        self.prom[0x6] = 0b00_101_010; // DATA 2A
        self.prom[0x7] = 0b11_111_111; // HLT
        */

        /*
        // PROM 
        self.prom[0x0] = 0b00_000_110; // LAI
        self.prom[0x1] = 0b11_111_111; // DATA FF
        self.prom[0x2] = 0b01_111_100; // JMP
        self.prom[0x3] = 0b00_001_001; // 
        self.prom[0x4] = 0b00_000_000; // 
        self.prom[0x9] = 0b01_111_110; // CAL
        self.prom[0xA] = 0b00_001_111;
        self.prom[0xB] = 0b00_000_000;
        self.prom[0xF] = 0b11_111_111; // HLT
        */

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

        /*
        // PROM
        self.prom[0x0] = 0b00_000_110; // LAI
        self.prom[0x1] = 0b00_000_010; // 09
        self.prom[0x2] = 0b11_011_000; // LDA
        self.prom[0x3] = 0b10_000_011; // ADD
        //self.prom[0x4] = 0b00_011_100; // SBI
        //self.prom[0x5] = 0b00_000_001; //
        self.prom[0x6] = 0b11_111_111; // HLT
        */

        self.prom[0x0] = 0b00_111_100; // CPI
        self.prom[0x1] = 0b00_000_000;
        self.prom[0x2] = 0b01_100_000; // JTZ
        self.prom[0x3] = 0b00_000_111;
        self.prom[0x4] = 0b00_000_000; // CPI

        self.prom[0x7] = 0b11_111_111; // HLT
    }
}