pub mod registers;

enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
}

// arithmetic can be performed on all physical registers other than F
enum ArithmeticTarget {
    A,B,C,D,E,H,L,
}

// implementation of the instructions on the cpu
pub struct CPU {
    pub registers: registers::Registers
}

impl CPU {
    // arithmetic add instruction, adds given value to the value of the a register and returns it
    fn add(&mut self, value: u8) -> u8 {
        // add the accumulator
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        // handle flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    // arithmetic addhl instruction, adds given value to the value of the a register and returns it
    fn addhl(&mut self, value: u16) -> u16 {
        // add the accumulator
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        // handle flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFF) + (value & 0xFF) > 0xFF;
        new_value
    }

    // add and carry: adds given value and carry flag to the value of register a and returns it
    fn adc(&mut self, value: u8) -> u8 {
        // add the accumulator
        let (mid_value, did_overflow_add) = self.registers.a.overflowing_add(value);
        // add the value of the carry flag
        let (new_value, did_overflow_adc) = mid_value.overflowing_add(self.registers.f.carry as u8);
        // handle flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // the value of carry is considered when determining half_carry
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + (self.registers.f.carry as u8)> 0xF;
        self.registers.f.carry = did_overflow_add || did_overflow_adc;
        new_value
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                let value = self.get_register_value(target);
                self.registers.a = self.add(value);
            }
            Instruction::ADDHL(target) => {
                let value = self.get_register_value(target) as u16;
                let new_value = self.addhl(value);
                self.registers.set_hl(new_value);
            }
            Instruction::ADC(target) => {
                let value = self.get_register_value(target);
                self.registers.a = self.adc(value);
            }
        }
    }

    // return the value of the register determined by the enum
    fn get_register_value(&self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_add_work_properly() {
        let mut cpu = CPU { registers: registers::Registers::default()};
        cpu.registers.a = 11;
        cpu.registers.b = 127;

        cpu.execute(Instruction::ADD(ArithmeticTarget::B));

        assert_eq!(cpu.registers.a, 138);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn does_addhl_work_properly() {
        let mut cpu = CPU { registers: registers::Registers::default()};
        cpu.registers.set_hl(65535);
        cpu.registers.b = 255;

        cpu.execute(Instruction::ADDHL(ArithmeticTarget::B));

        assert_eq!(cpu.registers.get_hl(), 254);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn does_adc_work_properly() {
        let mut cpu = CPU { registers: registers::Registers::default()};
        cpu.registers.f.carry = true;
        cpu.registers.a = 254;
        cpu.registers.d = 21;

        cpu.execute(Instruction::ADC(ArithmeticTarget::D));

        assert_eq!(cpu.registers.a, 20);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, true);
    }
}