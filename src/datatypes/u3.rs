/// u3 datatype which represents a bit index for u8 values.
#[derive(Copy, Clone)]
pub struct U3 {
    pub value: [bool; 3]
}

impl U3 {
    pub fn from(n: u8) -> Self {
        Self {
            value: [
                n & 0b001 != 0,
                n & 0b010 != 0,
                n & 0b100 != 0,
            ]
        }
    }

    /// Returns a u8 where the only true bit is in the position dictated by the value of the u3, where the lowest bit is 0.
    pub fn to_definition(&self) -> u8 {
        1u8 << u8::from(self)
    }
}

impl From<&U3> for u8 {
    fn from(u3: &U3) -> u8 {
        (if u3.value[0] { 1 } else { 0 }) +
            (if u3.value[1]   { 2 } else { 0 }) +
            (if u3.value[2] { 4 } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u3_roundtrip() {
        for n in 0..7u8 {
            let u3 = U3::from(n);
            assert_eq!(n, u8::from(&u3));
            assert_eq!(1 << n, u3.to_definition());
        }
    }
}