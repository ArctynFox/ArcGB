pub struct MemoryBus {
    pub memory: [u8; 0xFFFF]
}

impl Default for MemoryBus {
    fn default() -> MemoryBus {
        MemoryBus { memory: [0; 0xFFFF] }
    }
}

impl MemoryBus {
    fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::MemoryBus;

    #[test]
    fn test_read_byte() {
        let mut bus = MemoryBus::default();
        bus.memory[0x0000] = 175;

        assert_eq!(bus.read_byte(0x0000), 175);
    }
}