//use ux::{u1, u14};

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

    // stack  14-bit
    stack: [u16; 7],

    instruct: instructions::Instruction
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {r_pc: 0, r_sp: 0, r_a: 0, r_b: 0, r_c: 0, r_d: 0, r_e: 0, r_h: 0, r_l: 0, 
            f_c: false, f_z: false, f_s: false, f_p: false,
            stack: [0; 7],
            instruct: instructions::Instruction::new()}
    }

    pub fn reset(&mut self) {
        self.r_pc = 0;
        self.r_sp = 0;
    }

    pub fn execute(&mut self, mem: &mut mem::Mem) -> Result<bool, ()> {
        let instr: u8 = self.fetch_opcode(mem);
        let instr: String = self.decode(instr);
        let mut cycles: u8 = self.cycles(&instr);
        let length: u8 = self.length(&instr);
        let kind: String = self.kind(&instr);

        //println!("{}\t{}\t{}\t{}\t\t{:X}", instr, cycles, length, kind, self.r_pc);

        while cycles > 0 {
            if      &kind == "index" {self.index_command(&instr, &mut cycles, &length, mem)}
            else if &kind == "accum" {self.accumulator_command(&instr, &mut cycles, &length, mem)}
            else if &kind == "stack" {self.stack_command(&instr, &mut cycles, &length, mem)}
            else if &kind == "machine" && &instr == "HLT"{return Ok(true)}
            cycles -= 1;
        }
        
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
        else if self.instruct.get_instr_type()[1].contains(instr) {"accum".to_string()}
        else if self.instruct.get_instr_type()[2].contains(instr) {"stack".to_string()}
        else {"machine".to_string()}
    }

    fn index_command(&mut self, instr: &String, cycle: &mut u8, length: &u8, mem: &mut mem::Mem) {
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
            else if instr == "INB" {self.r_b = self.inc(&(self.r_b as i16));}
            else if instr == "INC" {self.r_c = self.inc(&(self.r_c as i16));}
            else if instr == "IND" {self.r_d = self.inc(&(self.r_d as i16));}
            else if instr == "INE" {self.r_e = self.inc(&(self.r_e as i16));}
            else if instr == "INH" {self.r_h = self.inc(&(self.r_h as i16));}
            else if instr == "INL" {self.r_l = self.inc(&(self.r_l as i16));}
            // dec
            else if instr == "DCB" {self.r_b = self.dec(&(self.r_b as i16));}
            else if instr == "DCC" {self.r_c = self.dec(&(self.r_c as i16));}
            else if instr == "DCD" {self.r_d = self.dec(&(self.r_d as i16));}
            else if instr == "DCE" {self.r_e = self.dec(&(self.r_e as i16));}
            else if instr == "DCH" {self.r_h = self.dec(&(self.r_h as i16));}
            else if instr == "DCL" {self.r_l = self.dec(&(self.r_l as i16));}
        }

        else if *cycle == 2 {
            let load_byte: u8 = if *length == 2 {
                self.r_pc += 1;
                *cycle -=1;
                let byte: u8 = self.fetch_opcode(mem);
                byte
            } else {0};
            // LOAD REG <- DATA  IMMEDIATE
            if      instr == "LAI" {self.r_a = load_byte;}
            else if instr == "LBI" {self.r_b = load_byte;}
            else if instr == "LCI" {self.r_c = load_byte;}
            else if instr == "LDI" {self.r_d = load_byte;}
            else if instr == "LEI" {self.r_e = load_byte;}
            else if instr == "LHI" {self.r_h = load_byte;}
            else if instr == "LLI" {self.r_l = load_byte;}
            // INSTRUCTIONS WHICH USES MEMORY
            else {   
                let address: usize = (((self.r_h.clone() as u16) << 8) | (self.r_l.clone() as u16)) as usize;
                let byte_data: u8 = self.fetch_byte(mem, &address);
                if *cycle == 2 {*cycle -= 1;};
                // LOAD REG <- MEM
                if      instr == "LAM" {self.r_a = byte_data;}
                else if instr == "LBM" {self.r_b = byte_data;}
                else if instr == "LCM" {self.r_c = byte_data;}
                else if instr == "LDM" {self.r_d = byte_data;}
                else if instr == "LEM" {self.r_e = byte_data;}
                else if instr == "LHM" {self.r_h = byte_data;}
                else if instr == "LLM" {self.r_l = byte_data;}
                // LOAD MEM <- REG
                else if instr == "LMA" {mem.put_byte_data(address, self.r_a.clone());}
                else if instr == "LMB" {mem.put_byte_data(address, self.r_b.clone());}
                else if instr == "LMC" {mem.put_byte_data(address, self.r_c.clone());}
                else if instr == "LMD" {mem.put_byte_data(address, self.r_d.clone());}
                else if instr == "LME" {mem.put_byte_data(address, self.r_e.clone());}
                else if instr == "LMH" {mem.put_byte_data(address, self.r_h.clone());}
                else if instr == "LML" {mem.put_byte_data(address, self.r_l.clone());}
                
            };
        }
        
        else if *cycle == 3 {
            self.r_pc += 1;
            let load_byte: u8 = self.fetch_opcode(mem);
            *cycle -= 1;

            let address: usize = (((self.r_h.clone() as u16) << 8) | (self.r_l.clone() as u16)) as usize;
            *cycle -= 1;

            // LOAD MEM <- DATA IMMEDIATE
            if instr == "LMI" {mem.put_byte_data(address, load_byte);}
        }
        self.r_pc += 1;
    }

    fn accumulator_command(&mut self, instr: &String, cycle: &mut u8, length: &u8, mem: &mut mem::Mem) {
        // ALL THIC COMMAND AFFECT ON FLAGS
        if *length == 1{
            if *cycle == 1 {
                // ADr
                if      instr == "ADB" {self.r_a = self.add(&(self.r_b as i16));}
                else if instr == "ADC" {self.r_a = self.add(&(self.r_c as i16));}
                else if instr == "ADD" {self.r_a = self.add(&(self.r_d as i16));}
                else if instr == "ADE" {self.r_a = self.add(&(self.r_e as i16));}
                else if instr == "ADH" {self.r_a = self.add(&(self.r_h as i16));}
                else if instr == "ADL" {self.r_a = self.add(&(self.r_l as i16));}
                // ACr
                else if instr == "ACB" {self.r_a = self.add_carry(&(self.r_b as i16));}
                else if instr == "ACC" {self.r_a = self.add_carry(&(self.r_c as i16));}
                else if instr == "ACD" {self.r_a = self.add_carry(&(self.r_d as i16));}
                else if instr == "ACE" {self.r_a = self.add_carry(&(self.r_e as i16));}
                else if instr == "ACH" {self.r_a = self.add_carry(&(self.r_h as i16));}
                else if instr == "ACL" {self.r_a = self.add_carry(&(self.r_l as i16));}
                // SUr
                else if instr == "SUB" {self.r_a = self.sub(&(self.r_b as i16));}
                else if instr == "SUC" {self.r_a = self.sub(&(self.r_c as i16));}
                else if instr == "SUD" {self.r_a = self.sub(&(self.r_d as i16));}
                else if instr == "SUE" {self.r_a = self.sub(&(self.r_e as i16));}
                else if instr == "SUH" {self.r_a = self.sub(&(self.r_h as i16));}
                else if instr == "SUL" {self.r_a = self.sub(&(self.r_l as i16));}
                // SBr
                else if instr == "SBB" {self.r_a = self.sub_borrow(&(self.r_b as i16));}
                else if instr == "SBC" {self.r_a = self.sub_borrow(&(self.r_c as i16));}
                else if instr == "SBD" {self.r_a = self.sub_borrow(&(self.r_d as i16));}
                else if instr == "SBE" {self.r_a = self.sub_borrow(&(self.r_e as i16));}
                else if instr == "SBH" {self.r_a = self.sub_borrow(&(self.r_h as i16));}
                else if instr == "SBL" {self.r_a = self.sub_borrow(&(self.r_l as i16));}
                // NDr
                else if instr == "NDB" {self.r_a = self.and(&(self.r_b as i16));}
                else if instr == "NDC" {self.r_a = self.and(&(self.r_c as i16));}
                else if instr == "NDD" {self.r_a = self.and(&(self.r_d as i16));}
                else if instr == "NDE" {self.r_a = self.and(&(self.r_e as i16));}
                else if instr == "NDH" {self.r_a = self.and(&(self.r_h as i16));}
                else if instr == "NDL" {self.r_a = self.and(&(self.r_l as i16));}
                // XRr
                else if instr == "XRB" {self.r_a = self.xor(&(self.r_b as i16));}
                else if instr == "XRC" {self.r_a = self.xor(&(self.r_c as i16));}
                else if instr == "XRD" {self.r_a = self.xor(&(self.r_d as i16));}
                else if instr == "XRE" {self.r_a = self.xor(&(self.r_e as i16));}
                else if instr == "XRH" {self.r_a = self.xor(&(self.r_h as i16));}
                else if instr == "XRL" {self.r_a = self.xor(&(self.r_l as i16));}
                // ORr
                else if instr == "ORB" {self.r_a = self.or(&(self.r_b as i16));}
                else if instr == "ORC" {self.r_a = self.or(&(self.r_c as i16));}
                else if instr == "ORD" {self.r_a = self.or(&(self.r_d as i16));}
                else if instr == "ORE" {self.r_a = self.or(&(self.r_e as i16));}
                else if instr == "ORH" {self.r_a = self.or(&(self.r_h as i16));}
                else if instr == "ORL" {self.r_a = self.or(&(self.r_l as i16));}
                // CPr
                else if instr == "CPB" {self.compare(&(self.r_b as i16));}
                else if instr == "CPC" {self.compare(&(self.r_c as i16));}
                else if instr == "CPD" {self.compare(&(self.r_d as i16));}
                else if instr == "CPE" {self.compare(&(self.r_e as i16));}
                else if instr == "CPH" {self.compare(&(self.r_h as i16));}
                else if instr == "CPL" {self.compare(&(self.r_l as i16));}

                // ROTATE
                else if instr == "RLC" {
                    let msb: u8 = self.r_a.clone() >> 7;
                    if msb == 1 {self.f_c = true}
                    else {self.f_c = false}
                    self.r_a = self.r_a.rotate_left(1);
                }
                else if instr == "RRC" {
                    let lsb: u8 = (self.r_a.clone() << 7) >> 7;
                    if lsb == 1 {self.f_c = true}
                    else {self.f_c = false}
                    self.r_a = self.r_a.rotate_right(1);
                }
                else if instr == "RAL" {
                    let lsb: u8 = if self.f_c {1} else {0};
                    let msb: u8 = self.r_a.clone() >> 7;
                    if msb == 1 {self.f_c = true}
                    else {self.f_c = false}
                    
                    self.r_a = self.r_a.rotate_left(1);
                    {
                        let mut buf: u8 = self.r_a.clone();
                        if (buf % 2) == 0 {
                            buf += lsb;
                        } else {
                            if lsb == 1 {}
                            else {buf -= 1;}
                        }
                        self.r_a = buf;
                    } 
                }
                else if instr == "RAR" {
                    let msb: u8 = if self.f_c {1} else {0};
                    let lsb: u8 = (self.r_a.clone() << 7) >> 7 ;
                    if lsb == 1 {self.f_c = true}
                    else {self.f_c = false}
                    self.r_a = self.r_a.rotate_right(1);
                    {
                        let mut buf: u8 = self.r_a.clone().rotate_left(1);
                        if (buf % 2) == 0 {
                            buf += msb;
                        } else {
                            if msb == 1 {}
                            else {buf -= 1;}
                        }
                        self.r_a = buf.rotate_right(1);
                    }
                }
            }
            else if *cycle == 2 {
                let address: usize = (((self.r_h.clone() as u16) << 8) | (self.r_l.clone() as u16)) as usize;
                let byte_data: u8 = self.fetch_byte(mem, &address);
                *cycle -= 1;

                // ADM
                if instr == "ADM" {self.r_a = self.add(&(byte_data as i16))}
                // ACM
                else if instr == "ACM" {self.r_a = self.add_carry(&(byte_data as i16))}
                // SUM
                else if instr == "SUM" {self.r_a = self.sub(&(byte_data as i16))}
                // SBM
                else if instr == "SBM" {self.r_a = self.sub_borrow(&(byte_data as i16))}
                // NDM
                else if instr == "NDM" {self.r_a = self.and(&(byte_data as i16))}
                // XRM
                else if instr == "XRM" {self.r_a = self.xor(&(byte_data as i16))}
                // ORM
                else if instr == "ORM" {self.r_a = self.or(&(byte_data as i16))}
                // CPM 
                else if instr == "CPM" {self.compare(&(byte_data as i16))}
            };
        } else if *length == 2 {
            self.r_pc += 1;
            let byte_data: u8 = self.fetch_opcode(mem);
            *cycle -= 1;

            // ADI
            if instr == "ADI" {self.r_a = self.add(&(byte_data as i16))}
            // ACI
            else if instr == "ACI" {self.r_a = self.add_carry(&(byte_data as i16))}
            // SUI
            else if instr == "SUI" {self.r_a = self.sub(&(byte_data as i16))}
            // SBI
            else if instr == "SBI" {self.r_a = self.sub_borrow(&(byte_data as i16))}
            // NDI
            else if instr == "NDI" {self.r_a = self.and(&(byte_data as i16))}
            // XRI
            else if instr == "XRI" {self.r_a = self.xor(&(byte_data as i16))}
            // ORI
            else if instr == "ORI" {self.r_a = self.or(&(byte_data as i16))}
            // CPI 
            else if instr == "CPI" {self.compare(&(byte_data as i16))}
        } else {};
        self.r_pc += 1;
    }

    fn add(&mut self, b: &i16) -> u8 {
        let result: i16 = (self.r_a as i16) + (*b);
        self.set_flags(&result);
        result as u8
    }
    fn add_carry(&mut self, b: &i16) -> u8 {
        let result: i16 = if self.f_c {
            (self.r_a as i16) + (*b) + 1
        } else {
            (self.r_a as i16) + (*b)
        };
        self.set_flags(&result);
        result as u8
    }
    fn sub(&mut self, b: &i16) -> u8 {
        let result: i16 = (self.r_a as i16) - (*b);
        self.set_flags(&result);
        result as u8
    }
    fn sub_borrow(&mut self, b: &i16) -> u8 {
        let result: i16 = if self.f_c {
            (self.r_a as i16) - (*b) - 1
        } else {
            (self.r_a as i16) - (*b)
        };
        self.set_flags(&result);
        result as u8
    }
    fn and(&mut self, b: &i16) -> u8 {
        let result: u8 = (self.r_a) & (*b as u8);
        self.set_flags(&(result as i16));
        result
    }
    fn xor(&mut self, b: &i16) -> u8 {
        let result: u8 = (self.r_a) ^ (*b as u8);
        self.set_flags(&(result as i16));
        result
    }
    fn or(&mut self, b: &i16) -> u8 {
        let result: u8 = (self.r_a) | (*b as u8);
        self.set_flags(&(result as i16));
        result
    }
    fn compare(&mut self, b: &i16) {
        let result: i16 = (self.r_a as i16) - *b;
        self.set_flags(&result);
    }
    fn inc(&mut self, b: &i16) -> u8 {
        let result: u8 = (*b as u8) + 1;
        self.set_flags(&(result as i16));
        result
    }
    fn dec(&mut self, b: &i16) -> u8 {
        let result: u8 = (*b as u8) - 1;
        self.set_flags(&(result as i16));
        result
    }

    fn stack_command(&mut self, instr: &String, cycle: &mut u8, _length: &u8, mem: &mut mem::Mem) {
        if *cycle == 3 {
            self.r_pc += 1;
            let low_byte: u8 = self.fetch_opcode(mem);
            *cycle -= 1;
            self.r_pc += 1;
            let high_byte: u8 = self.fetch_opcode(mem);
            *cycle -= 1;
            let address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
            
            // JMP
            if      instr == "JMP" {self.r_pc = address;}
            // JUMP FALSE -> FLAG
            else if instr == "JFC" {if !self.f_c {self.r_pc = address;} else {self.r_pc += 1;}}
            else if instr == "JFZ" {if !self.f_z {self.r_pc = address;} else {self.r_pc += 1;}}
            else if instr == "JFS" {if !self.f_s {self.r_pc = address;} else {self.r_pc += 1;}}
            else if instr == "JFP" {if !self.f_p {self.r_pc = address;} else {self.r_pc += 1;}}
            // JUMP TRUE -> FLAG
            else if instr == "JTC" {if self.f_c {self.r_pc = address;} else {self.r_pc += 1;}}
            else if instr == "JTZ" {if self.f_z {self.r_pc = address;} else {self.r_pc += 1;}}
            else if instr == "JTS" {if self.f_s {self.r_pc = address;} else {self.r_pc += 1;}}
            else if instr == "JTP" {if self.f_p {self.r_pc = address;} else {self.r_pc += 1;}}

            // CALL
            else if instr == "CAL" {
                self.stack[self.r_sp as usize] = self.r_pc;
                self.r_sp += 1;
                self.r_pc = address;
            }
            // CALL FALSE -> FLAG
            else if instr == "CFC" {
                if !self.f_c {
                    self.stack[self.r_sp as usize] = self.r_pc;
                    self.r_sp += 1;
                    self.r_pc = address;
                } else {self.r_pc += 1}
            }
            else if instr == "CFZ" {
                if !self.f_z {
                    self.stack[self.r_sp as usize] = self.r_pc;
                    self.r_sp += 1;
                    self.r_pc = address;
                } else {self.r_pc += 1}
            }
            else if instr == "CFS" {
                if !self.f_s {
                    self.stack[self.r_sp as usize] = self.r_pc;
                    self.r_sp += 1;
                    self.r_pc = address;
                } else {self.r_pc += 1}
            }
            else if instr == "CFP" {
                if !self.f_p {
                    self.stack[self.r_sp as usize] = self.r_pc;
                    self.r_sp += 1;
                    self.r_pc = address;
                } else {self.r_pc += 1}
            }
            // CALL TRUE -> FLAG
            else if instr == "CTC" {
                if self.f_c {
                    self.stack[self.r_sp as usize] = self.r_pc;
                    self.r_sp += 1;
                    self.r_pc = address;
                } else {self.r_pc += 1}
            }
            else if instr == "CTZ" {
                if self.f_z {
                    self.stack[self.r_sp as usize] = self.r_pc;
                    self.r_sp += 1;
                    self.r_pc = address;
                } else {self.r_pc += 1}
            }
            else if instr == "CTS" {
                if self.f_s {
                    self.stack[self.r_sp as usize] = self.r_pc;
                    self.r_sp += 1;
                    self.r_pc = address;
                } else {self.r_pc += 1}
            }
            else if instr == "CTP" {
                if self.f_p {
                    self.stack[self.r_sp as usize] = self.r_pc;
                    self.r_sp += 1;
                    self.r_pc = address;
                } else {self.r_pc += 1}
            }
        }
        else {
            // RETURN
            if instr == "RET" {
                self.r_sp -= 1;
                self.r_pc = self.stack[self.r_sp as usize];
                self.r_pc += 1;
            }
            // RETURN FALSE -> FLAG
            else if instr == "RFC" {
                if !self.f_c {
                    self.r_sp -= 1;
                    self.r_pc = self.stack[self.r_sp as usize];
                    self.r_pc += 1;
                } else {self.r_pc += 1}
            }
            else if instr == "RFZ" {
                if !self.f_z {
                    self.r_sp -= 1;
                    self.r_pc = self.stack[self.r_sp as usize];
                    self.r_pc += 1;
                } else {self.r_pc += 1}
            }
            else if instr == "RFS" {
                if !self.f_s {
                    self.r_sp -= 1;
                    self.r_pc = self.stack[self.r_sp as usize];
                    self.r_pc += 1;
                } else {self.r_pc += 1}
            }
            else if instr == "RFP" {
                if !self.f_p {
                    self.r_sp -= 1;
                    self.r_pc = self.stack[self.r_sp as usize];
                    self.r_pc += 1;
                } else {self.r_pc += 1}
            }
            // RETURN TRUE -> FLAG
            if instr == "RTC" {
                if self.f_c {
                    self.r_sp -= 1;
                    self.r_pc = self.stack[self.r_sp as usize];
                    self.r_pc += 1;
                } else {self.r_pc += 1}
            }
            else if instr == "RTZ" {
                if self.f_z {
                    self.r_sp -= 1;
                    self.r_pc = self.stack[self.r_sp as usize];
                    self.r_pc += 1;
                } else {self.r_pc += 1}
            }
            else if instr == "RTS" {
                if self.f_s {
                    self.r_sp -= 1;
                    self.r_pc = self.stack[self.r_sp as usize];
                    self.r_pc += 1;
                } else {self.r_pc += 1}
            }
            else if instr == "RTP" {
                if self.f_p {
                    self.r_sp -= 1;
                    self.r_pc = self.stack[self.r_sp as usize];
                    self.r_pc += 1;
                } else {self.r_pc += 1}
            }


            // RST
            else if instr == "RST" {
                let address: u8 = self.fetch_opcode(mem) >> 3;
                self.r_pc += 1;

                self.stack[self.r_sp as usize] = self.r_pc;
                self.r_sp += 1;
                self.r_pc = (address << 3) as u16;
            }
        }
    }

    fn reset_flags(&mut self) {
        self.f_c = false;
        self.f_z = false;
        self.f_s = false;
        self.f_p = false;
    }
    pub fn set_flags(&mut self, result: &i16) {
        self.reset_flags();
        // PARITY
        if (*result % 2) == 0 {self.f_p = true}
        else {self.f_p = false}
        // ZERO
        if *result == 0 {self.f_z = true}
        else {self.f_z = false;}
        // CARRY
        if (*result > 255) || (*result < 0) {self.f_c = true}
        else {self.f_c = false}
        // SIGN
        let msb: u8 = (*result as u8) >> 7;
        if msb == 1 {self.f_s = true}
        else {self.f_s = false}
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
    pub fn get_byte_stack(&mut self, address: usize) -> u16 {self.stack[address]}
}