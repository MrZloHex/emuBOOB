use std::collections::HashMap;

pub struct Instruction {
    instr_set: HashMap<u8, String>,
    instr_time: [Vec<String>; 3],
    instr_length: [Vec<String>; 3],
    instr_type: [Vec<String>; 4]
}

impl Instruction {
    pub fn new() -> Instruction {
        let instrucitions: Instruction = Instruction{
            instr_set: Instruction::opcodes(),
            instr_time: Instruction::time_instr(),
            instr_length: Instruction::length_instr(),
            instr_type: Instruction::type_instr()
        };
        instrucitions
    }

    fn opcodes() -> HashMap<u8, String> {
        let mut instr_set: HashMap<u8, String> = HashMap::new();

        // INDEX REGISTER INSTRUCTIONS
        {
            // LOAD REG -> REG INSTRUCTION
            {
                // load reg2 -> reg1
                // reg1 == reg2 => NOP
                let nop: [u8; 7] = [192, 201, 210, 219, 228, 237, 246];
                // reg1 != reg2 => Lrr
                for reg1 in (192..241).step_by(8) {
                    for reg2 in 0..7 {
                        let op = reg1 + reg2;
                        if nop.contains(&op) {
                            instr_set.insert(op, "NOP".to_string());
                        } else {
                            if reg1 == 192 {
                                match reg2 {
                                    1 => instr_set.insert(op, "LAB".to_string()),
                                    2 => instr_set.insert(op, "LAC".to_string()),
                                    3 => instr_set.insert(op, "LAD".to_string()),
                                    4 => instr_set.insert(op, "LAE".to_string()),
                                    5 => instr_set.insert(op, "LAH".to_string()),
                                    6 => instr_set.insert(op, "LAL".to_string()),
                                    _ => continue
                                };
                            } else if reg1 == 200 {
                                match reg2 {
                                    0 => instr_set.insert(op, "LBA".to_string()),
                                    2 => instr_set.insert(op, "LBC".to_string()),
                                    3 => instr_set.insert(op, "LBD".to_string()),
                                    4 => instr_set.insert(op, "LBE".to_string()),
                                    5 => instr_set.insert(op, "LBH".to_string()),
                                    6 => instr_set.insert(op, "LBL".to_string()),
                                    _ => continue
                                };
                            } else if reg1 == 208 {
                                match reg2 {
                                    0 => instr_set.insert(op, "LCA".to_string()),
                                    1 => instr_set.insert(op, "LCB".to_string()),
                                    3 => instr_set.insert(op, "LCD".to_string()),
                                    4 => instr_set.insert(op, "LCE".to_string()),
                                    5 => instr_set.insert(op, "LCH".to_string()),
                                    6 => instr_set.insert(op, "LCL".to_string()),
                                    _ => continue
                                };
                            } else if reg1 == 216 {
                                match reg2 {
                                    0 => instr_set.insert(op, "LDA".to_string()),
                                    1 => instr_set.insert(op, "LDB".to_string()),
                                    2 => instr_set.insert(op, "LDC".to_string()),
                                    4 => instr_set.insert(op, "LDE".to_string()),
                                    5 => instr_set.insert(op, "LDH".to_string()),
                                    6 => instr_set.insert(op, "LDL".to_string()),
                                    _ => continue
                                };
                            } else if reg1 == 224 {
                                match reg2 {
                                    0 => instr_set.insert(op, "LEA".to_string()),
                                    1 => instr_set.insert(op, "LEB".to_string()),
                                    2 => instr_set.insert(op, "LEC".to_string()),
                                    3 => instr_set.insert(op, "LED".to_string()),
                                    5 => instr_set.insert(op, "LEH".to_string()),
                                    6 => instr_set.insert(op, "LEL".to_string()),
                                    _ => continue
                                };
                            } else if reg1 == 232 {
                                match reg2 {
                                    0 => instr_set.insert(op, "LHA".to_string()),
                                    1 => instr_set.insert(op, "LHB".to_string()),
                                    2 => instr_set.insert(op, "LHC".to_string()),
                                    3 => instr_set.insert(op, "LHD".to_string()),
                                    4 => instr_set.insert(op, "LHE".to_string()),
                                    6 => instr_set.insert(op, "LHL".to_string()),
                                    _ => continue
                                };
                            } else if reg1 == 240 {
                                match reg2 {
                                    0 => instr_set.insert(op, "LLA".to_string()),
                                    1 => instr_set.insert(op, "LLB".to_string()),
                                    2 => instr_set.insert(op, "LLC".to_string()),
                                    3 => instr_set.insert(op, "LLD".to_string()),
                                    4 => instr_set.insert(op, "LLE".to_string()),
                                    5 => instr_set.insert(op, "LLH".to_string()),
                                    _ => continue
                                };
                            };
                        };
                    };
                };
            }
            // LOAD REG <- MEM
            {
                for reg in (199..249).step_by(8) {
                    match reg {
                        199 => instr_set.insert(reg, "LAM".to_string()),
                        207 => instr_set.insert(reg, "LBM".to_string()),
                        215 => instr_set.insert(reg, "LCM".to_string()),
                        223 => instr_set.insert(reg, "LDM".to_string()),
                        231 => instr_set.insert(reg, "LEM".to_string()),
                        239 => instr_set.insert(reg, "LHM".to_string()),
                        248 => instr_set.insert(reg, "LLM".to_string()),
                        _ => continue
                    };
                };
            };
            // LOAD MEM <- REG
            {
                for reg in 248..255{
                    match reg {
                        248 => instr_set.insert(reg, "LMA".to_string()),
                        249 => instr_set.insert(reg, "LMB".to_string()),
                        250 => instr_set.insert(reg, "LMC".to_string()),
                        251 => instr_set.insert(reg, "LMD".to_string()),
                        252 => instr_set.insert(reg, "LME".to_string()),
                        253 => instr_set.insert(reg, "LMH".to_string()),
                        254 => instr_set.insert(reg, "LML".to_string()),
                        _ => continue
                    };
                };
            };
            // LOAD REG <- DATA IMMEDIATE
            {
                for reg in (6..55).step_by(8) {
                    match reg {
                        6 => instr_set.insert(reg, "LAI".to_string()),
                        14 => instr_set.insert(reg, "LBI".to_string()),
                        22 => instr_set.insert(reg, "LCI".to_string()),
                        30 => instr_set.insert(reg, "LDI".to_string()),
                        38 => instr_set.insert(reg, "LEI".to_string()),
                        46 => instr_set.insert(reg, "LHI".to_string()),
                        54 => instr_set.insert(reg, "LLI".to_string()),
                        _ => continue
                    };
                };
            };
            // LOAD MEM <- DATA IMEDIATE
            {
                instr_set.insert(0b00_111_110, "LMI".to_string());
            };
            // INCREMENT / DECREMENT INSTRUCTION
            {
                for reg in (8..49).step_by(8) {
                    match reg {
                        8 => {
                            instr_set.insert(reg, "INB".to_string());
                            instr_set.insert(reg+1, "DCB".to_string());
                        },
                        16 => {
                            instr_set.insert(reg, "INC".to_string());
                            instr_set.insert(reg+1, "DCC".to_string());
                        },
                        24 => {
                            instr_set.insert(reg, "IND".to_string());
                            instr_set.insert(reg+1, "DCD".to_string());
                        },
                        32 => {
                            instr_set.insert(reg, "INE".to_string());
                            instr_set.insert(reg+1, "DCE".to_string());
                        },
                        40 => {
                            instr_set.insert(reg, "INH".to_string());
                            instr_set.insert(reg+1, "DCH".to_string());
                        },
                        48 => {
                            instr_set.insert(reg, "INL".to_string());
                            instr_set.insert(reg+1, "DCL".to_string());
                        },
                        _ => {}
                    };

                };
            };
        };
        // ACCUMULATOR INSTRUCTIONS
        {
            // ALU INDEX REGISTER INSTRUCTIONS
            {
                // ADD REG A <- REG A + reg
                for reg in 128..135 {
                    match reg {
                        128 | 134 => instr_set.insert(reg, "ADL".to_string()),
                        129 => instr_set.insert(reg, "ADB".to_string()),
                        130 => instr_set.insert(reg, "ADC".to_string()),
                        131 => instr_set.insert(reg, "ADD".to_string()),
                        132 => instr_set.insert(reg, "ADE".to_string()),
                        133 => instr_set.insert(reg, "ADH".to_string()),
                        _ => continue
                    };
                }
                // ADD REG A <- REG A + reg + CARRY
                for reg in 136..144 {
                    match reg {
                        136 | 142 => instr_set.insert(reg, "ACL".to_string()),
                        137 => instr_set.insert(reg, "ACB".to_string()),
                        138 => instr_set.insert(reg, "ACC".to_string()),
                        139 => instr_set.insert(reg, "ACD".to_string()),
                        140 => instr_set.insert(reg, "ACE".to_string()),
                        141 => instr_set.insert(reg, "ACH".to_string()),
                        _ => continue, 
                    };
                }
                // SUBSTRACT REG A <- REG A - reg
                for reg in 144..152 {
                    match reg {
                        144 | 150 => instr_set.insert(reg, "SUL".to_string()),
                        145 => instr_set.insert(reg, "SU".to_string()),
                        146 => instr_set.insert(reg, "SU".to_string()),
                        147 => instr_set.insert(reg, "SU".to_string()),
                        148 => instr_set.insert(reg, "SU".to_string()),
                        149 => instr_set.insert(reg, "SU".to_string()),
                        _ => continue,
                    };
                }
                // SUBSTRACT REG A <- REG A - reg - BORROW
                for reg in 152..160 {
                    match reg {
                        152 | 158 => instr_set.insert(reg, "SBL".to_string()),
                        153 => instr_set.insert(reg, "SBB".to_string()),
                        154 => instr_set.insert(reg, "SBC".to_string()),
                        155 => instr_set.insert(reg, "SBD".to_string()),
                        156 => instr_set.insert(reg, "SBE".to_string()),
                        157 => instr_set.insert(reg, "SBH".to_string()),
                        _ => continue,
                    };
                }
                // AND REG A <- REG A & reg
                for reg in 160..168 {
                    match reg {
                        160 | 166 => instr_set.insert(reg, "NDL".to_string()),
                        161 => instr_set.insert(reg, "NDB".to_string()),
                        162 => instr_set.insert(reg, "NDC".to_string()),
                        163 => instr_set.insert(reg, "NDD".to_string()),
                        164 => instr_set.insert(reg, "NDE".to_string()),
                        165 => instr_set.insert(reg, "NDH".to_string()),
                        _ => continue,
                    };
                }
                // XOR REG A <- REG A ^ reg
                for reg in 168..176 {
                    match reg {
                        168 | 174 => instr_set.insert(reg, "XRL".to_string()),
                        169 => instr_set.insert(reg, "XRB".to_string()),
                        170 => instr_set.insert(reg, "XRC".to_string()),
                        171 => instr_set.insert(reg, "XRD".to_string()),
                        172 => instr_set.insert(reg, "XRE".to_string()),
                        173 => instr_set.insert(reg, "XRH".to_string()),
                        _ => continue,
                    };
                }
                // OR REG A <- REG A | reg
                for reg in 176..184 {
                    match reg {
                        176 | 182 => instr_set.insert(reg, "ORL".to_string()),
                        177 => instr_set.insert(reg, "ORB".to_string()),
                        178 => instr_set.insert(reg, "ORC".to_string()),
                        179 => instr_set.insert(reg, "ORD".to_string()),
                        180 => instr_set.insert(reg, "ORE".to_string()),
                        181 => instr_set.insert(reg, "ORH".to_string()),
                        _ => continue,
                    };
                }
                // COMPARE REG A - reg
                for reg in 184..192 {
                    match reg {
                        184 | 190 => instr_set.insert(reg, "CPL".to_string()),
                        185 => instr_set.insert(reg, "CPB".to_string()),
                        186 => instr_set.insert(reg, "CPC".to_string()),
                        187 => instr_set.insert(reg, "CPD".to_string()),
                        188 => instr_set.insert(reg, "CPE".to_string()),
                        189 => instr_set.insert(reg, "CPH".to_string()),
                        _ => continue,
                    };
                }
            };
            // ALU OPERATIONS WITH MEMORY
            {
                // ADD
                instr_set.insert(0b10_000_111, "ADM".to_string());
                // ADD + CARRY
                instr_set.insert(0b10_001_111, "ACM".to_string());
                // SUBSTRACT
                instr_set.insert(0b10_010_111, "SUM".to_string());
                // SUBSTRACT WITH BORROW
                instr_set.insert(0b10_011_111, "SBM".to_string());
                // LOGICAL AND
                instr_set.insert(0b10_100_111, "NDM".to_string());
                // LOGICAL XOR
                instr_set.insert(0b10_101_111, "XRM".to_string());
                // LOGICAL ORI
                instr_set.insert(0b10_110_111, "ORM".to_string());
                // COMPARE
                instr_set.insert(0b10_111_111, "CPM".to_string());
            };
            // ALU IMMEDIATE INSTRUCTIONS
            {
                // ADD
                instr_set.insert(0b00_000_100, "ADI".to_string());
                // ADD + CARRY
                instr_set.insert(0b00_001_100, "ACI".to_string());
                // SUBSTRACT
                instr_set.insert(0b00_010_100, "SUI".to_string());
                // SUBSTRACT WITH BORROW
                instr_set.insert(0b00_011_100, "SBI".to_string());
                // LOGICAL AND
                instr_set.insert(0b00_100_100, "NDI".to_string());
                // LOGICAL XOR
                instr_set.insert(0b00_101_100, "XRI".to_string());
                // LOGICAL ORI
                instr_set.insert(0b00_110_100, "ORI".to_string());
                // COMPARE
                instr_set.insert(0b00_111_100, "CPI".to_string());
            };
        };
        // PROGRAM COUNTER AND STACK CONTROL INSTRUCTIONS
        {
            // JUMP INSTRUCTIONS
            {
                // JMP
                for x in (68..125).step_by(8) {
                    instr_set.insert(x, "JMP".to_string());
                };
                // JFc
                for flag in (64..89).step_by(8) {
                    match flag {
                        64 => instr_set.insert(flag, "JFC".to_string()),
                        72 => instr_set.insert(flag, "JFZ".to_string()),
                        80 => instr_set.insert(flag, "JFS".to_string()),
                        88 => instr_set.insert(flag, "JFP".to_string()),
                        _ => continue
                    };
                };
                // JTc
                for flag in (96..121).step_by(8) {
                    match flag {
                        96 => instr_set.insert(flag, "JTC".to_string()),
                        104 => instr_set.insert(flag, "JTZ".to_string()),
                        112 => instr_set.insert(flag, "JTS".to_string()),
                        120 => instr_set.insert(flag, "JTP".to_string()),
                        _ => continue
                    };
                };
            };
            // CALL INSTRUCTIONS
            {
                // CAL
                for x in (70..127).step_by(8) {
                    instr_set.insert(x, "CAL".to_string());
                }
                // CFc
                for flag in (66..91).step_by(8) {
                    match flag {
                        66 => instr_set.insert(flag, "CFC".to_string()),
                        74 => instr_set.insert(flag, "CFZ".to_string()),
                        82 => instr_set.insert(flag, "CFS".to_string()),
                        90 => instr_set.insert(flag, "CFP".to_string()),
                        _ => continue
                    };
                };
                // CTc
                for flag in (98..123).step_by(8) {
                    match flag {
                        98 => instr_set.insert(flag, "CTC".to_string()),
                        106 => instr_set.insert(flag, "CTZ".to_string()),
                        114 => instr_set.insert(flag, "CTS".to_string()),
                        122 => instr_set.insert(flag, "CTP".to_string()),
                        _ => continue
                    };
                };
            };
            // RETURN INSTRUCTIONS
            {
                // RET
                for x in (7..64).step_by(8) {
                    instr_set.insert(x, "RET".to_string());
                }
                // RFc
                for flag in (3..28).step_by(8) {
                    match flag {
                        3 => instr_set.insert(flag, "RFC".to_string()),
                        11 => instr_set.insert(flag, "RFZ".to_string()),
                        19 => instr_set.insert(flag, "RFS".to_string()),
                        27 => instr_set.insert(flag, "RFP".to_string()),
                        _ => continue
                    };
                };
                // RTc
                for flag in (35..60).step_by(8) {
                    match flag {
                        35 => instr_set.insert(flag, "RTC".to_string()),
                        43 => instr_set.insert(flag, "RTZ".to_string()),
                        51 => instr_set.insert(flag, "RTS".to_string()),
                        59 => instr_set.insert(flag, "RTP".to_string()),
                        _ => continue
                    };
                };
            };
        };
        // MACHINE INSTRUCTION
        {
            instr_set.insert(0, "HLT".to_string());   // 0b00_000_000
            instr_set.insert(1, "HLT".to_string());   // 0b00_000_001
            instr_set.insert(255, "HLT".to_string()); // 0b11_111_111
        };
        instr_set
    }

    fn time_instr() -> [Vec<String>; 3] {
        let one_cycle_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string(),
            "NOP".to_string(),"HLT".to_string(),
            "INB".to_string(),"INC".to_string(),"IND".to_string(),"INE".to_string(),"INH".to_string(),"INL".to_string(),
            "DCB".to_string(),"DCC".to_string(),"DCD".to_string(),"DCE".to_string(),"DCH".to_string(),"DCL".to_string(),
            "ADB".to_string(),"ADC".to_string(),"ADD".to_string(),"ADE".to_string(),"ADH".to_string(),"ADL".to_string(),
            "ACB".to_string(),"ACC".to_string(),"ACD".to_string(),"ACE".to_string(),"ACH".to_string(),"ACL".to_string(),
            "SUB".to_string(),"SUC".to_string(),"SUD".to_string(),"SUE".to_string(),"SUH".to_string(),"SUL".to_string(),
            "SBB".to_string(),"SBC".to_string(),"SBD".to_string(),"SBE".to_string(),"SBH".to_string(),"SBL".to_string(),
            "NDB".to_string(),"NDC".to_string(),"NDD".to_string(),"NDE".to_string(),"NDH".to_string(),"NDL".to_string(),
            "XRB".to_string(),"XRC".to_string(),"XRD".to_string(),"XRE".to_string(),"XRH".to_string(),"XRL".to_string(),
            "ORB".to_string(),"ORC".to_string(),"ORD".to_string(),"ORE".to_string(),"ORH".to_string(),"ORL".to_string(),
            "CPB".to_string(),"CPC".to_string(),"CPD".to_string(),"CPE".to_string(),"CPH".to_string(),"CPL".to_string(),
            "RET".to_string(),
            "RFC".to_string(),"RFZ".to_string(),"RFS".to_string(),"RFP".to_string(),
            "RTC".to_string(),"RTZ".to_string(),"RTS".to_string(),"RTP".to_string(),
        ];
        let two_cycle_instrs: Vec<String> = vec![
            "LAM".to_string(),"LBM".to_string(),"LCM".to_string(),"LDM".to_string(),"LEM".to_string(),"LHM".to_string(),"LLM".to_string(),
            "LMA".to_string(),"LMB".to_string(),"LMC".to_string(),"LMD".to_string(),"LME".to_string(),"LMH".to_string(),"LML".to_string(),
            "LAI".to_string(),"LBI".to_string(),"LCI".to_string(),"LDI".to_string(),"LEI".to_string(),"LHI".to_string(),"LLI".to_string(),
            "ADM".to_string(),
            "ACM".to_string(),
            "SUM".to_string(),
            "SBM".to_string(),
            "NDM".to_string(),
            "XRM".to_string(),
            "ORM".to_string(),
            "CPM".to_string(),
            "ADI".to_string(),
            "ACI".to_string(),
            "SUI".to_string(),
            "SBI".to_string(),
            "NDI".to_string(),
            "XRI".to_string(),
            "ORI".to_string(),
            "CPI".to_string(),
        ];
        let three_cycle_instrs: Vec<String> = vec![
            "LMI".to_string(),
            "JMP".to_string(),
            "JFC".to_string(),"JFZ".to_string(),"JFS".to_string(),"JFP".to_string(),
            "JTC".to_string(),"JTZ".to_string(),"JTS".to_string(),"JTP".to_string(),
            "CFC".to_string(),"CFZ".to_string(),"CFS".to_string(),"CFP".to_string(),
            "CTC".to_string(),"CTZ".to_string(),"CTS".to_string(),"CTP".to_string(),
            "CAL".to_string(),
        ];
        let instrs: [Vec<String>; 3] = [one_cycle_instrs, two_cycle_instrs, three_cycle_instrs];
        instrs
    }
    
    fn length_instr() -> [Vec<String>; 3] {
        let one_byte_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string(),
            "LAM".to_string(),"LBM".to_string(),"LCM".to_string(),"LDM".to_string(),"LEM".to_string(),"LHM".to_string(),"LLM".to_string(),
            "LMA".to_string(),"LMB".to_string(),"LMC".to_string(),"LMD".to_string(),"LME".to_string(),"LMH".to_string(),"LML".to_string(),
            "INB".to_string(),"INC".to_string(),"IND".to_string(),"INE".to_string(),"INH".to_string(),"INL".to_string(),
            "DCB".to_string(),"DCC".to_string(),"DCD".to_string(),"DCE".to_string(),"DCH".to_string(),"DCL".to_string(),
            "ADB".to_string(),"ADC".to_string(),"ADD".to_string(),"ADE".to_string(),"ADH".to_string(),"ADL".to_string(),
            "ACB".to_string(),"ACC".to_string(),"ACD".to_string(),"ACE".to_string(),"ACH".to_string(),"ACL".to_string(),
            "SUB".to_string(),"SUC".to_string(),"SUD".to_string(),"SUE".to_string(),"SUH".to_string(),"SUL".to_string(),
            "SBB".to_string(),"SBC".to_string(),"SBD".to_string(),"SBE".to_string(),"SBH".to_string(),"SBL".to_string(),
            "NDB".to_string(),"NDC".to_string(),"NDD".to_string(),"NDE".to_string(),"NDH".to_string(),"NDL".to_string(),
            "XRB".to_string(),"XRC".to_string(),"XRD".to_string(),"XRE".to_string(),"XRH".to_string(),"XRL".to_string(),
            "ORB".to_string(),"ORC".to_string(),"ORD".to_string(),"ORE".to_string(),"ORH".to_string(),"ORL".to_string(),
            "CPB".to_string(),"CPC".to_string(),"CPD".to_string(),"CPE".to_string(),"CPH".to_string(),"CPL".to_string(),
            "ADM".to_string(),
            "ACM".to_string(),
            "SUM".to_string(),
            "SBM".to_string(),
            "NDM".to_string(),
            "XRM".to_string(),
            "ORM".to_string(),
            "CPM".to_string(),
            "NOP".to_string(),"HLT".to_string(),
            "RET".to_string(),
            "RFC".to_string(),"RFZ".to_string(),"RFS".to_string(),"RFP".to_string(),
            "RTC".to_string(),"RTZ".to_string(),"RTS".to_string(),"RTP".to_string(),
        ];
        let two_byte_instrs: Vec<String> = vec![
            "LAI".to_string(),"LBI".to_string(),"LCI".to_string(),"LDI".to_string(),"LEI".to_string(),"LHI".to_string(),"LLI".to_string(),
            "LMI".to_string(),
            "ADI".to_string(),
            "ACI".to_string(),
            "SUI".to_string(),
            "SBI".to_string(),
            "NDI".to_string(),
            "XRI".to_string(),
            "ORI".to_string(),
            "CPI".to_string(),
        ];
        let three_byte_instrs: Vec<String> = vec![
            "JMP".to_string(),
            "JFC".to_string(),"JFZ".to_string(),"JFS".to_string(),"JFP".to_string(),
            "JTC".to_string(),"JTZ".to_string(),"JTS".to_string(),"JTP".to_string(),
            "CFC".to_string(),"CFZ".to_string(),"CFS".to_string(),"CFP".to_string(),
            "CTC".to_string(),"CTZ".to_string(),"CTS".to_string(),"CTP".to_string(),
            "CAL".to_string(),
        ];
        let instrs: [Vec<String>; 3] = [one_byte_instrs, two_byte_instrs, three_byte_instrs];
        instrs
    }
    
    fn type_instr() -> [Vec<String>; 4] {
        let index_register_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string(),
            "LAM".to_string(),"LBM".to_string(),"LCM".to_string(),"LDM".to_string(),"LEM".to_string(),"LHM".to_string(),"LLM".to_string(),
            "LMA".to_string(),"LMB".to_string(),"LMC".to_string(),"LMD".to_string(),"LME".to_string(),"LMH".to_string(),"LML".to_string(),
            "LAI".to_string(),"LBI".to_string(),"LCI".to_string(),"LDI".to_string(),"LEI".to_string(),"LHI".to_string(),"LLI".to_string(),
            "LMI".to_string(),
            "INB".to_string(),"INC".to_string(),"IND".to_string(),"INE".to_string(),"INH".to_string(),"INL".to_string(),
            "DCB".to_string(),"DCC".to_string(),"DCD".to_string(),"DCE".to_string(),"DCH".to_string(),"DCL".to_string(),
        ];
        let accumulator_instr: Vec<String> = vec![
            "ADB".to_string(),"ADC".to_string(),"ADD".to_string(),"ADE".to_string(),"ADH".to_string(),"ADL".to_string(),
            "ACB".to_string(),"ACC".to_string(),"ACD".to_string(),"ACE".to_string(),"ACH".to_string(),"ACL".to_string(),
            "SUB".to_string(),"SUC".to_string(),"SUD".to_string(),"SUE".to_string(),"SUH".to_string(),"SUL".to_string(),
            "SBB".to_string(),"SBC".to_string(),"SBD".to_string(),"SBE".to_string(),"SBH".to_string(),"SBL".to_string(),
            "NDB".to_string(),"NDC".to_string(),"NDD".to_string(),"NDE".to_string(),"NDH".to_string(),"NDL".to_string(),
            "XRB".to_string(),"XRC".to_string(),"XRD".to_string(),"XRE".to_string(),"XRH".to_string(),"XRL".to_string(),
            "ORB".to_string(),"ORC".to_string(),"ORD".to_string(),"ORE".to_string(),"ORH".to_string(),"ORL".to_string(),
            "CPB".to_string(),"CPC".to_string(),"CPD".to_string(),"CPE".to_string(),"CPH".to_string(),"CPL".to_string(),
            "ADM".to_string(),
            "ACM".to_string(),
            "SUM".to_string(),
            "SBM".to_string(),
            "NDM".to_string(),
            "XRM".to_string(),
            "ORM".to_string(),
            "CPM".to_string(),
            "ADI".to_string(),
            "ACI".to_string(),
            "SUI".to_string(),
            "SBI".to_string(),
            "NDI".to_string(),
            "XRI".to_string(),
            "ORI".to_string(),
            "CPI".to_string(),
        ];
        let pc_stack_instr: Vec<String> = vec![
            "JMP".to_string(),
            "JFC".to_string(),"JFZ".to_string(),"JFS".to_string(),"JFP".to_string(),
            "JTC".to_string(),"JTZ".to_string(),"JTS".to_string(),"JTP".to_string(),
            "CFC".to_string(),"CFZ".to_string(),"CFS".to_string(),"CFP".to_string(),
            "CTC".to_string(),"CTZ".to_string(),"CTS".to_string(),"CTP".to_string(),
            "CAL".to_string(),
            "RET".to_string(),
            "RFC".to_string(),"RFZ".to_string(),"RFS".to_string(),"RFP".to_string(),
            "RTC".to_string(),"RTZ".to_string(),"RTS".to_string(),"RTP".to_string(),
        ];
        let machine_instr: Vec<String> = vec!["NOP".to_string(), "HLT".to_string()];
        let instrs: [Vec<String>; 4] = [index_register_instrs, accumulator_instr, pc_stack_instr, machine_instr];
        instrs
    }

    // ffunctions for getting all fields
    pub fn get_instr_set(&mut self) -> &HashMap<u8, String> {&self.instr_set}
    pub fn get_instr_time(&mut self) -> &[Vec<String>; 3] {&self.instr_time}
    pub fn get_instr_length(&mut self) -> &[Vec<String>; 3] {&self.instr_length}
    pub fn get_instr_type(&mut self) -> &[Vec<String>; 4] {&self.instr_type}
}