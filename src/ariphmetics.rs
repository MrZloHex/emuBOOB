use super::Cpu;

impl Cpu {
    pub fn add(&mut self, reg: &str) -> u8 {
        let result: i16 = match reg {
            (self.get_r_a() as i16) + (*reg_b as i16);
        };
        self.set_flags(&result);
        result as u8
    }

    pub fn add_carry(&self, reg_a: &u8, reg_b: &u8, carry: &bool) -> u8 {
        let result: i16 = if *carry {
            (*reg_a as i16) + (*reg_b as i16) + 1
        } else {
            (*reg_a as i16) + (*reg_b as i16)
        };
        self.set_flags(&result);
        result as u8
    }
}