// ---------- emulate the CPU's registers
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister, // Flags register, only the upper 4 bits should ever be used and are flags automatically set by CPU
    pub h: u8,
    pub l: u8,
}

// implementations for 16-bit virtual register functions
impl Registers {
    // shift b left 8 bits and OR with c
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }

    // separate the given u16 into the first 8 bits for b andd the last 8 bits for c
    pub fn set_bc(&mut self, value: u16) {
        // shift the bits right by 8 to get the b value
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    // the rest of these are the same as how bc works but for different registers
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8
        | u8::from(self.f) as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = ((value & 0xFF) as u8).into();
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
// ---------- end emulation of CPU registers

// ---------- model to handle the flags register easier by only ever modifying the upper 4 bits
// struct with separate bools for each of the possible flags, also deriving copy and clone so that converting to u8 doesn't require a pointer.
#[derive(Copy, Clone)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

// basically writing this as an enum which tells what bit position belongs to what flag
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

// the following two functions only interact with the upper 4 bits of the u8 used for the flag register, so that we are certain we never mess with the lower 4
// convert from the FlagsRegister representative struct to a u8
impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

//convert from a u8 to FlagsRegister, ignoring the lower 4 bits
impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}
// ---------- end flags register model model

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_get_bc_work() {
        let registers = Registers {
            a: 0,
            b: 170, // 10101010
            c: 234, // 11101010
            d: 0,
            e: 0,
            f: FlagsRegister {zero: false,subtract: false,half_carry: false,carry: false},
            h: 0,
            l: 0,
        };

        let testval = registers.get_bc();
        assert_eq!(testval, 43754); // 1010101011101010
    }

    #[test]
    fn does_set_bc_work() {
        let mut registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister {zero: false,subtract: false,half_carry: false,carry: false},
            h: 0,
            l: 0,
        };

        registers.set_bc(50000); // 1100001101010000
        assert_eq!(registers.b, 195); // 11000011
        assert_eq!(registers.c, 80); // 01010000
    }

    #[test]
    fn does_get_de_work() {
        let registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 122, // 01111010
            e: 3, // 00000011
            f: FlagsRegister {zero: false,subtract: false,half_carry: false,carry: false},
            h: 0,
            l: 0,
        };

        let testval = registers.get_de();
        assert_eq!(testval, 31235); // 0111101000000011
    }

    #[test]
    fn does_set_de_work() {
        let mut registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister {zero: false,subtract: false,half_carry: false,carry: false},
            h: 0,
            l: 0,
        };

        registers.set_de(24419); // 0101111101100011
        assert_eq!(registers.d, 95); // 01011111
        assert_eq!(registers.e, 99); // 01100011
    }
}