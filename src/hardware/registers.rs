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

impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::default(),
            h: 0,
            l: 0,
        }
    }
}


// implementations for 16-bit virtual register functions
impl Registers {
    // shift b left 8 bits and OR with c
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }

    // separate the given u16 into the first 8 bits for b and the last 8 bits for c
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
        self.f = FlagsRegister::from((value & 0xFF) as u8);
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

// ---------- model to handle the flags register easier by only ever modifying the upper 4 bits
// struct with separate bools for each of the possible flags, also deriving copy and clone so that converting to u8 doesn't require a pointer.
#[derive(Copy, Clone)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Default for FlagsRegister {
    fn default() -> Self {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

// basically writing this as an enum which tells what bit position belongs to what flag
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

// the following two functions only interact with the upper 4 bits of the u8 used for the flag register, so that we are certain we never mess with the lower 4
// convert from the FlagsRegister representative struct to u8
impl From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

//convert from u8 to FlagsRegister, ignoring the lower 4 bits
impl From<u8> for FlagsRegister {
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
// ---------- end flags register model

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_FLAGS_REGISTER: FlagsRegister = FlagsRegister {zero: false,subtract: false,half_carry: false,carry: false};

    const EMPTY_REGISTERS: Registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: EMPTY_FLAGS_REGISTER,
        h: 0,
        l: 0,
    };

    const UPPER_BYTE: u8 = 0b10101010;
    const LOWER_BYTE: u8 = 0b11000000;

    fn upper_lower_as_u16(upper: u8, lower: u8) -> u16 {
        (upper as u16) << 8 | lower as u16
    }

    #[test]
    fn test_get_bc() {
        let registers = Registers {
            a: 0,
            b: UPPER_BYTE,
            c: LOWER_BYTE,
            d: 0,
            e: 0,
            f: EMPTY_FLAGS_REGISTER,
            h: 0,
            l: 0,
        };

        let result = registers.get_bc();
        assert_eq!(result, upper_lower_as_u16(UPPER_BYTE, LOWER_BYTE));
    }

    #[test]
    fn test_set_bc() {
        let mut registers = EMPTY_REGISTERS;

        registers.set_bc(upper_lower_as_u16(UPPER_BYTE, LOWER_BYTE));
        assert_eq!(registers.b, UPPER_BYTE);
        assert_eq!(registers.c, LOWER_BYTE);
    }

    #[test]
    fn test_get_af() {
        let registers = Registers {
            a: UPPER_BYTE,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: LOWER_BYTE.into(),
            h: 0,
            l: 0,
        };

        let result = registers.get_af();
        assert_eq!(result, upper_lower_as_u16(UPPER_BYTE, LOWER_BYTE));
    }

    #[test]
    fn test_set_af() {
        let mut registers = EMPTY_REGISTERS;

        registers.set_af(upper_lower_as_u16(UPPER_BYTE, LOWER_BYTE));
        assert_eq!(registers.a, UPPER_BYTE);
        assert_eq!(u8::from(registers.f), LOWER_BYTE);

        // also a sanity check for conversion between u8 and FlagsRegister
        assert_eq!(registers.f.zero, true);
        assert_eq!(registers.f.subtract, true);
        assert_eq!(registers.f.half_carry, false);
        assert_eq!(registers.f.carry, false);
    }

    #[test]
    fn test_get_de() {
        let registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: UPPER_BYTE,
            e: LOWER_BYTE,
            f: EMPTY_FLAGS_REGISTER,
            h: 0,
            l: 0,
        };

        let result = registers.get_de();
        assert_eq!(result, upper_lower_as_u16(UPPER_BYTE, LOWER_BYTE));
    }

    #[test]
    fn test_set_de() {
        let mut registers = EMPTY_REGISTERS;

        registers.set_de(upper_lower_as_u16(UPPER_BYTE, LOWER_BYTE));
        assert_eq!(registers.d, UPPER_BYTE);
        assert_eq!(registers.e, LOWER_BYTE);
    }

    #[test]
    fn test_get_hl() {
        let registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: EMPTY_FLAGS_REGISTER,
            h: UPPER_BYTE,
            l: LOWER_BYTE,
        };

        let result = registers.get_hl();
        assert_eq!(result, upper_lower_as_u16(UPPER_BYTE, LOWER_BYTE));
    }

    #[test]
    fn test_set_hl() {
        let mut registers = EMPTY_REGISTERS;

        registers.set_hl(upper_lower_as_u16(UPPER_BYTE, LOWER_BYTE));
        assert_eq!(registers.h, UPPER_BYTE);
        assert_eq!(registers.l, LOWER_BYTE);
    }
}