#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::collapsible_else_if)] // TRY TO FIX IT!!!

use std::{thread::sleep, time::Duration};

pub mod mcs;
pub mod cmp;

use mcs::cpu::Cpu;
use mcs::mem::Mem;
use mcs::dump::Dump;
use cmp::translator::Compile;


fn main() {
    let filename: String = "/home/zs/Projects/3-bit_SP/test_prog/multiply.asm".to_string();
    
    let mut cpu: Cpu = Cpu::new();
    let mut mem: Mem = Mem::new();
    let mut translator: Compile = Compile::new(filename);
    
    cpu.reset(&mut mem);
    // for test ROM
    mem.print_dump();
    cpu.print_dump();
    println!();
    //execute commands
    println!("Instructions:");
    println!("Mnem\tCycle\tBytes\tType\t\tPC");
    'main_loop: loop {
        if let Ok(res) = cpu.execute(&mut mem) {  // halt - returns true
            if res {
                println!("\nExecuting finished!");
                break 'main_loop
            }
            else {if cpu.get_r_pc() as usize == mem.get_length_prom() {println!("\nPROCESSOR WASN'T HALTED")}}
        }
        sleep(Duration::from_millis(10));
        //cpu.print_dump();
    }
    cpu.print_dump();
    //mem.print_dump();
    println!();
}
