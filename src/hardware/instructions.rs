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
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
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
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFF) + (value & 0xFF) > 0xFF;
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
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + (self.registers.f.carry as u8) > 0xF;
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
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.a = new_value;
    }
    //-------------------INSTRUCTION FUNCTIONALITY END-------------------

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

}

#[cfg(test)]
mod tests {
    use crate::hardware::cpu::*;

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
}