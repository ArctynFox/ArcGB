mod cpu;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_main_reference_and_use_registers() {
        let registers = cpu::registers::Registers {
            a: 0,
            b: 170, // 10101010
            c: 234, // 11101010
            d: 0,
            e: 0,
            f: cpu::registers::FlagsRegister {zero: false,subtract: false,half_carry: false,carry: false},
            h: 0,
            l: 0,
        };
    
        println!("{:#010b}", registers.b);
    }
}