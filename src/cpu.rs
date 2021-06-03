use super::mem;
use super::instructions;

pub struct Cpu {
    // programme counter 14-bit
    r_pc: u16,
    // stack pointer 3-bit
    r_sp: u8,

    // accumulator
    r_a: u8,
    // 8-bit registers
    r_b: u8,
    r_c: u8,
    r_d: u8,
    r_e: u8,
    r_h: u8,
    r_l: u8,    

    // status flags
    f_c: bool,
    f_z: bool,
    f_s: bool,
    f_p: bool,

    instruct: instructions::Instruction
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {r_pc: 0, r_sp: 0, r_a: 0, r_b: 0, r_c: 0, r_d: 0, r_e: 0, r_h: 0, r_l: 0, 
            f_c: false, f_z: false, f_s: false, f_p: false,
            instruct: instructions::Instruction::new()}
    }

    pub fn reset(&mut self, mem: &mut mem::Mem) {
        self.r_pc = 0;
        self.r_sp = 0;
        self.r_b = 0x0A; // for tests
        mem.initialize();
    }

    pub fn execute(&mut self, mem: &mut mem::Mem) -> Result<bool, ()> {
        let instr: u8 = self.fetch_opcode(mem);
        let instr: String = self.decode(instr);
        let mut cycles: u8 = self.cycles(&instr);
        let length: u8 = self.length(&instr);
        let kind: String = self.kind(&instr);

        println!("{}\t{}\t{}\t{}", instr, cycles, length, kind);

        while cycles > 0 {
            if &kind == "index" {self.index_command(&instr, &mut cycles, &length, mem)}
            else if &kind == "machine" && &instr == "HLT"{return Ok(true)}
            cycles -= 1;
        }
        self.r_pc += 1;
        Ok(false)
    }

    fn fetch_byte(&mut self, mem: &mut mem::Mem, addres: &usize) -> u8 {
        let byte: u8 = mem.get_byte_data(*addres);
        byte
    }
 
    fn fetch_opcode(&mut self, mem: &mut mem::Mem) -> u8 {
        let opcode: u8 = mem.get_byte_prom(self.r_pc as usize);
        opcode
    }

    fn decode(&mut self, opcode: u8) -> String {
        match self.instruct.get_instr_set().get(&opcode) {
            Some(instr) => instr.to_string(),
            None => "NOP".to_string()
        }
    }

    fn cycles(&mut self, instr: &String) -> u8 {
        if self.instruct.get_instr_time()[0].contains(instr) {1}
        else if self.instruct.get_instr_time()[1].contains(instr) {2}
        else {3}
    }
    fn length(&mut self, instr: &String) -> u8 {
        if self.instruct.get_instr_length()[0].contains(instr) {1}
        else if self.instruct.get_instr_length()[1].contains(instr) {2}
        else {3}
    }
    fn kind(&mut self, instr: &String) -> String{
        if self.instruct.get_instr_type()[0].contains(instr) {"index".to_string()}
        else {"machine".to_string()}
        //else if self.instruct.get_instr_type()[1].contains(instr) {"machine".to_string()}
        
    }

    fn index_command(&mut self, instr: &String, cycle: &mut u8, _length: &u8, mem: &mut mem::Mem) {
        if *cycle == 1 {
            // LOAD REG REG
            // a register
            if      instr == "LAB" {self.r_a = self.r_b.clone();}
            else if instr == "LAC" {self.r_a = self.r_c.clone();}
            else if instr == "LAD" {self.r_a = self.r_d.clone();}
            else if instr == "LAE" {self.r_a = self.r_e.clone();}
            else if instr == "LAH" {self.r_a = self.r_h.clone();}
            else if instr == "LAL" {self.r_a = self.r_l.clone();}
            // b register
            else if instr == "LBA" {self.r_b = self.r_a.clone();}
            else if instr == "LBC" {self.r_b = self.r_c.clone();}
            else if instr == "LBD" {self.r_b = self.r_d.clone();}
            else if instr == "LBE" {self.r_b = self.r_e.clone();}
            else if instr == "LBH" {self.r_b = self.r_h.clone();}
            else if instr == "LBL" {self.r_b = self.r_l.clone();}
            // c register
            else if instr == "LCA" {self.r_c = self.r_a.clone();}
            else if instr == "LCB" {self.r_c = self.r_b.clone();}
            else if instr == "LCD" {self.r_c = self.r_d.clone();}
            else if instr == "LCE" {self.r_c = self.r_e.clone();}
            else if instr == "LCH" {self.r_c = self.r_h.clone();}
            else if instr == "LCL" {self.r_c = self.r_l.clone();}
            // d register
            else if instr == "LDA" {self.r_d = self.r_a.clone();}
            else if instr == "LDB" {self.r_d = self.r_b.clone();}
            else if instr == "LDC" {self.r_d = self.r_c.clone();}
            else if instr == "LDE" {self.r_d = self.r_e.clone();}
            else if instr == "LDH" {self.r_d = self.r_h.clone();}
            else if instr == "LDL" {self.r_d = self.r_l.clone();}
            // e register
            else if instr == "LEA" {self.r_e = self.r_a.clone();}
            else if instr == "LEB" {self.r_e = self.r_b.clone();}
            else if instr == "LEC" {self.r_e = self.r_c.clone();}
            else if instr == "LED" {self.r_e = self.r_d.clone();}
            else if instr == "LEH" {self.r_e = self.r_h.clone();}
            else if instr == "LEL" {self.r_e = self.r_l.clone();}
            // h register
            else if instr == "LHA" {self.r_h = self.r_a.clone();}
            else if instr == "LHB" {self.r_h = self.r_b.clone();}
            else if instr == "LHC" {self.r_h = self.r_c.clone();}
            else if instr == "LHD" {self.r_h = self.r_d.clone();}
            else if instr == "LHE" {self.r_h = self.r_e.clone();}
            else if instr == "LHL" {self.r_h = self.r_l.clone();}
            // l register
            else if instr == "LLA" {self.r_l = self.r_a.clone();}
            else if instr == "LLB" {self.r_l = self.r_b.clone();}
            else if instr == "LLC" {self.r_l = self.r_c.clone();}
            else if instr == "LLD" {self.r_l = self.r_d.clone();}
            else if instr == "LLE" {self.r_l = self.r_e.clone();}
            else if instr == "LLH" {self.r_l = self.r_h.clone();}

            // INCREMENT / DECREMENT
            // inc
            else if instr == "INB" {self.in_dc_flags("b"); self.r_b += 1;}
            else if instr == "INC" {self.in_dc_flags("c"); self.r_c += 1;}
            else if instr == "IND" {self.in_dc_flags("d"); self.r_d += 1;}
            else if instr == "INE" {self.in_dc_flags("e"); self.r_e += 1;}
            else if instr == "INH" {self.in_dc_flags("h"); self.r_h += 1;}
            else if instr == "INL" {self.in_dc_flags("l"); self.r_l += 1;}
            // dec
            else if instr == "DCB" {self.in_dc_flags("b"); self.r_b -= 1;}
            else if instr == "DCC" {self.in_dc_flags("c"); self.r_c -= 1;}
            else if instr == "DCD" {self.in_dc_flags("d"); self.r_d -= 1;}
            else if instr == "DCE" {self.in_dc_flags("e"); self.r_e -= 1;}
            else if instr == "DCH" {self.in_dc_flags("h"); self.r_h -= 1;}
            else if instr == "DCL" {self.in_dc_flags("l"); self.r_l -= 1;}
        }

        else if *cycle == 2 {
            let addres: usize = (((self.r_h.clone() as u16) << 8) | (self.r_l.clone() as u16)) as usize;
            let byte_data: u8 = self.fetch_byte(mem, &addres);
            *cycle -= 1;
            // LOAD REG <- MEM
            if      instr == "LAM" {self.r_b = byte_data;}
            else if instr == "LBM" {self.r_b = byte_data;}
            else if instr == "LCM" {self.r_c = byte_data;}
            else if instr == "LDM" {self.r_d = byte_data;}
            else if instr == "LEM" {self.r_e = byte_data;}
            else if instr == "LHM" {self.r_h = byte_data;}
            else if instr == "LLM" {self.r_l = byte_data;}
        }
    }   

    fn in_dc_flags(&mut self, _reg: &str) {/*
        if reg == "b" {
            if self.
        }*/
    }

    // methods for know all registers and flags value
    pub fn get_r_pc(&mut self) -> u16 {self.r_pc}
    pub fn get_r_sp(&mut self) -> u8 {self.r_sp}
    pub fn get_r_a(&mut self) -> u8 {self.r_a}
    pub fn get_r_b(&mut self) -> u8 {self.r_b}
    pub fn get_r_c(&mut self) -> u8 {self.r_c}
    pub fn get_r_d(&mut self) -> u8 {self.r_d}
    pub fn get_r_e(&mut self) -> u8 {self.r_e}
    pub fn get_r_h(&mut self) -> u8 {self.r_h}
    pub fn get_r_l(&mut self) -> u8 {self.r_l}
    pub fn get_f_c(&mut self) -> bool {self.f_c}
    pub fn get_f_z(&mut self) -> bool {self.f_z}
    pub fn get_f_s(&mut self) -> bool {self.f_s}
    pub fn get_f_p(&mut self) -> bool {self.f_p}
}