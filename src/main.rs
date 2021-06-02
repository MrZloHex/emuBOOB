use std::collections::HashMap;


const MAX_MEM: usize = 16; // orig 16384 * 8 


fn opcodes() -> HashMap<u8, String> {
    let mut instr_set: HashMap<u8, String> = HashMap::new();

    // load reg2 -> reg1
    // reg1 == reg2 => NOP
    let nop: [u8; 7] = [192, 201, 210, 219, 228, 237, 246];
    // reg1 != reg2 => Lrr
    for reg1 in (192..241).step_by(8) {
        for reg2 in 0..7 {
            let op = reg1 + reg2;
            if nop.contains(&op) {
                instr_set.insert(op, "NOP".to_string());
            } else {
                if reg1 == 192 {
                    match reg2 {
                        1 => instr_set.insert(op, "LAB".to_string()),
                        2 => instr_set.insert(op, "LAC".to_string()),
                        3 => instr_set.insert(op, "LAD".to_string()),
                        4 => instr_set.insert(op, "LAE".to_string()),
                        5 => instr_set.insert(op, "LAH".to_string()),
                        6 => instr_set.insert(op, "LAL".to_string()),
                        _ => continue
                    };
                } else if reg1 == 201 {
                    match reg2 {
                        0 => instr_set.insert(op, "LBA".to_string()),
                        2 => instr_set.insert(op, "LBC".to_string()),
                        3 => instr_set.insert(op, "LBD".to_string()),
                        4 => instr_set.insert(op, "LBE".to_string()),
                        5 => instr_set.insert(op, "LBH".to_string()),
                        6 => instr_set.insert(op, "LBL".to_string()),
                        _ => continue
                    };
                } else if reg1 == 210 {
                    match reg2 {
                        0 => instr_set.insert(op, "LCA".to_string()),
                        1 => instr_set.insert(op, "LCB".to_string()),
                        3 => instr_set.insert(op, "LCD".to_string()),
                        4 => instr_set.insert(op, "LCE".to_string()),
                        5 => instr_set.insert(op, "LCH".to_string()),
                        6 => instr_set.insert(op, "LCL".to_string()),
                        _ => continue
                    };
                } else if reg1 == 219 {
                    match reg2 {
                        0 => instr_set.insert(op, "LDA".to_string()),
                        1 => instr_set.insert(op, "LDB".to_string()),
                        2 => instr_set.insert(op, "LDC".to_string()),
                        4 => instr_set.insert(op, "LDE".to_string()),
                        5 => instr_set.insert(op, "LDH".to_string()),
                        6 => instr_set.insert(op, "LDL".to_string()),
                        _ => continue
                    };
                } else if reg1 == 228 {
                    match reg2 {
                        0 => instr_set.insert(op, "LEA".to_string()),
                        1 => instr_set.insert(op, "LEB".to_string()),
                        2 => instr_set.insert(op, "LEC".to_string()),
                        3 => instr_set.insert(op, "LED".to_string()),
                        5 => instr_set.insert(op, "LEH".to_string()),
                        6 => instr_set.insert(op, "LEL".to_string()),
                        _ => continue
                    };
                } else if reg1 == 237 {
                    match reg2 {
                        0 => instr_set.insert(op, "LHA".to_string()),
                        1 => instr_set.insert(op, "LHB".to_string()),
                        2 => instr_set.insert(op, "LHC".to_string()),
                        3 => instr_set.insert(op, "LHD".to_string()),
                        4 => instr_set.insert(op, "LHE".to_string()),
                        6 => instr_set.insert(op, "LHL".to_string()),
                        _ => continue
                    };
                } else if reg1 == 246 {
                    match reg2 {
                        0 => instr_set.insert(op, "LLA".to_string()),
                        1 => instr_set.insert(op, "LLB".to_string()),
                        2 => instr_set.insert(op, "LLC".to_string()),
                        3 => instr_set.insert(op, "LLD".to_string()),
                        4 => instr_set.insert(op, "LLE".to_string()),
                        5 => instr_set.insert(op, "LLH".to_string()),
                        _ => continue
                    };
                }
            }
            
        }
    }
    return instr_set;
}




struct CPU {
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
    f_p: bool
}

impl CPU {
    fn reset(&mut self, mem: &mut MEM) -> () {
        self.r_pc = 0;
        self.r_sp = 0;
        self.r_b = 0x0A; // for tests
        mem.initialize();
    }

    fn execute(&mut self, mem: &mut MEM, ticks: u32) -> () {
        let mut cycles: u32 = ticks;

        while cycles > 0 {
            let instr: u8 = self.fetch_mem(&mut cycles, mem);
            
        }
    }

    fn fetch_mem(&mut self, cycles: &mut u32, mem: &mut MEM) -> u8 {
        let data_byte: u8 = mem.get_byte(self.r_pc as usize);

        self.r_pc += 1;
        *cycles -= 1;
        return data_byte;
    }
}


struct MEM {
    data: [u8; MAX_MEM]
}

impl MEM {
    fn initialize(&mut self) -> () {
        for byte in self.data.iter_mut() {
            *byte = 0;
        }
    }

    fn get_byte(&mut self, addres: usize) -> u8 {
        return self.data[addres];
    }

    fn load_instr(&mut self) -> () {
        self.data[0x0] = 0b11_001_000; // LAB
        self.data[0x1] = 0b11_100_000; // LEA
    }

    fn print_dump(&mut self) -> () {
        println!("MEM DUMP");
        for i in 0..self.data.len() {
            println!("{number:>0width$}\t{bute:b}", number=i, width=3, bute=self.data[i]);
        }
    }
}



fn main() {
    let instr_set: HashMap<u8, String> = opcodes();

    let mut cpu: CPU = CPU {r_pc: 0, r_sp: 0, r_a: 0, r_b: 0, r_c: 0, r_d: 0, r_e: 0, r_h: 0, r_l: 0, 
                            f_c: false, f_z: false, f_s: false, f_p: false};

    let mut mem: MEM = MEM {data: [0; MAX_MEM]};
    
    cpu.reset(&mut mem);
    // for test ROM
    mem.load_instr();
    mem.print_dump();
    //cpu.execute(&mut mem, 2);
}
