use std::collections::HashMap;

const MAX_PROM: usize = 16; // orig 16384 * 8 
const MAX_DATA: usize = 8;

struct INSTRUCTION {
    instr_set: HashMap<u8, String>,
    instr_time: [Vec<String>; 3],
    instr_length: [Vec<String>; 3],
    instr_type: [Vec<String>; 2]
}

impl INSTRUCTION {
    fn new() -> INSTRUCTION {
        let instrucitions: INSTRUCTION = INSTRUCTION{
            instr_set: INSTRUCTION::opcodes(),
            instr_time: INSTRUCTION::time_instr(),
            instr_length: INSTRUCTION::length_instr(),
            instr_type: INSTRUCTION::type_instr()
        };
        return instrucitions;
    }

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
                    } else if reg1 == 200 {
                        match reg2 {
                            0 => instr_set.insert(op, "LBA".to_string()),
                            2 => instr_set.insert(op, "LBC".to_string()),
                            3 => instr_set.insert(op, "LBD".to_string()),
                            4 => instr_set.insert(op, "LBE".to_string()),
                            5 => instr_set.insert(op, "LBH".to_string()),
                            6 => instr_set.insert(op, "LBL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 208 {
                        match reg2 {
                            0 => instr_set.insert(op, "LCA".to_string()),
                            1 => instr_set.insert(op, "LCB".to_string()),
                            3 => instr_set.insert(op, "LCD".to_string()),
                            4 => instr_set.insert(op, "LCE".to_string()),
                            5 => instr_set.insert(op, "LCH".to_string()),
                            6 => instr_set.insert(op, "LCL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 216 {
                        match reg2 {
                            0 => instr_set.insert(op, "LDA".to_string()),
                            1 => instr_set.insert(op, "LDB".to_string()),
                            2 => instr_set.insert(op, "LDC".to_string()),
                            4 => instr_set.insert(op, "LDE".to_string()),
                            5 => instr_set.insert(op, "LDH".to_string()),
                            6 => instr_set.insert(op, "LDL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 224 {
                        match reg2 {
                            0 => instr_set.insert(op, "LEA".to_string()),
                            1 => instr_set.insert(op, "LEB".to_string()),
                            2 => instr_set.insert(op, "LEC".to_string()),
                            3 => instr_set.insert(op, "LED".to_string()),
                            5 => instr_set.insert(op, "LEH".to_string()),
                            6 => instr_set.insert(op, "LEL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 232 {
                        match reg2 {
                            0 => instr_set.insert(op, "LHA".to_string()),
                            1 => instr_set.insert(op, "LHB".to_string()),
                            2 => instr_set.insert(op, "LHC".to_string()),
                            3 => instr_set.insert(op, "LHD".to_string()),
                            4 => instr_set.insert(op, "LHE".to_string()),
                            6 => instr_set.insert(op, "LHL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 240 {
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

    fn time_instr() -> [Vec<String>; 3] {
        let one_cycle_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string(),
            "NOP".to_string()
        ];
        let two_cycle_instrs: Vec<String> = vec!["LMI".to_string()];
        let three_cycle_instrs: Vec<String> = vec!["CAL".to_string()];
        let instrs: [Vec<String>; 3] = [one_cycle_instrs, two_cycle_instrs, three_cycle_instrs];
        return instrs;
    }
    
    fn length_instr() -> [Vec<String>; 3] {
        let one_byte_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string(),
            "NOP".to_string()
        ];
        let two_byte_instrs: Vec<String> = vec!["LMI".to_string()];
        let three_byte_instrs: Vec<String> = vec!["JMP".to_string()];
        let instrs: [Vec<String>; 3] = [one_byte_instrs, two_byte_instrs, three_byte_instrs];
        return instrs;
    }
    
    fn type_instr() -> [Vec<String>; 2] {
        let index_register_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string()
        ];
        let machine_instr: Vec<String> = vec!["NOP".to_string(), "HLT".to_string()];
        let instrs: [Vec<String>; 2] = [index_register_instrs, machine_instr];
        return instrs;
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
        self.r_pc = 0;
        self.r_sp = 0;
        self.r_b = 0x0A; // for tests
        mem.initialize();
    }

    fn execute(&mut self, mem: &mut MEM, instructions: &INSTRUCTION) -> () {
        let instr: u8 = self.fetch_opcode(mem);
        let instr: String = self.decode(instr, instructions);
        let mut cycles: u8 = self.cycles(instructions, &instr);
        let length: u8 = self.length(instructions, &instr);
        let kind: String = self.kind(instructions, &instr);

        println!("Instruction:");
        println!("{}\t{}\t{}\t{}", instr, cycles, length, kind);

        let _load: String = "load".to_string();
        let _machine: String = "machine".to_string();

        #[allow(unreachable_patterns)]
        while cycles > 0 {
            match &kind {
                _load => self.load_command(&instr),
                _machine => self.machine_command(&instr)
            };
            cycles -= 1;
        }
    }
 
    fn fetch_opcode(&mut self, mem: &mut MEM) -> u8 {
        let opcode: u8 = mem.get_byte(self.r_pc as usize);
        self.r_pc += 1;
        return opcode;
    }

    fn decode(&mut self, opcode: u8, instructions: &INSTRUCTION) -> String {
        match instructions.instr_set.get(&opcode) {
            Some(instr) => return instr.to_string(),
            None => return "NOP".to_string()
        }
    }

    fn cycles(&mut self, instructions: &INSTRUCTION, instr: &String) -> u8 {
        if instructions.instr_time[0].contains(instr) {return 1}
        else if instructions.instr_time[1].contains(instr) {return 2}
        else {return 3};
    }
    fn length(&mut self, instructions: &INSTRUCTION, instr: &String) -> u8 {
        if instructions.instr_length[0].contains(instr) {return 1}
        else if instructions.instr_length[1].contains(instr) {return 2}
        else {return 3};
    }
    fn kind(&mut self, instructions: &INSTRUCTION, instr: &String) -> String{
        if instructions.instr_type[0].contains(&instr) {
            return "load".to_string()
        } else {
            return "machine".to_string()
        };
    }

    fn load_command(&mut self, instr: &String) -> () {
        // a register
        if instr == "LAB" {self.r_a = self.r_b.clone();}
        else if instr == "LAC" {self.r_a = self.r_c.clone();}
        else if instr == "LAD" {self.r_a = self.r_d.clone();}
        else if instr == "LAE" {self.r_a = self.r_e.clone();}
        else if instr == "LAH" {self.r_a = self.r_h.clone();}
        else if instr == "LAL" {self.r_a = self.r_l.clone();}
        // b register
        else if instr == "LBA" {self.r_a = self.r_a.clone();}
        else if instr == "LBC" {self.r_a = self.r_c.clone();}
        else if instr == "LBD" {self.r_a = self.r_d.clone();}
        else if instr == "LBE" {self.r_a = self.r_e.clone();}
        else if instr == "LBH" {self.r_a = self.r_h.clone();}
        else if instr == "LBL" {self.r_a = self.r_l.clone();}
        // c register
        else if instr == "LCA" {self.r_a = self.r_a.clone();}
        else if instr == "LCB" {self.r_a = self.r_b.clone();}
        else if instr == "LCD" {self.r_a = self.r_d.clone();}
        else if instr == "LCE" {self.r_a = self.r_e.clone();}
        else if instr == "LCH" {self.r_a = self.r_h.clone();}
        else if instr == "LCL" {self.r_a = self.r_l.clone();}
        // d register
        else if instr == "LDA" {self.r_a = self.r_a.clone();}
        else if instr == "LDB" {self.r_a = self.r_b.clone();}
        else if instr == "LDC" {self.r_a = self.r_c.clone();}
        else if instr == "LDE" {self.r_a = self.r_e.clone();}
        else if instr == "LDH" {self.r_a = self.r_h.clone();}
        else if instr == "LDL" {self.r_a = self.r_l.clone();}
        // e register
        else if instr == "LEA" {self.r_e = self.r_a.clone();}
        else if instr == "LEB" {self.r_a = self.r_b.clone();}
        else if instr == "LEC" {self.r_a = self.r_c.clone();}
        else if instr == "LED" {self.r_a = self.r_d.clone();}
        else if instr == "LEH" {self.r_a = self.r_h.clone();}
        else if instr == "LEL" {self.r_a = self.r_l.clone();}
        // h register
        else if instr == "LHA" {self.r_a = self.r_a.clone();}
        else if instr == "LHB" {self.r_a = self.r_b.clone();}
        else if instr == "LHC" {self.r_a = self.r_c.clone();}
        else if instr == "LHD" {self.r_a = self.r_d.clone();}
        else if instr == "LHE" {self.r_a = self.r_e.clone();}
        else if instr == "LHL" {self.r_a = self.r_l.clone();}
        // l register
        else if instr == "LLA" {self.r_a = self.r_a.clone();}
        else if instr == "LLB" {self.r_a = self.r_b.clone();}
        else if instr == "LLC" {self.r_a = self.r_c.clone();}
        else if instr == "LLD" {self.r_a = self.r_d.clone();}
        else if instr == "LLE" {self.r_a = self.r_e.clone();}
        else if instr == "LLH" {self.r_a = self.r_h.clone();}
    }

    fn machine_command(&mut self, instr: &String) -> () {
        if instr == "NOP" {
            //nothing
        }
    }

    fn print_reg(&mut self) -> () {
        println!("\nCPU DUMP");
        println!("REG A\t{:X}", self.r_a);
        println!("REG B\t{:X}", self.r_b);
        println!("REG C\t{:X}", self.r_c);
        println!("REG D\t{:X}", self.r_d);
        println!("REG E\t{:X}", self.r_e);
        println!("REG H\t{:X}", self.r_h);
        println!("REG L\t{}\n", self.r_l);
        println!("FLAG C\t{}", self.f_c);
        println!("FLAG Z\t{}", self.f_z);
        println!("FLAG S\t{}", self.f_s);
        println!("FLAG P\t{}\n", self.f_p);
        println!("REG PC\t{:x}", self.r_pc);
        println!("REG SP\t{:x}", self.r_sp);
    }
}


struct MEM {
    prom: [u8; MAX_PROM],
    data: [u8; MAX_DATA]
}

impl MEM {
    fn initialize(&mut self) -> () {
        for byte in self.prom.iter_mut() {
            *byte = 0;
        }
    }

    fn get_byte(&mut self, addres: usize) -> u8 {
        return self.prom[addres];
    }

    fn load_instr(&mut self) -> () {
        self.prom[0x1] = 0b11_001_001; // LBB -> NOP
        self.prom[0x1] = 0b11_000_001; // LAB
        self.prom[0x2] = 0b11_100_000; // LEA
    }

    fn print_dump(&mut self) -> () {
        println!("\nMEM DUMP");
        for i in 0..(self.prom.len()/2) {
            if self.prom[i] == 0 {
                println!("{number:>0width$}\t{bute:>0wi$}\t\t\t", number=i, width=3, bute=self.prom[i], wi=8);
            } else {
                println!("{number:>0width$}\t{bute:b}\t\t\t", number=i, width=3, bute=self.prom[i]);
            }
        }
    }
}



fn main() {
    let instructions: INSTRUCTION = INSTRUCTION::new();

    let mut cpu: CPU = CPU {r_pc: 0, r_sp: 0, r_a: 0, r_b: 0, r_c: 0, r_d: 0, r_e: 0, r_h: 0, r_l: 0, 
                            f_c: false, f_z: false, f_s: false, f_p: false};

    let mut mem: MEM = MEM {prom: [0; MAX_PROM], data: [0; MAX_DATA]};
    
    cpu.reset(&mut mem);
    // for test ROM
    mem.load_instr();
    mem.print_dump();
    cpu.print_reg();
    println!();
    //execute commands
    cpu.execute(&mut mem, &instructions);
    cpu.execute(&mut mem, &instructions);
    cpu.execute(&mut mem, &instructions);
    cpu.execute(&mut mem, &instructions);
    cpu.print_reg();
    println!();
}
