#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::ptr_arg)]

#![allow(clippy::collapsible_else_if)] // TRY TO FIX IT!!!

mod intstructions;
use intstructions::Instruction;

mod mem;
use mem::Mem;

mod cpu;
use cpu::Cpu;

mod dump;
use dump::Dump;


fn main() {
    let instructions: Instruction = Instruction::new();
    let mut cpu: Cpu = Cpu::new();
    let mut mem: Mem = Mem::new();
    
    cpu.reset(&mut mem);
    // for test ROM
    mem.print_dump();
    cpu.print_dump();
    println!();
    //execute commands
    println!("Instructions:");
    println!("Mnem\tCycle\tBytes\tType");
    for _i in 0..mem.get_length_prom() {
        cpu.execute(&mut mem, &instructions);
    }
    println!("\nExecuting finished!\n");
    cpu.print_dump();
    println!();
}
