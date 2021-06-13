#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::collapsible_else_if)] // TRY TO FIX IT!!!
#![allow(unused_imports)]

use clap::{load_yaml, App};
use std::{thread::sleep, time::Duration};

pub mod cmp;
pub mod mcs;

use cmp::translator::Compile;
use mcs::cpu::Cpu;
use mcs::dump::Dump;
use mcs::mem::Mem;

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
            output_filename = input_filename.replace(".asm", "");
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
            output_filename = input_filename.replace(".asm", "");
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
    let mut cpu: Cpu = Cpu::default();
    let mut mem: Mem = Mem::default();
    let mut translator: Compile = Compile::default();

    if "build".eq(command) {
        build(&mut translator, verbosity, input_filename, output_filename);
    } else if "run".eq(command) {
        run(
            &mut cpu,
            &mut mem,
            &mut translator,
            verbosity,
            build_f,
            input_filename,
            output_filename,
        );
    } else if "help".eq(command) {
        help();
    }

    println!();
}

fn build(
    translator: &mut Compile,
    verbosity: bool,
    input_filename: String,
    output_filename: String,
) {
    translator.precompile(input_filename, output_filename);
    match translator.compile(verbosity) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };
}

fn run(
    cpu: &mut Cpu,
    mem: &mut Mem,
    translator: &mut Compile,
    verbosity: bool,
    build_f: bool,
    input_filename: String,
    output_filename: String,
) {
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
    if verbosity {
        mem.print_dump();
        cpu.print_dump()
    }
    println!("Instructions:");
    if verbosity {
        println!("Mnem\tCycle\tBytes\tType\t\tPC");
    } else {
        println!("Mnem\tPC")
    }
    'main_loop: loop {
        if let Ok(res) = cpu.execute(mem, verbosity.clone()) {
            // halt - returns true
            if res {
                println!("\nExecuting finished!");
                break 'main_loop;
            } else {
                if cpu.get_r_pc() as usize == mem.get_length_prom() {
                    println!("\nPROCESSOR WASN'T HALTED")
                }
            }
        }
        sleep(Duration::from_millis(10));
        /*let mut line = String::new();   //      MANUAL CYCLE
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        cpu.print_dump();*/
    }
    if verbosity {
        mem.print_dump()
    }
    cpu.print_dump();
}

fn help() {
    println!(
        "sp_3-bit-run 1.8.6
    MrZloHex <zlo.alex.it@gmail.com>
    Execute binary file
    
    USAGE:
        sp_3-bit run [FLAGS] [OPTIONS] --input <INPUT_FILE>
    
    FLAGS:
        -b, --build      Before run program build input file
        -h, --help       Prints help information
        -V, --version    Prints version information
        -v, --verbose    Displaying dumps and executing instructions
    
    OPTIONS:
        -i, --input <INPUT_FILE>      Input filename with binary/assembly code
        -o, --output <OUTPUT_FILE>    Name of filename if is set flag build
    "
    )
}
