use super::memory::MemoryBus;
use super::registers::Registers;

pub enum Instruction {
    ADD(TargetRegister),
    ADDHL(TargetRegister),
    ADC(TargetRegister),
    SUB(TargetRegister),
}

// arithmetic can be performed on all physical registers other than F
pub enum TargetRegister {
    A,B,C,D,E,H,L,N8(i16),N16(i32),E8(i8),U3(u8),BC,DE,HL,HLAsAddress
}

// implementation of the instructions on the hardware
pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub memory_bus: MemoryBus,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: Default::default(),
            pc: Default::default(),
            memory_bus: MemoryBus::default(),
        }
    }
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            // accepts any target r8 register and n8 and HL address
            Instruction::ADD(target) => self.add(target),
            // accepts any target r16 register and SP (TODO: SP functionality)
            Instruction::ADDHL(target) => self.addhl(target),
            // accepts any target r8 register and n8 and HL address
            Instruction::ADC(target) => self.adc(target),
            // accepts any target r8 register and n8 and HL address
            Instruction::SUB(target) => self.sub(target),
        }
    }
}

