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
    fn reset(&mut self) -> () {
        self.r_pc = 0xFF_FC;
        self.r_sp = 0x01_00;
        self.f_d = false;
    }
}


fn main() {
    let mut cpu: CPU = CPU {r_pc: 0, r_sp: 0, r_a: 0, r_x: 0, r_y: 0,
                            f_c: false, f_z: false, f_i: false, f_d: false, f_b: false, f_v: false, f_n: false};
    cpu.reset();
}
