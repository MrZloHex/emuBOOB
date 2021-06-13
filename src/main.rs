#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::collapsible_else_if)] // TRY TO FIX IT!!!
#![allow(unused_imports)]

use std::{thread::sleep, time::Duration};
use clap::{load_yaml, App};

pub mod mcs;
pub mod cmp;

use mcs::cpu::Cpu;
use mcs::mem::Mem;
use mcs::dump::Dump;
use cmp::translator::Compile;


fn main() {
    let mut input_filename: String = String::new();
    let mut output_filename: String = String::new();

    let mut command = "help";
    let mut verbosity: bool = false;
    let mut build_f: bool = false;

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        if matches.is_present("input") {
            if let Some(in_f) = matches.value_of("input") {
                input_filename = in_f.to_string();
            }
        }
        if matches.is_present("output") {
            if let Some(out_f) = matches.value_of("output") {
                output_filename = out_f.to_string();
            };
        } else {
            output_filename = input_filename.clone().replace(".asm", "");
        }
        if matches.is_present("verbose") {
            verbosity = true;
        }
        command = "build";
    }
    if let Some(matches) = matches.subcommand_matches("run") {
        if matches.is_present("input") {
            if let Some(in_f) = matches.value_of("input") {
                input_filename = in_f.to_string();
            }
        }
        if matches.is_present("output") {
            if let Some(out_f) = matches.value_of("output") {
                output_filename = out_f.to_string();
            };
        } else {
            output_filename = input_filename.clone().replace(".asm", "");
        }
        if matches.is_present("build") {
            build_f = true;
        }
        if matches.is_present("verbose") {
            verbosity = true;
        }
        command = "run";
    }

    // INITIALIZING PART
    let mut cpu: Cpu = Cpu::new();
    let mut mem: Mem = Mem::new();
    let mut translator: Compile = Compile::new();
    
    if "build".eq(command) {
        build(&mut translator, verbosity, input_filename, output_filename);
    } else if "run".eq(command) {
        run(&mut cpu, &mut mem, &mut translator, verbosity, build_f, input_filename, output_filename);
    } else if "help".eq(command) {
        help();
    }
    /*
    // COMPILE PART
    let machine_code = translator.compile().unwrap();

    // PROGRAMMING PART
    mem.programme_insert(machine_code);
    
    // EXECUTE PART
    cpu.reset();
    // for test ROM
    //mem.print_dump(); 
    //cpu.print_dump();
    //println!();
    //execute commands
    //println!("Instructions:");
    //println!("Mnem\tCycle\tBytes\tType\t\tPC");
    'main_loop: loop {
        if let Ok(res) = cpu.execute(&mut mem) {  // halt - returns true
            if res {
                //println!("\nExecuting finished!");
                break 'main_loop
            }
            else {if cpu.get_r_pc() as usize == mem.get_length_prom() {/*println!("\nPROCESSOR WASN'T HALTED")*/}}
        }
        sleep(Duration::from_millis(10));
        /*let mut line = String::new();   //      MANUAL CYCLE
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        cpu.print_dump();*/
    }
    //cpu.print_dump();
    //mem.print_dump();*/
    //println!();
}

fn build(translator: &mut Compile, verbosity: bool, input_filename: String, output_filename: String) {
    translator.precompile(input_filename, output_filename);
    match translator.compile(verbosity) {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    };
}

fn run(cpu: &mut Cpu, mem: &mut Mem, translator: &mut Compile, verbosity: bool, build_f: bool, input_filename: String, output_filename: String) {
    if build_f {
        translator.precompile(input_filename, output_filename.clone());
        match translator.compile(verbosity.clone()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
        mem.programme_insert(output_filename);
    } else {
        mem.programme_insert(input_filename);
    }
    if verbosity {mem.print_dump(); cpu.print_dump()}
    println!("Instructions:");
    if verbosity {println!("Mnem\tCycle\tBytes\tType\t\tPC");}
    else {println!("Mnem\tPC")}
    'main_loop: loop {
        if let Ok(res) = cpu.execute(mem, verbosity.clone()) {  // halt - returns true
            if res {
                println!("\nExecuting finished!");
                break 'main_loop
            }
            else {if cpu.get_r_pc() as usize == mem.get_length_prom() {println!("\nPROCESSOR WASN'T HALTED")}}
        }
        sleep(Duration::from_millis(10));
        /*let mut line = String::new();   //      MANUAL CYCLE
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        cpu.print_dump();*/
    }
    if verbosity {mem.print_dump()}
    cpu.print_dump();

}

fn help() {

}
