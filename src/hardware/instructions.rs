use super::cpu::{TargetRegister, CPU};

impl CPU {
    //-------------------INSTRUCTION FUNCTIONALITY START-------------------
    // arithmetic add instruction, adds given value to the value of the a register and returns it
    pub fn add(&mut self, target: TargetRegister) {
        let value = self.get_register_value(target) as u8;
        // add the accumulator
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        // handle flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = Self::did_half_carry(self.registers.a as usize, value as usize, 3, true);
        self.registers.a = new_value;
    }

    // arithmetic addhl instruction, adds given value to the value of the a register and returns it
    pub fn addhl(&mut self, target: TargetRegister) {
        let value = self.get_register_value(target) as u16;
        // add the accumulator
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        // handle flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = Self::did_half_carry(self.registers.get_hl() as usize, value as usize,11, true);
        self.registers.set_hl(new_value);
    }

    // add and carry: adds given value and carry flag to the value of register a and returns it
    pub fn adc(&mut self, target: TargetRegister) {
        let value = self.get_register_value(target) as u8;
        // add the accumulator
        let (mid_value, did_overflow_add) = self.registers.a.overflowing_add(value);
        // add the value of the carry flag
        let (new_value, did_overflow_adc) = mid_value.overflowing_add(self.registers.f.carry as u8);
        // handle flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // the value of carry is considered when determining half_carry
        self.registers.f.half_carry = (self.registers.a & 0xF) as u16 + (value & 0xF) as u16 + self.registers.f.carry as u16 > 0xF;
        self.registers.f.carry = did_overflow_add || did_overflow_adc;
        self.registers.a = new_value;
    }

    pub fn sub(&mut self, target: TargetRegister) {
        let value = self.get_register_value(target) as u8;
        // subtract the value from the register
        let (new_value, did_underflow) = self.registers.a.overflowing_sub(value);
        // handle flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_underflow;
        self.registers.f.half_carry = Self::did_half_carry(self.registers.a as usize, value as usize, 3, false);
        self.registers.a = new_value;
    }
    //-------------------INSTRUCTION FUNCTIONALITY END-------------------

    //----------------------HELPER FUNCTIONS START-----------------------
    // return the value of the register determined by the enum
    fn get_register_value(&self, target: TargetRegister) -> isize {
        match target {
            TargetRegister::A => self.registers.a as isize,
            TargetRegister::B => self.registers.b as isize,
            TargetRegister::C => self.registers.c as isize,
            TargetRegister::D => self.registers.d as isize,
            TargetRegister::E => self.registers.e as isize,
            TargetRegister::H => self.registers.h as isize,
            TargetRegister::L => self.registers.l as isize,
            TargetRegister::N8(value) => value as isize,
            TargetRegister::N16(value) => value as isize,
            TargetRegister::E8(value) => value as isize,
            TargetRegister::U3(value) => value as isize,
            TargetRegister::BC => self.registers.get_bc() as isize,
            TargetRegister::DE => self.registers.get_de() as isize,
            TargetRegister::HL => self.registers.get_hl() as isize,
            TargetRegister::HLAsAddress => self.memory_bus.memory[self.registers.get_hl() as usize] as isize
        }
    }

    fn did_half_carry(a: usize, b: usize, carry_position: u8, is_addition: bool) -> bool {
        let mask = (1usize << carry_position) - 1;

        if is_addition {
            (a & mask) + (b & mask) > mask
        } else {
            (a & mask) < (b & mask)
        }
    }
    //---------------------HELPER FUNCTIONS END-------------------------
}

#[cfg(test)]
mod tests {
    use crate::hardware::cpu::*;

    //--------------------ADD START-----------------------
    #[test]
    fn test_add() {
        let mut cpu = CPU::default();
        cpu.registers.a = 11;
        cpu.registers.b = 127;

        cpu.execute(Instruction::ADD(TargetRegister::B));

        assert_eq!(cpu.registers.a, 138);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn test_add_no_flags() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b0000_0001;
        cpu.registers.b = 0b0000_0010;

        cpu.execute(Instruction::ADD(TargetRegister::B));

        assert_eq!(cpu.registers.a, 0b0000_0011);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, false);
    }

    #[test]
    fn test_add_carry_no_half_carry() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b1111_0000;
        cpu.registers.b = 0b0010_0000;

        cpu.execute(Instruction::ADD(TargetRegister::B));

        assert_eq!(cpu.registers.a, 0b0001_0000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, false);
    }

    #[test]
    fn test_add_zero() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b1111_1111;
        cpu.registers.b = 0b0000_0001;

        cpu.execute(Instruction::ADD(TargetRegister::B));

        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, true);
    }
    //----------------ADD END-----------------

    //--------------ADDHL START---------------
    #[test]
    fn test_addhl() {
        let mut cpu = CPU::default();
        cpu.registers.set_hl(65535);
        cpu.registers.b = 255;

        cpu.execute(Instruction::ADDHL(TargetRegister::B));

        assert_eq!(cpu.registers.get_hl(), 254);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn test_addhl_no_flags() {
        let mut cpu = CPU::default();
        cpu.registers.set_hl(0b0001_0000_0000_0000);
        cpu.registers.set_bc(0b0000_0000_0000_0001);
        cpu.registers.f.zero = false;

        cpu.execute(Instruction::ADDHL(TargetRegister::BC));

        assert_eq!(cpu.registers.get_hl(), 0b0001_0000_0000_0001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, false);
    }

    #[test]
    fn test_addhl_half_carry_no_carry() {
        let mut cpu = CPU::default();
        cpu.registers.set_hl(0b0000_1111_1111_1111);
        cpu.registers.set_bc(0b0000_0000_0000_0001);

        cpu.execute(Instruction::ADDHL(TargetRegister::BC));

        assert_eq!(cpu.registers.get_hl(), 0b0001_0000_0000_0000);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn test_addhl_carry_no_half_carry() {
        let mut cpu = CPU::default();
        cpu.registers.set_hl(0b1111_0000_0000_0000);
        cpu.registers.set_bc(0b0001_0000_0000_0000);
        cpu.registers.f.zero = true;

        cpu.execute(Instruction::ADDHL(TargetRegister::BC));

        assert_eq!(cpu.registers.get_hl(), 0b0000_0000_0000_0000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, false);
    }
    //---------------ADDHL END---------------

    //---------------ADC START---------------
    #[test]
    fn test_adc() {
        let mut cpu = CPU::default();
        cpu.registers.f.carry = true;
        cpu.registers.a = 254;
        cpu.registers.d = 21;

        cpu.execute(Instruction::ADC(TargetRegister::D));

        assert_eq!(cpu.registers.a, 20);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn test_adc_no_flags() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b0000_0001;
        cpu.registers.d = 0b0000_0001;
        cpu.registers.f.carry = false;

        cpu.execute(Instruction::ADC(TargetRegister::D));

        assert_eq!(cpu.registers.a, 0b0000_0010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, false);
    }

    #[test]
    fn test_adc_carry_in_causes_half_carry() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b0000_1111;
        cpu.registers.d = 0b0000_0000;
        cpu.registers.f.carry = true;

        cpu.execute(Instruction::ADC(TargetRegister::D));

        assert_eq!(cpu.registers.a, 0b0001_0000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn test_adc_carry_no_half_carry() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b1111_0000;
        cpu.registers.d = 0b0001_0000;
        cpu.registers.f.carry = false;

        cpu.execute(Instruction::ADC(TargetRegister::D));

        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, false);
    }
    //---------------ADC END----------------

    //--------------SUB START---------------
    #[test]
    fn test_sub() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b11010000;
        cpu.registers.d = 0b00100100;

        cpu.execute(Instruction::SUB(TargetRegister::D));

        assert_eq!(cpu.registers.a, 0b10101100);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn test_sub_no_borrow() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b0010_0000;
        cpu.registers.d = 0b0000_0001;

        cpu.execute(Instruction::SUB(TargetRegister::D));

        assert_eq!(cpu.registers.a, 0b0001_1111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn test_sub_full_borrow() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b0000_0000;
        cpu.registers.d = 0b0000_0001;

        cpu.execute(Instruction::SUB(TargetRegister::D));

        assert_eq!(cpu.registers.a, 0b1111_1111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn test_sub_zero() {
        let mut cpu = CPU::default();
        cpu.registers.a = 0b0101_0101;
        cpu.registers.d = 0b0101_0101;

        cpu.execute(Instruction::SUB(TargetRegister::D));

        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, false);
    }
    //--------------SUB END----------------

    #[test]
    fn test_did_half_carry() {
        // u8 half-carry: bit 3 -> bit 4
        assert!(CPU::did_half_carry(0b0000_1111, 0b0000_0001, 4, true));
        assert!(CPU::did_half_carry(0b0000_1000, 0b0000_1000, 4, true));
        assert!(!CPU::did_half_carry(0b0000_0111, 0b0000_0001, 4, true));
        assert!(!CPU::did_half_carry(0b0000_0010, 0b0000_0011, 4, true));

        // u8 half-borrow: bit 4 -> bit 3
        assert!(CPU::did_half_carry(0b0001_0000, 0b0000_0001, 4, false));
        assert!(CPU::did_half_carry(0b0010_0000, 0b0000_0001, 4, false));
        assert!(!CPU::did_half_carry(0b0001_0101, 0b0000_0011, 4, false));
        assert!(!CPU::did_half_carry(0b0000_0011, 0b0000_0010, 4, false));

        // ADD HL, rr half-carry: bit 11 -> bit 12
        assert!(CPU::did_half_carry(0b0000_1111_1111_1111, 0b0000_0000_0000_0001, 12, true));
        assert!(CPU::did_half_carry(0b0000_1000_0000_0000, 0b0000_1000_0000_0000, 12, true));
        assert!(!CPU::did_half_carry(0b0000_0111_1111_1111, 0b0000_0000_0000_0001, 12, true));

        // 16-bit subtraction half-borrow at bit 12
        assert!(CPU::did_half_carry(0b0001_0000_0000_0000, 0b0000_0000_0000_0001, 12, false));
        assert!(!CPU::did_half_carry(0b0001_0000_0000_0101, 0b0000_0000_0000_0011, 12, false));
        assert!(!CPU::did_half_carry(0b0000_0000_0000_0011, 0b0000_0000_0000_0010, 12, false));
    }
}