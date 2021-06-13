# 3-bit Stack Pointer

## What is 3-bit SP?

It's an emulator of INTeL processor 8008.</br>
And it has 3-bit length stack pointer, that's why there are such name.

## RoadMap

 - [X] Main implementations of ALU and Instruction Decoder
 - [X] Simple memory RAM/PROM
 - [X] Dumps of memory and processor
 - [X] Implementation of all Instructions (46/46)
 - [X] Real memory as in MCS-8
 - [X] Translator for assembly code
 - [ ] Normal usage
 - [ ] TUI / GUI

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
 