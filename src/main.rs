use std::collections::HashMap;


const MAX_MEM: usize = 1024 * 8; // orig 16384 * 8 


fn opcodes(){
    let mut instr_set = HashMap::new();

    // load reg2 -> reg1
    // reg1 == reg2 => NOP
    let nop: [u8; 7] = [192, 201, 210, 219, 228, 237, 246];
    // reg1 != reg2 => Lrr
    for reg1 in (192..241).step_by(8) {
        for reg2 in 0..7 {
            let op = reg1 + reg2;
            if nop.contains(&op) {
                instr_set.insert(op, "NOP");
            } else {
                if reg1 == 192 {
                    match reg2 {
                        1 => instr_set.insert(op, "LAB"),
                        2 => instr_set.insert(op, "LAC"),
                        3 => instr_set.insert(op, "LAD"),
                        4 => instr_set.insert(op, "LAE"),
                        5 => instr_set.insert(op, "LAH"),
                        6 => instr_set.insert(op, "LAL"),
                        _ => continue
                    };
                } else if reg1 == 201 {
                    match reg2 {
                        0 => instr_set.insert(op, "LBA"),
                        2 => instr_set.insert(op, "LBC"),
                        3 => instr_set.insert(op, "LBD"),
                        4 => instr_set.insert(op, "LBE"),
                        5 => instr_set.insert(op, "LBH"),
                        6 => instr_set.insert(op, "LBL"),
                        _ => continue
                    };
                } else if reg1 == 210 {
                    match reg2 {
                        0 => instr_set.insert(op, "LCA"),
                        1 => instr_set.insert(op, "LCB"),
                        3 => instr_set.insert(op, "LCD"),
                        4 => instr_set.insert(op, "LCE"),
                        5 => instr_set.insert(op, "LCH"),
                        6 => instr_set.insert(op, "LCL"),
                        _ => continue
                    };
                } else if reg1 == 219 {
                    match reg2 {
                        0 => instr_set.insert(op, "LDA"),
                        1 => instr_set.insert(op, "LDB"),
                        2 => instr_set.insert(op, "LDC"),
                        4 => instr_set.insert(op, "LDE"),
                        5 => instr_set.insert(op, "LDH"),
                        6 => instr_set.insert(op, "LDL"),
                        _ => continue
                    };
                } else if reg1 == 228 {
                    match reg2 {
                        0 => instr_set.insert(op, "LEA"),
                        1 => instr_set.insert(op, "LEB"),
                        2 => instr_set.insert(op, "LEC"),
                        3 => instr_set.insert(op, "LED"),
                        5 => instr_set.insert(op, "LEH"),
                        6 => instr_set.insert(op, "LEL"),
                        _ => continue
                    };
                } else if reg1 == 237 {
                    match reg2 {
                        0 => instr_set.insert(op, "LHA"),
                        1 => instr_set.insert(op, "LHB"),
                        2 => instr_set.insert(op, "LHC"),
                        3 => instr_set.insert(op, "LHD"),
                        4 => instr_set.insert(op, "LHE"),
                        6 => instr_set.insert(op, "LHL"),
                        _ => continue
                    };
                } else if reg1 == 246 {
                    match reg2 {
                        0 => instr_set.insert(op, "LLA"),
                        1 => instr_set.insert(op, "LLB"),
                        2 => instr_set.insert(op, "LLC"),
                        3 => instr_set.insert(op, "LLD"),
                        4 => instr_set.insert(op, "LLE"),
                        5 => instr_set.insert(op, "LLH"),
                        _ => continue
                    };
                }
            }
            
        }
    }
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
        self.r_pc = 0 ;
        self.r_sp = 0;
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
}



fn main() {
    opcodes();

    let mut cpu: CPU = CPU {r_pc: 0, r_sp: 0, r_a: 0, r_b: 0, r_c: 0, r_d: 0, r_e: 0, r_h: 0, r_l: 0, 
                            f_c: false, f_z: false, f_s: false, f_p: false};

    let mut mem: MEM = MEM {data: [0; MAX_MEM]};
    
    cpu.reset(&mut mem);
    //cpu.execute(&mut mem, 2);
}
