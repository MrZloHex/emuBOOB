
struct CPU {
    // programme counter
    PC: u16,
    // stack pointer
    SP: u16,

    // accumulator
    A: u8,
    // 8-bit registers
    X: u8,
    Y: u8,

    // status flags
    C: bool,
    Z: bool,
    I: bool,
    D: bool,
    B: bool,
    V: bool,
    N: bool
}


fn main() {
    let _cpu: CPU = CPU {PC: 0, SP: 0, A: 0, X: 0, Y: 0, C: false, Z: false, I: false, D: false, B: false, V: false, N: false};
}
