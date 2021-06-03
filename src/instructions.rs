use std::collections::HashMap;

pub struct Instruction {
    instr_set: HashMap<u8, String>,
    instr_time: [Vec<String>; 3],
    instr_length: [Vec<String>; 3],
    instr_type: [Vec<String>; 2]
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

    pub fn opcodes() -> HashMap<u8, String> {
        let mut instr_set: HashMap<u8, String> = HashMap::new();
        
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
                        }
                    }
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
                    }
                }
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
                    }

                }
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
                        }
                        
                    }
                }
                // HALT INSTRUCTION
                {
                    instr_set.insert(0, "HLT".to_string());   // 0b00_000_000
                    instr_set.insert(1, "HLT".to_string());   // 0b00_000_001
                    instr_set.insert(255, "HLT".to_string()); // 0b11_111_111
                }
            }
        }
        instr_set
    }

    pub fn time_instr() -> [Vec<String>; 3] {
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
        ];
        let two_cycle_instrs: Vec<String> = vec![
            "LAM".to_string(),"LBM".to_string(),"LCM".to_string(),"LDM".to_string(),"LEM".to_string(),"LHM".to_string(),"LLM".to_string(),
            "LMA".to_string(),"LMB".to_string(),"LMC".to_string(),"LMD".to_string(),"LME".to_string(),"LMH".to_string(),"LML".to_string(),
            "LMI".to_string()
        ];
        let three_cycle_instrs: Vec<String> = vec!["CAL".to_string()];
        let instrs: [Vec<String>; 3] = [one_cycle_instrs, two_cycle_instrs, three_cycle_instrs];
        instrs
    }
    
    pub fn length_instr() -> [Vec<String>; 3] {
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
            "NOP".to_string(),"HLT".to_string(),
            "INB".to_string(),"INC".to_string(),"IND".to_string(),"INE".to_string(),"INH".to_string(),"INL".to_string(),
            "DCB".to_string(),"DCC".to_string(),"DCD".to_string(),"DCE".to_string(),"DCH".to_string(),"DCL".to_string(),
        ];
        let two_byte_instrs: Vec<String> = vec!["LMI".to_string()];
        let three_byte_instrs: Vec<String> = vec!["JMP".to_string()];
        let instrs: [Vec<String>; 3] = [one_byte_instrs, two_byte_instrs, three_byte_instrs];
        instrs
    }
    
    pub fn type_instr() -> [Vec<String>; 2] {
        let index_register_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string(),
            "INB".to_string(),"INC".to_string(),"IND".to_string(),"INE".to_string(),"INH".to_string(),"INL".to_string(),
            "DCB".to_string(),"DCC".to_string(),"DCD".to_string(),"DCE".to_string(),"DCH".to_string(),"DCL".to_string(),
            "LAM".to_string(),"LBM".to_string(),"LCM".to_string(),"LDM".to_string(),"LEM".to_string(),"LHM".to_string(),"LLM".to_string(),
            "LMA".to_string(),"LMB".to_string(),"LMC".to_string(),"LMD".to_string(),"LME".to_string(),"LMH".to_string(),"LML".to_string(),
        ];
        let machine_instr: Vec<String> = vec!["NOP".to_string(), "HLT".to_string()];
        let instrs: [Vec<String>; 2] = [index_register_instrs, machine_instr];
        instrs
    }

    // ffunctions for getting all fields
    pub fn get_instr_set(&mut self) -> &HashMap<u8, String> {&self.instr_set}
    pub fn get_instr_time(&mut self) -> &[Vec<String>; 3] {&self.instr_time}
    pub fn get_instr_length(&mut self) -> &[Vec<String>; 3] {&self.instr_length}
    pub fn get_instr_type(&mut self) -> &[Vec<String>; 2] {&self.instr_type}
}