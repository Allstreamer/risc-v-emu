pub const MAX_MEMORY: u64 = 1024 * 1024 * 128;
pub const DRAM_BASE: u64 = 0x8000_0000;

#[derive(Debug, Clone, Default)]
pub struct Bus {
    memory: Memory,
}

impl Bus {
    pub fn new(memory: Memory) -> Self {
        Self { memory }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        if DRAM_BASE <= addr {
            return self.memory.load(addr, size);
        }
        Err(())
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        if DRAM_BASE <= addr {
            return self.memory.store(addr, size, value);
        }
        Err(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub dram: Vec<u8>,
}

impl Memory {
    pub fn new(code: Vec<u8>) -> Self {
        let mut dram = vec![0; MAX_MEMORY as usize];
        dram.splice(..code.len(), code.iter().cloned());

        Self { dram }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(()),
        }
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        match size {
            8 => Ok(self.store8(addr, value)),
            16 => Ok(self.store16(addr, value)),
            32 => Ok(self.store32(addr, value)),
            64 => Ok(self.store64(addr, value)),
            _ => Err(()),
        }
    }

    fn load64(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24)
            | ((self.dram[index + 4] as u64) << 32)
            | ((self.dram[index + 5] as u64) << 40)
            | ((self.dram[index + 6] as u64) << 48)
            | ((self.dram[index + 7] as u64) << 56);
    }

    fn store64(&mut self, addr: u64, value: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8;
        self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
        self.dram[index + 2] = ((value >> 16) & 0xff) as u8;
        self.dram[index + 3] = ((value >> 24) & 0xff) as u8;
        self.dram[index + 4] = ((value >> 32) & 0xff) as u8;
        self.dram[index + 5] = ((value >> 40) & 0xff) as u8;
        self.dram[index + 6] = ((value >> 48) & 0xff) as u8;
        self.dram[index + 7] = ((value >> 56) & 0xff) as u8;
    }

    fn load32(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24);
    }

    fn store32(&mut self, addr: u64, value: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8;
        self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
        self.dram[index + 2] = ((value >> 16) & 0xff) as u8;
        self.dram[index + 3] = ((value >> 24) & 0xff) as u8;
    }

    fn load16(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.dram[index] as u64) | ((self.dram[index + 1] as u64) << 8);
    }

    fn store16(&mut self, addr: u64, value: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8;
        self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
    }

    fn load8(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        return self.dram[index] as u64;
    }

    fn store8(&mut self, addr: u64, value: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dram_test() {
        let mut memory = Memory::new(vec![]);
        let sizes = [
            (8, [0, u8::MAX as u64]),
            (16, [0, u16::MAX as u64]),
            (32, [0, u32::MAX as u64]),
            (64, [0, u64::MAX]),
        ];
        for (size, targets) in sizes {
            for target in targets {
                let addr = DRAM_BASE + 0;

                assert!(memory.store(addr, size, target).is_ok());

                assert_eq!(Ok(target), memory.load(addr, size));
            }
        }
    }
}
