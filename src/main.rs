const MAX_MEM: usize = 1024 * 64; 

struct CPU {
    // programme counter
    r_pc: u16,
    // stack pointer
    r_sp: u16,

    // accumulator
    r_a: u8,
    // 8-bit registers
    r_x: u8,
    r_y: u8,

    // status flags
    f_c: bool,
    f_z: bool,
    f_i: bool,
    f_d: bool,
    f_b: bool,
    f_v: bool,
    f_n: bool
}

impl CPU {
    fn reset(&mut self, mem: &mut MEM) -> () {
        self.r_pc = 0xFF_FC;
        self.r_sp = 0x01_00;
        self.f_d = false;
        mem.initialize();
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
}



fn main() {
    let mut cpu: CPU = CPU {r_pc: 0, r_sp: 0, r_a: 0, r_x: 0, r_y: 0,
                            f_c: false, f_z: false, f_i: false, f_d: false, f_b: false, f_v: false, f_n: false};

    let mut mem: MEM = MEM {data: [0; MAX_MEM]};
    cpu.reset(&mut mem);
}
