use super::cpu::Cpu;
use super::mem::Mem;

pub trait Dump {
    fn print_dump(&mut self);
}

impl Dump for Cpu {
    fn print_dump(&mut self) {
        println!("\nCPU DUMP");
        println!("REG A\t{:>0width$b}", self.get_r_a(), width = 8);
        println!("REG B\t{:>0width$X}", self.get_r_b(), width = 2);
        println!("REG C\t{:>0width$X}", self.get_r_c(), width = 2);
        println!("REG D\t{:>0width$X}", self.get_r_d(), width = 2);
        println!("REG E\t{:>0width$X}", self.get_r_e(), width = 2);
        println!("REG H\t{:>0width$X}", self.get_r_h(), width = 2);
        println!("REG L\t{:>0width$X}\n", self.get_r_l(), width = 2);
        println!("FLAG C\t{}", self.get_f_c());
        println!("FLAG Z\t{}", self.get_f_z());
        println!("FLAG S\t{}", self.get_f_s());
        println!("FLAG P\t{}\n", self.get_f_p());
        println!("REG PC\t{:>0width$X}", self.get_r_pc(), width = 2);
        println!("REG SP\t{:>0width$X}", self.get_r_sp(), width = 2);
        println!("Stack:");
        for i in 0..7 {
            println!(
                "\t{ad_1:X}\t{data_stack:>0wi$b}",
                ad_1 = i,
                data_stack = self.get_byte_stack(i as usize),
                wi = 8
            );
        }
    }
}

impl Dump for Mem {
    fn print_dump(&mut self) {
        println!("\nMEM DUMP");
        println!("PROM:");
        let offset: usize = self.get_length_prom() / 2;
        for i in 0..(self.get_length_prom() / 2) {
            println!(
                "\t{ad_1:>0width$x}\t{data_1:>0wi$b}\t\t{ad_2:>0width$x}\t{data_2:>0wi$b}",
                ad_1 = i,
                width = 2,
                data_1 = self.get_byte_prom(i),
                ad_2 = i + offset,
                data_2 = self.get_byte_prom(i + offset),
                wi = 8
            );
        }
        println!("DATA:");
        for i in 0..self.get_length_data() {
            println!(
                "\t{ad_1:>0width$x}\t{data_1:>0wi$b}",
                ad_1 = i,
                width = 2,
                data_1 = self.get_byte_data(i),
                wi = 8
            );
        }
    }
}
