use std::collections::HashMap;

pub struct Dictionary {
    opcode_set: HashMap<String, u8>
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary{
            opcode_set: Dictionary::instructions()
        }
    }

    fn instructions() -> HashMap<String, u8> {
        let mut opcode_set: HashMap<String, u8> = HashMap::new();

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
                            opcode_set.insert("NOP".to_string(), op);
                        } else {
                            if reg1 == 192 {
                                match reg2 {
                                    1 => opcode_set.insert("LAB".to_string(), op),
                                    2 => opcode_set.insert("LAC".to_string(), op),
                                    3 => opcode_set.insert("LAD".to_string(), op),
                                    4 => opcode_set.insert("LAE".to_string(), op),
                                    5 => opcode_set.insert("LAH".to_string(), op),
                                    6 => opcode_set.insert("LAL".to_string(), op),
                                    _ => continue
                                };
                            } else if reg1 == 200 {
                                match reg2 {
                                    0 => opcode_set.insert("LBA".to_string(), op),
                                    2 => opcode_set.insert("LBC".to_string(), op),
                                    3 => opcode_set.insert("LBD".to_string(), op),
                                    4 => opcode_set.insert("LBE".to_string(), op),
                                    5 => opcode_set.insert("LBH".to_string(), op),
                                    6 => opcode_set.insert("LBL".to_string(), op),
                                    _ => continue
                                };
                            } else if reg1 == 208 {
                                match reg2 {
                                    0 => opcode_set.insert("LCA".to_string(), op),
                                    1 => opcode_set.insert("LCB".to_string(), op),
                                    3 => opcode_set.insert("LCD".to_string(), op),
                                    4 => opcode_set.insert("LCE".to_string(), op),
                                    5 => opcode_set.insert("LCH".to_string(), op),
                                    6 => opcode_set.insert("LCL".to_string(), op),
                                    _ => continue
                                };
                            } else if reg1 == 216 {
                                match reg2 {
                                    0 => opcode_set.insert("LDA".to_string(), op),
                                    1 => opcode_set.insert("LDB".to_string(), op),
                                    2 => opcode_set.insert("LDC".to_string(), op),
                                    4 => opcode_set.insert("LDE".to_string(), op),
                                    5 => opcode_set.insert("LDH".to_string(), op),
                                    6 => opcode_set.insert("LDL".to_string(), op),
                                    _ => continue
                                };
                            } else if reg1 == 224 {
                                match reg2 {
                                    0 => opcode_set.insert("LEA".to_string(), op),
                                    1 => opcode_set.insert("LEB".to_string(), op),
                                    2 => opcode_set.insert("LEC".to_string(), op),
                                    3 => opcode_set.insert("LED".to_string(), op),
                                    5 => opcode_set.insert("LEH".to_string(), op),
                                    6 => opcode_set.insert("LEL".to_string(), op),
                                    _ => continue
                                };
                            } else if reg1 == 232 {
                                match reg2 {
                                    0 => opcode_set.insert("LHA".to_string(), op),
                                    1 => opcode_set.insert("LHB".to_string(), op),
                                    2 => opcode_set.insert("LHC".to_string(), op),
                                    3 => opcode_set.insert("LHD".to_string(), op),
                                    4 => opcode_set.insert("LHE".to_string(), op),
                                    6 => opcode_set.insert("LHL".to_string(), op),
                                    _ => continue
                                };
                            } else if reg1 == 240 {
                                match reg2 {
                                    0 => opcode_set.insert("LLA".to_string(), op),
                                    1 => opcode_set.insert("LLB".to_string(), op),
                                    2 => opcode_set.insert("LLC".to_string(), op),
                                    3 => opcode_set.insert("LLD".to_string(), op),
                                    4 => opcode_set.insert("LLE".to_string(), op),
                                    5 => opcode_set.insert("LLH".to_string(), op),
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
                        199 => opcode_set.insert("LAM".to_string(), reg),
                        207 => opcode_set.insert("LBM".to_string(), reg),
                        215 => opcode_set.insert("LCM".to_string(), reg),
                        223 => opcode_set.insert("LDM".to_string(), reg),
                        231 => opcode_set.insert("LEM".to_string(), reg),
                        239 => opcode_set.insert("LHM".to_string(), reg),
                        248 => opcode_set.insert("LLM".to_string(), reg),
                        _ => continue
                    };
                };
            };
            // LOAD MEM <- REG
            {
                for reg in 248..255{
                    match reg {
                        248 => opcode_set.insert("LMA".to_string(), reg),
                        249 => opcode_set.insert("LMB".to_string(), reg),
                        250 => opcode_set.insert("LMC".to_string(), reg),
                        251 => opcode_set.insert("LMD".to_string(), reg),
                        252 => opcode_set.insert("LME".to_string(), reg),
                        253 => opcode_set.insert("LMH".to_string(), reg),
                        254 => opcode_set.insert("LML".to_string(), reg),
                        _ => continue
                    };
                };
            };
            // LOAD REG <- DATA IMMEDIATE
            {
                for reg in (6..55).step_by(8) {
                    match reg {
                        6 =>  opcode_set.insert("LAI".to_string(), reg),
                        14 => opcode_set.insert("LBI".to_string(), reg),
                        22 => opcode_set.insert("LCI".to_string(), reg),
                        30 => opcode_set.insert("LDI".to_string(), reg),
                        38 => opcode_set.insert("LEI".to_string(), reg),
                        46 => opcode_set.insert("LHI".to_string(), reg),
                        54 => opcode_set.insert("LLI".to_string(), reg),
                        _ => continue
                    };
                };
            };
            // LOAD MEM <- DATA IMEDIATE
            {
                opcode_set.insert("LMI".to_string(), 0x3E); 
            };
            // INCREMENT / DECREMENT INSTRUCTION
            {
                for reg in (8..49).step_by(8) {
                    match reg {
                        8 => {
                            opcode_set.insert("INB".to_string(), reg  );
                            opcode_set.insert("DCB".to_string(), reg+1);
                        },
                        16 => {
                            opcode_set.insert("INC".to_string(), reg  );
                            opcode_set.insert("DCC".to_string(), reg+1);
                        },
                        24 => {
                            opcode_set.insert("IND".to_string(), reg  );
                            opcode_set.insert("DCD".to_string(), reg+1);
                        },
                        32 => {
                            opcode_set.insert("INE".to_string(), reg  );
                            opcode_set.insert("DCE".to_string(), reg+1);
                        },
                        40 => {
                            opcode_set.insert("INH".to_string(), reg  );
                            opcode_set.insert("DCH".to_string(), reg+1);
                        },
                        48 => {
                            opcode_set.insert("INL".to_string(), reg  );
                            opcode_set.insert("DCL".to_string(), reg+1);
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
                        128 | 134 => opcode_set.insert("ADL".to_string(), reg),
                        129 => opcode_set.insert("ADB".to_string(), reg),
                        130 => opcode_set.insert("ADC".to_string(), reg),
                        131 => opcode_set.insert("ADD".to_string(), reg),
                        132 => opcode_set.insert("ADE".to_string(), reg),
                        133 => opcode_set.insert("ADH".to_string(), reg),
                        _ => continue
                    };
                }
                // ADD REG A <- REG A + reg + CARRY
                for reg in 136..144 {
                    match reg {
                        136 | 142 => opcode_set.insert("ACL".to_string(), reg),
                        137 => opcode_set.insert("ACB".to_string(), reg),
                        138 => opcode_set.insert("ACC".to_string(), reg),
                        139 => opcode_set.insert("ACD".to_string(), reg),
                        140 => opcode_set.insert("ACE".to_string(), reg),
                        141 => opcode_set.insert("ACH".to_string(), reg),
                        _ => continue, 
                    };
                }
                // SUBSTRACT REG A <- REG A - reg
                for reg in 144..152 {
                    match reg {
                        144 | 150 => opcode_set.insert("SUL".to_string(), reg),
                        145 => opcode_set.insert("SUB".to_string(), reg),
                        146 => opcode_set.insert("SUC".to_string(), reg),
                        147 => opcode_set.insert("SUD".to_string(), reg),
                        148 => opcode_set.insert("SUE".to_string(), reg),
                        149 => opcode_set.insert("SUH".to_string(), reg),
                        _ => continue,
                    };
                }
                // SUBSTRACT REG A <- REG A - reg - BORROW
                for reg in 152..160 {
                    match reg {
                        152 | 158 => opcode_set.insert("SBL".to_string(), reg),
                        153 => opcode_set.insert("SBB".to_string(), reg),
                        154 => opcode_set.insert("SBC".to_string(), reg),
                        155 => opcode_set.insert("SBD".to_string(), reg),
                        156 => opcode_set.insert("SBE".to_string(), reg),
                        157 => opcode_set.insert("SBH".to_string(), reg),
                        _ => continue,
                    };
                }
                // AND REG A <- REG A & reg
                for reg in 160..168 {
                    match reg {
                        160 | 166 => opcode_set.insert("NDL".to_string(), reg),
                        161 => opcode_set.insert("NDB".to_string(), reg),
                        162 => opcode_set.insert("NDC".to_string(), reg),
                        163 => opcode_set.insert("NDD".to_string(), reg),
                        164 => opcode_set.insert("NDE".to_string(), reg),
                        165 => opcode_set.insert("NDH".to_string(), reg),
                        _ => continue,
                    };
                }
                // XOR REG A <- REG A ^ reg
                for reg in 168..176 {
                    match reg {
                        168 | 174 => opcode_set.insert("XRL".to_string(), reg),
                        169 => opcode_set.insert("XRB".to_string(), reg),
                        170 => opcode_set.insert("XRC".to_string(), reg),
                        171 => opcode_set.insert("XRD".to_string(), reg),
                        172 => opcode_set.insert("XRE".to_string(), reg),
                        173 => opcode_set.insert("XRH".to_string(), reg),
                        _ => continue,
                    };
                }
                // OR REG A <- REG A | reg
                for reg in 176..184 {
                    match reg {
                        176 | 182 => opcode_set.insert("ORL".to_string(), reg),
                        177 => opcode_set.insert("ORB".to_string(), reg),
                        178 => opcode_set.insert("ORC".to_string(), reg),
                        179 => opcode_set.insert("ORD".to_string(), reg),
                        180 => opcode_set.insert("ORE".to_string(), reg),
                        181 => opcode_set.insert("ORH".to_string(), reg),
                        _ => continue,
                    };
                }
                // COMPARE REG A - reg
                for reg in 184..192 {
                    match reg {
                        184 | 190 => opcode_set.insert("CPL".to_string(), reg),
                        185 => opcode_set.insert("CPB".to_string(), reg),
                        186 => opcode_set.insert("CPC".to_string(), reg),
                        187 => opcode_set.insert("CPD".to_string(), reg),
                        188 => opcode_set.insert("CPE".to_string(), reg),
                        189 => opcode_set.insert("CPH".to_string(), reg),
                        _ => continue,
                    };
                }
            };
            // ALU OPERATIONS WITH MEMORY
            {
                // ADD
                opcode_set.insert("ADM".to_string(), 0x87);
                // ADD + CARRY
                opcode_set.insert("ACM".to_string(), 0x8F);
                // SUBSTRACT
                opcode_set.insert("SUM".to_string(), 0x97);
                // SUBSTRACT WITH BORROW
                opcode_set.insert("SBM".to_string(), 0x9F);
                // LOGICAL AND
                opcode_set.insert("NDM".to_string(), 0xA7);
                // LOGICAL XOR
                opcode_set.insert("XRM".to_string(), 0xAF);
                // LOGICAL ORI
                opcode_set.insert("ORM".to_string(), 0xB7);
                // COMPARE
                opcode_set.insert("CPM".to_string(), 0xBF);
            };
            // ALU IMMEDIATE INSTRUCTIONS
            {
                // ADD
                opcode_set.insert("ADI".to_string(), 0x04);
                // ADD + CARRY
                opcode_set.insert("ACI".to_string(), 0x0C);
                // SUBSTRACT
                opcode_set.insert("SUI".to_string(), 0x14);
                // SUBSTRACT WITH BORROW
                opcode_set.insert("SBI".to_string(), 0x1C);
                // LOGICAL AND
                opcode_set.insert("NDI".to_string(), 0x24);
                // LOGICAL XOR
                opcode_set.insert("XRI".to_string(), 0x2C);
                // LOGICAL ORI
                opcode_set.insert("ORI".to_string(), 0x34);
                // COMPARE
                opcode_set.insert("CPI".to_string(), 0x3C);
            };
            // ROTATE INSTRUCTIONS
            {
                // ROTATE LEFT CONTENT
                opcode_set.insert("RLC".to_string(), 0x02);
                // ROATATE RIGHT CONTENT 
                opcode_set.insert("RRC".to_string(), 0x0A);
                // ROTATE LEFT ACCUMULATOR 
                opcode_set.insert("RAL".to_string(), 0x12);
                // ROTATE RIGHT ACCUMULATOR
                opcode_set.insert("RAR".to_string(), 0x1A);

            };
        };
        // PROGRAM COUNTER AND STACK CONTROL INSTRUCTIONS
        {
            // JUMP INSTRUCTIONS
            {
                // JMP
                for x in (68..125).step_by(8) {
                    opcode_set.insert("JMP".to_string(), x);
                };
                // JFc
                for flag in (64..89).step_by(8) {
                    match flag {
                        64 => opcode_set.insert("JFC".to_string(), flag),
                        72 => opcode_set.insert("JFZ".to_string(), flag),
                        80 => opcode_set.insert("JFS".to_string(), flag),
                        88 => opcode_set.insert("JFP".to_string(), flag),
                        _ => continue
                    };
                };
                // JTc
                for flag in (96..121).step_by(8) {
                    match flag {
                        96 => opcode_set.insert("JTC".to_string(), flag),
                        104 => opcode_set.insert("JTZ".to_string(), flag),
                        112 => opcode_set.insert("JTS".to_string(), flag),
                        120 => opcode_set.insert("JTP".to_string(), flag),
                        _ => continue
                    };
                };
            };
            // CALL INSTRUCTIONS
            {
                // CAL
                for x in (70..127).step_by(8) {
                    opcode_set.insert("CAL".to_string(), x);
                }
                // CFc
                for flag in (66..91).step_by(8) {
                    match flag {
                        66 => opcode_set.insert("CFC".to_string(), flag),
                        74 => opcode_set.insert("CFZ".to_string(), flag),
                        82 => opcode_set.insert("CFS".to_string(), flag),
                        90 => opcode_set.insert("CFP".to_string(), flag),
                        _ => continue
                    };
                };
                // CTc
                for flag in (98..123).step_by(8) {
                    match flag {
                        98 => opcode_set.insert("CTC".to_string(), flag),
                        106 => opcode_set.insert("CTZ".to_string(), flag),
                        114 => opcode_set.insert("CTS".to_string(), flag),
                        122 => opcode_set.insert("CTP".to_string(), flag),
                        _ => continue
                    };
                };
            };
            // RETURN INSTRUCTIONS
            {
                // RET
                for x in (7..64).step_by(8) {
                    opcode_set.insert("RET".to_string(), x);
                }
                // RFc
                for flag in (3..28).step_by(8) {
                    match flag {
                        3 => opcode_set.insert("RFC".to_string(), flag),
                        11 => opcode_set.insert("RFZ".to_string(), flag),
                        19 => opcode_set.insert("RFS".to_string(), flag),
                        27 => opcode_set.insert("RFP".to_string(), flag),
                        _ => continue
                    };
                };
                // RTc
                for flag in (35..60).step_by(8) {
                    match flag {
                        35 => opcode_set.insert("RTC".to_string(), flag),
                        43 => opcode_set.insert("RTZ".to_string(), flag),
                        51 => opcode_set.insert("RTS".to_string(), flag),
                        59 => opcode_set.insert("RTP".to_string(), flag),
                        _ => continue
                    };
                };
            };
            // RESART 
            {
                for address in (5..62).step_by(8) {
                    match address {
                        5 => opcode_set.insert("RST".to_string(), address),
                        13 => opcode_set.insert("RST".to_string(), address),
                        21 => opcode_set.insert("RST".to_string(), address),
                        29 => opcode_set.insert("RST".to_string(), address),
                        37 => opcode_set.insert("RST".to_string(), address),
                        45 => opcode_set.insert("RST".to_string(), address),
                        53 => opcode_set.insert("RST".to_string(), address),
                        61 => opcode_set.insert("RST".to_string(), address),
                        _ => continue,
                    };
                }
            };
        };
        // MACHINE INSTRUCTION
        {
            opcode_set.insert("HLT".to_string(), 0);   // 0b00_000_000
            opcode_set.insert("HLT".to_string(), 1);   // 0b00_000_001
            opcode_set.insert("HLT".to_string(), 0xFF); // 0b11_111_111
        };
        opcode_set
    }

    pub fn get_instr_set(&mut self) -> &HashMap<String, u8> {&self.opcode_set}
}