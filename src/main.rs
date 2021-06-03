#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::ptr_arg)]

#![allow(clippy::collapsible_else_if)] // TRY TO FIX IT!!!

mod intstructions;
use intstructions::Instruction;

const MAX_PROM: usize = 16; // orig 16384 * 8 
const MAX_DATA: usize = 8;

struct Cpu {
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

impl Cpu {
    fn reset(&mut self, mem: &mut Mem) {
        self.r_pc = 0;
        self.r_sp = 0;
        self.r_b = 0x0A; // for tests
        mem.initialize();
    }

    fn execute(&mut self, mem: &mut Mem, instructions: &Instruction) {
        let instr: u8 = self.fetch_opcode(mem);
        let instr: String = self.decode(instr, instructions);
        let mut cycles: u8 = self.cycles(instructions, &instr);
        let length: u8 = self.length(instructions, &instr);
        let kind: String = self.kind(instructions, &instr);

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
        self.r_pc += 1;
    }
 
    fn fetch_opcode(&mut self, mem: &mut Mem) -> u8 {
        let opcode: u8 = mem.get_byte(self.r_pc as usize);
        opcode
    }

    fn decode(&mut self, opcode: u8, instructions: &Instruction) -> String {
        match instructions.instr_set.get(&opcode) {
            Some(instr) => instr.to_string(),
            None => "NOP".to_string()
        }
    }

    fn cycles(&mut self, instructions: &Instruction, instr: &String) -> u8 {
        if instructions.instr_time[0].contains(instr) {1}
        else if instructions.instr_time[1].contains(instr) {2}
        else {3}
    }
    fn length(&mut self, instructions: &Instruction, instr: &String) -> u8 {
        if instructions.instr_length[0].contains(instr) {1}
        else if instructions.instr_length[1].contains(instr) {2}
        else {3}
    }
    fn kind(&mut self, instructions: &Instruction, instr: &String) -> String{
        if instructions.instr_type[0].contains(&instr) {
            "load".to_string()
        } else {
            "machine".to_string()
        }
    }

    fn load_command(&mut self, instr: &String) {
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

    fn machine_command(&mut self, instr: &String) {
        if instr == "NOP" {
            //nothing
        }
    }
}


struct Mem {
    prom: [u8; MAX_PROM],
    data: [u8; MAX_DATA]
}

impl Mem {
    fn initialize(&mut self) {
        for byte in self.prom.iter_mut() {
            *byte = 0;
        }
    }

    fn get_byte(&mut self, addres: usize) -> u8 {
        self.prom[addres]
    }

    fn load_instr(&mut self) {
        self.prom[0x0] = 0b11_001_001; // LBB -> NOP
        self.prom[0x1] = 0b11_000_001; // LAB
        self.prom[0x2] = 0b11_100_000; // LEA
        self.prom[0xA] = 0b11_000_110; // LAL
        self.prom[0xB] = 0b11_011_011; // LDD -> NOP
    }
}

trait Dump {
    fn print_dump(&mut self);
}

impl Dump for Cpu {
    fn print_dump(&mut self) {
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
        println!("REG PC\t{:X}", self.r_pc);
        println!("REG SP\t{:x}", self.r_sp);
    }
}

impl Dump for Mem {
    fn print_dump(&mut self) {
        println!("\nMEM DUMP");
        println!("PROM:");
        let offset: usize = self.prom.len()/2;
        for i in 0..(self.prom.len()/2) {
            if self.prom[i] == 0 {
                if self.prom[i+offset] == 0 {
                    println!("\t{ad_1:>0width$}\t{data:>0wi$}\t\t{ad_2:>0width$}\t{data:>0wi$}",
                            ad_1=i, width=3, data=self.prom[i], wi=8, ad_2=i+offset);
                } else {
                    println!("\t{ad_1:>0width$}\t{data:>0wi$}\t\t{ad_2:>0width$}\t{data_1:b}",
                            ad_1=i, width=3, data=self.prom[i], wi=8, ad_2=i+offset, data_1=self.prom[i+offset]);
                }
            } else {
                if self.prom[i+offset] == 0 {
                    println!("\t{ad_1:>0width$}\t{data:b}\t\t{ad_2:>0width$}\t{data_1:>0wi$}",
                            ad_1=i, width=3, data=self.prom[i], wi=8, ad_2=i+offset, data_1=self.prom[i+offset]);
                } else {
                    println!("\t{ad_1:>0width$}\t{data_1:b}\t\t{ad_2:>0width$}\t{data_2:b}",
                            ad_1=i, width=3, data_1=self.prom[i], ad_2=i+offset, data_2=self.prom[i+offset]);
                }
            }
        }
        println!("DATA:");
        for i in 0..self.data.len() {
            if self.data[i] == 0 {
                println!("\t{ad:>0width$}\t{data:>0wi$}", ad=i, width=3, data=self.data[i], wi=8);
            } else {
                println!("\t{ad:>0width$}\t{data:b}", ad=i, width=3, data=self.data[i]);
            };
        }
    }
}

/*
fn print_dump<T: Default>() {

}*/

fn main() {
    let instructions: Instruction = Instruction::new();

    let mut cpu: Cpu = Cpu {r_pc: 0, r_sp: 0, r_a: 0, r_b: 0, r_c: 0, r_d: 0, r_e: 0, r_h: 0, r_l: 0, 
                            f_c: false, f_z: false, f_s: false, f_p: false};

    let mut mem: Mem = Mem {prom: [0; MAX_PROM], data: [0; MAX_DATA]};
    
    cpu.reset(&mut mem);
    // for test ROM
    mem.load_instr();
    mem.print_dump();
    cpu.print_dump();
    //cpu.print_reg();
    println!();
    //execute commands
    println!("Instructions:");
    println!("Mnem\tCycle\tBytes\tType");
    for _i in 0..MAX_PROM {
        cpu.execute(&mut mem, &instructions);
    }
    println!("\nExecuting finished!\n");
    cpu.print_dump();
    println!();
}
