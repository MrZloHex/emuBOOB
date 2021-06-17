# emuBOOB

## What is emuBOOB?

It's an emulator of INTeL processor **i8008**.

## Table of contents

 * [Roadmap](#roadmap)
 * [Description](#description)
    * [Basement](#basement)
        * [CPU](#cpu)
            * [Registers](#registers)
    * [Emulator](#emulator)
    * [Compiler](#compiler)
 * [Deployment](#deployment)
    * [Installation](#installation)
    * [Uninstallation](#uninstallation)
 * [Usage](#usage)
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
| H    | 8  bit | GPR                  |
| L    | 8  bit | GPR                  |
| PC   | 14 bit | Program Counter<sup>[2](#PC)</sup> |
| SP   | 3  bit | Stack Pointer        |

### Emulator




### Compiler

## Deployment

### Installation

### Uninstallation

## Usage

## References

 - [Wiki with general info about](https://en.wikipedia.org/wiki/Intel_8008)
 - [Wiki with internal system of i8008](https://en.wikichip.org/wiki/intel/mcs-8/isa)
 - [INTeL's original reference for i8008](https://github.com/MrZloHex/3-bit_SP/blob/master/manuls/8008-Intel.pdf)
 - [MCS-8 datasheet](https://github.com/MrZloHex/3-bit_SP/blob/master/manuls/MCS-8_User_Manual_(Rev_2)_(Nov_1972).pdf)

## ASM Rules

**THEY ARE CHANGING OFTEN**

 - No comments
 - No free lines
 - First line is with `CPU` command and value `8008` (see examples)
 - All instructions should be shifted on one tab of 4 spaces
 - Labels should be whitout shift
 - After label should be colon `:`
 - Label should be before labaled block on another line
 - All values/labels which is needful for the instruction should be on same line and separated with whitespace from instruction
 - Values should be in decimal form
 - Calling or jumping to a label, the label name should starts from ampersand (`&`)
 - You can write values in different bases, for specify base you should write after value (`'d'`, `'h'`, `'o'`, `'b'`) for decimal, hexadecimal, octal and binary accordingly
 
## Footnotes

<a name="GPR">1</a>: GPR - General Purpose Register. This registers can be used for contain any data</br>
<a name="PC">2</a>: PC - Program Counter (Modern: IS - Instruction Pointer). This register is used to point address of next opcode in memory