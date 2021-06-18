# emuBOOB

## What is emuBOOB?

It's an emulator of INTeL processor **i8008**.

## Table of contents

 * [Roadmap](#roadmap)
 * [Description](#description)
 	* [Basement](#basement)
 		* [CPU](#cpu)
 			* [Registers](#registers)
 			* [Flags](#flags)
 			* [ALU](#alu)
 			* [Stack](#stack)
 			* [IS](#instruction-set)
 		* [Memory](#memory)
 			* [PROM](#prom)
 			* [RAM](#ram)
 	* [Emulator](#emulator)
 	* [Compiler](#compiler)
 		* [Specification](#compiler-specification)
 		* [Rules](#assembler-syntax-rules)
 * [Deployment](#deployment)
 	* [Rust](#rust-installation)
 	* [Installation](#installation)
 	* [Uninstallation](#uninstallation)
 * [Usage](#usage)
	* [Building](#building)
	* [Running](#running)
 * [References and manuals](#references)


## RoadMap

 - [X] Main implementations of ALU and Instruction Decoder
 - [X] Simple memory RAM/PROM
 - [X] Dumps of memory and processor
 - [X] Implementation of all Instructions (46/46)
 - [X] Real memory as in MCS-8
 - [X] Translator for assembly code
 - [X] Normal usage
 - [ ] TUI / GUI

## Description

### Basement

This project is based on [*MCS-8*](https://en.wikichip.org/wiki/intel/mcs-8).</br>
Parametres of MCS-8:
 - **CPU**:     i8008-1
 - **Memory**:
    - *PROM*:  2 KB
    - *RAM*:   1 KB

#### CPU

**i8008-1** is impoved version of standart i8008 with decreased cycle time (from 20 µs to 12.5 µs).</br>
*Futher in the text i8008-1 will be named just **8008***

##### Registers

| Name | Length | Description          |
|------|--------|----------------------|
| A    | 8  bit | Accumulator          |
| B    | 8  bit | GPR<sup>[1](#GPR)</sup>           |
| C    | 8  bit | GPR                  |
| D    | 8  bit | GPR                  |
| E    | 8  bit | GPR                  |
| H    | 8  bit | GPR/High byte of address for MI<sup>[2](#MI)</sup>                  |
| L    | 8  bit | GPR/Low byte of address for MI                  |
| PC   | 14 bit | Program Counter<sup>[3](#PC)</sup> |
| SP   | 3  bit | Stack Pointer        |

##### Flags

| Name | Description |
|------|-------------|
| C    | Carry       |
| Z    | Zero        |
| S    | Sign        |
| P    | Parity      |

##### ALU

ALU can do arithmetic and logical operations:
`ADD, ADD with carry, SUBTRACT, SUBSTRACT with borrow, AND, EXCLUSIVE OR, OR, COMPARE, INCREMENT, DECREMENT`

All ALU operations have influence on flags' flip-flops.</br>
But `INCREMENT` and `DECREMENT` don't touch `C` (carry) flag.

##### Stack

Stack in 8008 is located in proccessor. Subsequently, it has only **7 levels** of deepnest.</br>
*SP* is 3 bit length and you can't change its value.

If you overflow stack level it would erase first levels.
Try to prevent this overflow!

##### Instruction Set

![InstructionSet](https://github.com/MrZloHex/emuBOOB/blob/master/manuls/instuctions.png)

*Took from page 8-9 of 8008 [manual](https://github.com/MrZloHex/emuBOOB/blob/master/manuls/8008-Intel.pdf)*

#### Memory

In *MCS-8* memory block is separated into 2 parts.

##### PROM

Capacity of **PROM** is 2 KB.</br>
One *word* length is 1 Byte.

**PROM** is used to contain programme's code.</br>
You CAN'T write in PROM in runtime. For this use [RAM](#ram)

##### RAM

Capacity of **RAM** is 1 KB.</br>
One *word* length is 1 Byte

**RAM** is used to save data in runtime.</br>
After finishing executing programme all data from **RAM** would be erased.

### Emulator

Emulator is very close to real structure and ecosystem of MCS-8.</br>
Emualator has pretty dump output of *CPU* and *Memory*.

See more information:
 - `$ emuBOOB run help`
 - `$ man emuBOOB`

### Compiler

#### Compiler Specification

#### Assembler Syntax Rules

 - No comments
 - No free lines
 - First line is with `CPU` command and value `8008` (see [examples](https://github.com/MrZloHex/emuBOOB/blob/master/examples/multiply.asm))
 - All instructions should be shifted on one tab of 4 spaces
 - Labels should be without shift
 - After label should be colon `:`
 - Label should be before labaled block on another line
 - All values/labels which is needful for the instruction should be on same line and separated with whitespace from instruction
 - Values should be in decimal form
 - Calling or jumping to a label, the label name should starts from ampersand (`&`)
 - You can write values in different bases, for specify base you should write after value (`'d'`, `'h'`, `'o'`, `'b'`) for decimal, hexadecimal, octal and binary accordingly

## Deployment

**NOTE**</br>
YOU SHOULD HAVE RUST AND CARGO TO INSTALL THIS EMULATOR

### Rust Installation

Try run: `$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

And see official [guide](https://www.rust-lang.org/tools/install).

For `cargo` try this:
 - Ubuntu / Debian `$ sudo apt-get install cargo`
 - Arch `$ sudo pacman -S cargo`

Or see official [guide](https://github.com/rust-lang/cargo)

### Installation

1. Download or clone this repo:
	- Git: `$ git clone https://github.com/MrZloHex/emuBOOB.git`
2. Change working directory to *lscc*:
	- `$ cd emuBOOB`
3. Run *installation* script:
	- `$ ./deployment.sh -i`
	- **NOTE** You need to have **sudo** access.

### Uninstallation

1. Change working directory to *lscc*:
	- `$ cd emuBOOB`
2. Run *uninstallation* script:
	- `$ ./deployment.sh -u`
3. Go out from directory:
	- `$ cd ..`

## Usage

### Building

emuBOOB's compiler supporting only `.asm` files with very strcit rules.</br>
For build your assembly code for i8008 run this:</br>
	`$ emuBOOB build --input="<PATH TO YOUR CODE>"`</br>

This would make bianry file in same directory, which after you can execute in emulator

Options for `build` subcommand:
 - **-i**,  **--input**:</br>
	This is obligatory option, where you specify the path to the source file.</br>
	E. g. `$ emuBOOB build --input="project8008/src/main.asm"`
 - **-o**,  **--output**:</br>
	With this option you can specify path and name of output bianry file.</br>
	E. g. `$ emuBOOB build --input="project8008/src/main.asm" --output="target/my_app"`



Flags for `build` subcommand:
 - **-v**, **--verbose**:</br>
	Will be displayed your assembly code and after opcodes for i8008.
 - **-V**, **--version**:</br>
	You will see version of compiler for 8008.
 - **-h**, **--help**:</br>
	Display help information about `build`.

### Running

Emulator can execute binary made obly by emuBOOB's compiler.</br>
To run compiled binary on i8008 run:</br>
	`$ emuBOOB run --input="<PATH TO BINARY>"`

Options for `run` subcommand:
 - **-i**, **--input**:</br>
	This is obligatory option, where you specify the path to the binary file or source code (see `-b` flag)</br>
	E. g. `$ emuBOOB run --input="target/my_app"`
 - **-o**, **--ouput**:</br>
	This option is usefull only with `-b` flag, when you building before executing.</br>
	E. g. `$ emuBOOB run --input="project/src/main.asm" --output="target/my_app" --build`

## References

 - [Wiki with general info about](https://en.wikipedia.org/wiki/Intel_8008)
 - [Wiki with internal system of i8008](https://en.wikichip.org/wiki/intel/mcs-8/isa)
 - [INTeL's original reference for i8008](https://github.com/MrZloHex/emuBOOB/blob/master/manuls/8008-Intel.pdf)
 - [MCS-8 datasheet](https://github.com/MrZloHex/emuBOOB/blob/master/manuls/MCS-8_User_Manual_(Rev_2)_(Nov_1972).pdf)

## Footnotes

<a name="GPR">1</a>: **GPR**- General Purpose Register. This registers can be used for contain any data</br>
<a name="MI">2</a>: **MI** - Memory Instruction. This instruction addressing to RAM for write or read</br>
<a name="PC">3</a>: **PC** - Program Counter (Modern: **IP** - Instruction Pointer). This register is used to point address of next opcode in memory</br>
