#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::collapsible_else_if)] // TRY TO FIX IT!!!

// Command Line Argument Parser https://github.com/clap-rs/clap
use clap::{load_yaml, App};
// Sleep instructions for delay
use std::{thread::sleep, time::Duration};
use std::io::Read;

// Files for build assembly code
pub mod cmp;
use cmp::translator::Compile;

// Files for emulate binary file on i8008
pub mod mcs;
use mcs::cpu::Cpu;
use mcs::dump::Dump;
use mcs::mem::Mem;

fn main() {
    

    // Allocating memory for files' names
    let mut input_filename: String = String::new();
    let mut output_filename: String = String::new();

    // CLI options and arguments
    let command: &str;
    let mut verbosity: bool = false;
    let mut build_f: bool = false;
    let mut manual: bool = false;

    // Config file of parsing CLI options
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    // Checking for parsed CLI options
    // BUILD
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
            output_filename = input_filename.replace(".asm", "");
        }
        if matches.is_present("verbose") {
            verbosity = true;
        }
        command = "build";
    }
    // RUN
    else if let Some(matches) = matches.subcommand_matches("run") {
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
            output_filename = input_filename.replace(".asm", "");
        }
        if matches.is_present("build") {
            build_f = true;
        }
        if matches.is_present("verbose") {
            verbosity = true;
        }
        if matches.is_present("manual") {
            manual = true;
        }
        command = "run";
    } else {
        command = ""
    }

    // initializing All needful classes
    let mut cpu: Cpu = Cpu::default();
    let mut mem: Mem = Mem::default();
    let mut translator: Compile = Compile::default();

    // Distribution by instruction
    if "build".eq(command) {
        build(&mut translator, verbosity, input_filename, output_filename);
    } else if "run".eq(command) {
        run(
            &mut cpu,
            &mut mem,
            &mut translator,
            verbosity,
            build_f,
            manual,
            input_filename,
            output_filename,
        )
    }

    println!();
}

// BUILD SUBCOMMAND
fn build(
    translator: &mut Compile,
    verbosity: bool,
    input_filename: String,
    output_filename: String,
) {
    // Obtaining source file
    translator.precompile(input_filename, output_filename);
    // Compile this file into binary
    match translator.compile(verbosity) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };
}

// RUN SUBCOMMAND
fn run(
    cpu: &mut Cpu,
    mem: &mut Mem,
    translator: &mut Compile,
    verbosity: bool,
    build_f: bool,
    manual: bool,
    input_filename: String,
    output_filename: String,
) {
    // Initializing cpu
    cpu.reset();
    // Checking for pre-build flag and upload opcodes into PROM
    if build_f {
        translator.precompile(input_filename, output_filename.clone());
        match translator.compile(verbosity.clone()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
        mem.programme_insert(output_filename);
    } else {
        mem.programme_insert(input_filename);
    }
    // Check for verbose flag
    if verbosity {
        mem.print_dump();
        cpu.print_dump()
    }
    // Display header of instruction table
    println!("Instructions:");
    if verbosity {
        println!("Mnem\tCycle\tBytes\tType\t\tPC");
    } else {
        println!("Mnem\tPC")
    }

    // Main loop for executing instructions by i8008
    'main_loop: loop {
        // Run ONE instruction
        let res = cpu.execute(mem, verbosity.clone());
        // halt - returns true
        // Any program should stop with HALT instruction
        if res {
            println!("\nExecuting finished!");
            break 'main_loop;
        } else {
            if cpu.get_r_pc() as usize == mem.get_length_prom() {
                println!("\nPROCESSOR WASN'T HALTED")
            }
        }
        if manual {
            // MANUAL CYCLE
            // FLOW CONTROL
            let mut input = String::new();
            let _string = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
            let bytes = input.bytes().nth(0).expect("no byte read");
            if bytes == 113 {break 'main_loop;}

            if verbosity {cpu.print_dump();}
        }
        else {
            // Frequency 200 kHz
            sleep(Duration::from_micros(5));
        }
    }
    if verbosity {
        mem.print_dump()
    }
    cpu.print_dump();
}
