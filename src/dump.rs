use super::cpu::Cpu;
use super::mem::Mem;


pub trait Dump {
    fn print_dump(&mut self);
}

impl Dump for Cpu {
    fn print_dump(&mut self) {
        println!("\nCPU DUMP");
        println!("REG A\t{:X}", self.get_r_a());
        println!("REG B\t{:X}", self.get_r_b());
        println!("REG C\t{:X}", self.get_r_c());
        println!("REG D\t{:X}", self.get_r_d());
        println!("REG E\t{:X}", self.get_r_e());
        println!("REG H\t{:X}", self.get_r_h());
        println!("REG L\t{}\n", self.get_r_l());
        println!("FLAG C\t{}", self.get_f_c());
        println!("FLAG Z\t{}", self.get_f_z());
        println!("FLAG S\t{}", self.get_f_s());
        println!("FLAG P\t{}\n", self.get_f_p());
        println!("REG PC\t{:X}", self.get_r_pc());
        println!("REG SP\t{:x}", self.get_r_sp());
    }
}

impl Dump for Mem {
    fn print_dump(&mut self) {
        println!("\nMEM DUMP");
        println!("PROM:");
        let offset: usize = self.get_length_prom()/2;
        for i in 0..(self.get_length_prom()/2) {
            if self.get_byte_prom(i) == 0 {
                if self.get_byte_prom(i+offset) == 0 {
                    println!("\t{ad_1:>0width$}\t{data:>0wi$}\t\t{ad_2:>0width$}\t{data:>0wi$}",
                            ad_1=i, width=3, data=self.get_byte_prom(i), wi=8, ad_2=i+offset);
                } else {
                    println!("\t{ad_1:>0width$}\t{data:>0wi$}\t\t{ad_2:>0width$}\t{data_1:b}",
                            ad_1=i, width=3, data=self.get_byte_prom(i), wi=8, ad_2=i+offset, data_1=self.get_byte_prom(i+offset));
                }
            } else {
                if self.get_byte_prom(i+offset) == 0 {
                    println!("\t{ad_1:>0width$}\t{data:b}\t\t{ad_2:>0width$}\t{data_1:>0wi$}",
                            ad_1=i, width=3, data=self.get_byte_prom(i), wi=8, ad_2=i+offset, data_1=self.get_byte_prom(i+offset));
                } else {
                    println!("\t{ad_1:>0width$}\t{data_1:b}\t\t{ad_2:>0width$}\t{data_2:b}",
                            ad_1=i, width=3, data_1=self.get_byte_prom(i), ad_2=i+offset, data_2=self.get_byte_prom(i+offset));
                }
            }
        }
        println!("DATA:");
        for i in 0..self.get_length_data() {
            if self.get_byte_data(i) == 0 {
                println!("\t{ad:>0width$}\t{data:>0wi$}", ad=i, width=3, data=self.get_byte_data(i), wi=8);
            } else {
                println!("\t{ad:>0width$}\t{data:b}", ad=i, width=3, data=self.get_byte_data(i));
            };
        }
    }
}