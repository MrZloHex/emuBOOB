use super::cpu::Cpu;
use super::mem::Mem;

// Generic trait for CPU and MEM
pub trait Dump {
    fn print_dump(&mut self);
    fn print_dunp(&mut self);
}

// Display dump of CPU
impl Dump for Cpu {
    fn print_dump(&mut self) {
        println!("\nCPU DUMP");
        println!("REG A\t{:>0width$X}", self.get_r_a(), width = 2);
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

    fn print_dunp(&mut self) {}
}

// Display dump of MEM
impl Dump for Mem {
    fn print_dunp(&mut self) {
        println!("\nMEM DUMP");
        println!("PROM:");
        let offset: usize = self.get_length_prom() / 8;
        for i in 0..(self.get_length_prom() / 8) {
            println!(
                "\t{ad_1:>0width$x}\t{data_1:>0wi$X}\t\t{ad_2:>0width$X}\t{data_2:>0wi$X}\
                \t\t{ad_3:>0width$x}\t{data_3:>0wi$X}\t\t{ad_4:>0width$x}\t{data_4:>0wi$X}\
                \t\t{ad_5:>0width$x}\t{data_5:>0wi$X}\t\t{ad_6:>0width$x}\t{data_6:>0wi$X}\
                \t\t{ad_7:>0width$x}\t{data_7:>0wi$X}\t\t{ad_8:>0width$x}\t{data_8:>0wi$X}",
                ad_1 = i,
                width = 3,
                data_1 = self.get_byte_prom(i),
                ad_2 = i + offset,
                data_2 = self.get_byte_prom(i + offset),
                wi = 2,
                ad_3 = i + 2 * offset,
                data_3 = self.get_byte_prom(i + 2 * offset),
                ad_4 = i + 3 * offset,
                data_4 = self.get_byte_prom(i + 3 * offset),
                ad_5 = i + 4 * offset,
                data_5 = self.get_byte_prom(i + 4 * offset),
                ad_6 = i + 5 * offset,
                data_6 = self.get_byte_prom(i + 5 * offset),
                ad_7 = i + 6 * offset,
                data_7 = self.get_byte_prom(i + 6 * offset),
                ad_8 = i + 7 * offset,
                data_8 = self.get_byte_prom(i + 7 * offset),
            );
        }
        println!("\n\nDATA:");
        let offset: usize = self.get_length_data() / 8;
        for i in 0..(self.get_length_data() / 8) {
            println!(
                "\t{ad_1:>0width$x}\t{data_1:>0wi$X}\t\t{ad_2:>0width$X}\t{data_2:>0wi$X}\
                \t\t{ad_3:>0width$x}\t{data_3:>0wi$X}\t\t{ad_4:>0width$x}\t{data_4:>0wi$X}\
                \t\t{ad_5:>0width$x}\t{data_5:>0wi$X}\t\t{ad_6:>0width$x}\t{data_6:>0wi$X}\
                \t\t{ad_7:>0width$x}\t{data_7:>0wi$X}\t\t{ad_8:>0width$x}\t{data_8:>0wi$X}",
                ad_1 = i,
                width = 3,
                data_1 = self.get_byte_data(i),
                ad_2 = i + offset,
                data_2 = self.get_byte_data(i + offset),
                wi = 2,
                ad_3 = i + 2 * offset,
                data_3 = self.get_byte_data(i + 2 * offset),
                ad_4 = i + 3 * offset,
                data_4 = self.get_byte_data(i + 3 * offset),
                ad_5 = i + 4 * offset,
                data_5 = self.get_byte_data(i + 4 * offset),
                ad_6 = i + 5 * offset,
                data_6 = self.get_byte_data(i + 5 * offset),
                ad_7 = i + 6 * offset,
                data_7 = self.get_byte_data(i + 6 * offset),
                ad_8 = i + 7 * offset,
                data_8 = self.get_byte_data(i + 7 * offset),
            );
        }
    }

    fn print_dump(&mut self) {
        println!("\nMem dump:");
        println!("PROM:\t\t\t\t\t\t\t\t\t\t\t\t\t\t  RAM:");
        let offset: usize = self.get_length_data() / 32;
        let ad_of: usize = 32;
        for i in 0..(self.get_length_prom() / 32) {
            println!(
                "\t{address:>0wi_a$x}:\t{d1:>0w$X} {d2:>0w$X} {d3:>0w$X} {d4:>0w$X} \
                {d5:>0w$X} {d6:>0w$X} {d7:>0w$X} {d8:>0w$X} \
                {d9:>0w$X} {d10:>0w$X} {d11:>0w$X} {d12:>0w$X} \
                {d13:>0w$X} {d14:>0w$X} {d15:>0w$X} {d16:>0w$X}   \
                {d17:>0w$X} {d18:>0w$X} {d19:>0w$X} {d20:>0w$X} \
                {d21:>0w$X} {d22:>0w$X} {d23:>0w$X} {d24:>0w$X} \
                {d25:>0w$X} {d26:>0w$X} {d27:>0w$X} {d28:>0w$X} \
                {d29:>0w$X} {d30:>0w$X} {d31:>0w$X} {d32:>0w$X}",
                address=i*ad_of,
                wi_a=3,
                w=2,
                d1=self.get_byte_prom(i*offset),
                d2=self.get_byte_prom(i*offset+1),
                d3=self.get_byte_prom(i*offset+2),
                d4=self.get_byte_prom(i*offset+3),
                d5=self.get_byte_prom(i*offset+4),
                d6=self.get_byte_prom(i*offset+5),
                d7=self.get_byte_prom(i*offset+6),
                d8=self.get_byte_prom(i*offset+7),
                d9=self.get_byte_prom(i*offset+8),
                d10=self.get_byte_prom(i*offset+9),
                d11=self.get_byte_prom(i*offset+10),
                d12=self.get_byte_prom(i*offset+11),
                d13=self.get_byte_prom(i*offset+12),
                d14=self.get_byte_prom(i*offset+13),
                d15=self.get_byte_prom(i*offset+14),
                d16=self.get_byte_prom(i*offset+15),

                d17=self.get_byte_prom(i*offset+16),
                d18=self.get_byte_prom(i*offset+17),
                d19=self.get_byte_prom(i*offset+18),
                d20=self.get_byte_prom(i*offset+19),
                d21=self.get_byte_prom(i*offset+20),
                d22=self.get_byte_prom(i*offset+21),
                d23=self.get_byte_prom(i*offset+22),
                d24=self.get_byte_prom(i*offset+23),
                d25=self.get_byte_prom(i*offset+24),
                d26=self.get_byte_prom(i*offset+25),
                d27=self.get_byte_prom(i*offset+26),
                d28=self.get_byte_prom(i*offset+27),
                d29=self.get_byte_prom(i*offset+28),
                d30=self.get_byte_prom(i*offset+29),
                d31=self.get_byte_prom(i*offset+30),
                d32=self.get_byte_prom(i*offset+31),
            )
        }
    }
}
