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