use crate::datatypes::U3;
use super::memory::MemoryBus;
use super::Registers;

/**
 * Enum which allows type validation for target data used by instructions
 */
pub enum TargetRegister {
    A,B,C,D,E,H,L,N8(i16),N16(i32),E8(i8),U3(u8),BC,DE,HL,HLAsPointer
}

/**
 * Implementation of the GameBoy's CPU.
 */
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

/**
 * Jumptable of instructions supported by the GameBoy as an enum.
 */
pub enum Instruction {
    ADC(TargetRegister),
    ADD(TargetRegister),
    ADDHL(TargetRegister),
    AND(TargetRegister),
    BIT(TargetRegister, U3),
    SUB(TargetRegister),
}

impl CPU {
    /**
     * Execute the given instruction on the emulated CPU.
     *
     * | Parameter | Explanation                                                         |
     * | --------- | ------------------------------------------------------------------- |
     * | `instruction` | The instruction to execute along with its necessary parameters. |
     */
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            // accepts any target r8 register or n8 or HL address
            Instruction::ADC(target) => self.adc(target),
            // accepts any target r8 register or n8 or HL address
            Instruction::ADD(target) => self.add(target),
            // accepts any target r16 register or SP (TODO: SP functionality)
            Instruction::ADDHL(target) => self.addhl(target),
            // accepts any target r8 register or HL address or n8
            Instruction::AND(target) => self.and(target),
            // accepts any target r8 register or HL address and u3
            Instruction::BIT(target, bit)  => self.bit(target, bit),
            // accepts any target r8 register or n8 or HL address
            Instruction::SUB(target) => self.sub(target),
        }
    }
}

