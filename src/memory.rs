use crate::error::Error;

const MEMORY_SIZE_BYTES: usize = 1024 * 1024;

// Placed on the heap as the stack will otherwise overflow. Uses a `Box`ed array rather than a `Vec`
// because it better encapsulates the idea that this is an exact, fixed amount of memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Memory(Box<[u8; MEMORY_SIZE_BYTES]>);

impl Memory {
    /// Reads a byte from memory at the provided index. If the index is out-of-bounds, then an
    /// `Err` is returned.
    pub fn read8(&self, index: usize) -> Result<u8, Error> {
        match self.0.get(index) {
            Some(n) => Ok(*n),
            None => Err(Error::InvalidMemoryAddress(format!("{index}"))),
        }
    }

    /// Reads 2 bytes from memory starting from the provided index, in little-endian format. If an
    /// out-of-bounds area of memory is being read, then an `Err` is returned.
    pub fn read16(&self, index: usize) -> Result<u16, Error> {
        let Some(n) = self.0.get(index) else {
            return Err(Error::InvalidMemoryAddress(format!("{index}")));
        };
        let mut result = *n as u16;

        let Some(n) = self.0.get(index + 1) else {
            return Err(Error::InvalidMemoryAddress(format!("{}", index + 1)));
        };
        result |= (*n as u16) << 8;

        Ok(result)
    }

    /// Reads 4 bytes from memory starting from the provided index, in little-endian format. If an
    /// out-of-bounds area of memory is being read, an error is returned.
    pub fn read32(&self, index: usize) -> Result<u32, Error> {
        let mut result = 0;

        for i in 0..4 {
            let Some(n) = self.0.get(index + i) else {
                return Err(Error::InvalidMemoryAddress(format!("reading 4 bytes went out-of-bounds at {}", index + i)));
            };
            result |= (*n as u32) << 8 * i;
        }

        Ok(result)
    }

    /// Writes a byte into memory at the provided index. If the index is out-of-bounds, then an
    /// `Err` is returned.
    pub fn write8(&mut self, index: usize, value: u8) -> Result<(), Error> {
        if index >= MEMORY_SIZE_BYTES {
            return Err(Error::InvalidMemoryAddress(format!(
                "{index} is out-of-bounds"
            )));
        }
        self.0[index] = value;
        Ok(())
    }

    /// Writes 2 bytes into memory starting at the provided index, in little-endian format. If an
    /// out-of-bounds area of memory is accessed, then an `Err` is returned.
    pub fn write16(&mut self, index: usize, value: u16) -> Result<(), Error> {
        if index + 1 >= MEMORY_SIZE_BYTES {
            return Err(Error::InvalidMemoryAddress(format!(
                "writing 2 bytes starting at {index} would go out-of-bounds"
            )));
        }
        for i in 0..2 {
            self.0[index + i] = (value >> 8 * i) as u8;
        }
        Ok(())
    }

    /// Writes 4 bytes into memory starting at the provided index, in little-endian format. If an
    /// out-of-bounds area of memory is accessed, then an `Err` is returned.
    pub fn write32(&mut self, index: usize, value: u32) -> Result<(), Error> {
        if index + 3 >= MEMORY_SIZE_BYTES {
            return Err(Error::InvalidMemoryAddress(format!(
                "writing 4 bytes starting at {index} would go out-of-bounds"
            )));
        }
        for i in 0..4 {
            self.0[index + i] = (value >> 8 * i) as u8;
        }
        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self(Box::new([0; MEMORY_SIZE_BYTES]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_up_memory() -> Memory {
        let mut memory = Memory::default();
        for i in 0..10 {
            memory.0[i] = i as u8;
        }
        memory
    }

    #[test]
    fn read8() {
        let memory = set_up_memory();
        assert_eq!(memory.read8(0).unwrap(), 0);
        assert_eq!(memory.read8(1).unwrap(), 1);
        assert_eq!(memory.read8(11).unwrap(), 0);
        assert!(memory.read8(MEMORY_SIZE_BYTES + 1).is_err());
    }

    #[test]
    fn read16() {
        let memory = set_up_memory();
        assert_eq!(memory.read16(0).unwrap(), 0x100);
        assert_eq!(memory.read16(1).unwrap(), 0x201);
        assert_eq!(memory.read16(11).unwrap(), 0);
        assert!(memory.read16(MEMORY_SIZE_BYTES).is_err());
        assert!(memory.read16(MEMORY_SIZE_BYTES + 1).is_err());
    }

    #[test]
    fn read32() {
        let memory = set_up_memory();
        assert_eq!(memory.read32(0).unwrap(), 0x3020100);
        assert_eq!(memory.read32(1).unwrap(), 0x4030201);
        assert_eq!(memory.read32(11).unwrap(), 0);
        assert!(memory.read32(MEMORY_SIZE_BYTES - 1).is_err());
        assert!(memory.read32(MEMORY_SIZE_BYTES).is_err());
        assert!(memory.read32(MEMORY_SIZE_BYTES + 1).is_err());
    }

    #[test]
    fn write8() {
        let mut memory = Memory::default();
        assert!(memory.write8(1, 1).is_ok());
        assert_eq!(memory.0[0], 0);
        assert_eq!(memory.0[1], 1);
        assert_eq!(memory.0[2], 0);
        assert!(memory.write8(MEMORY_SIZE_BYTES, 0).is_err());
    }

    #[test]
    fn write16() {
        let mut memory = Memory::default();
        assert!(memory.write16(1, 0x201).is_ok());
        assert_eq!(memory.0[0], 0);
        assert_eq!(memory.0[1], 1);
        assert_eq!(memory.0[2], 2);
        assert_eq!(memory.0[3], 0);
        assert!(memory.write16(MEMORY_SIZE_BYTES - 1, 0).is_err());
        assert!(memory.write16(MEMORY_SIZE_BYTES, 0).is_err());
    }

    #[test]
    fn write32() {
        let mut memory = Memory::default();
        assert!(memory.write32(1, 0x4030201).is_ok());
        assert_eq!(memory.0[0], 0);
        assert_eq!(memory.0[1], 1);
        assert_eq!(memory.0[2], 2);
        assert_eq!(memory.0[3], 3);
        assert_eq!(memory.0[4], 4);
        assert_eq!(memory.0[5], 0);
        assert!(memory.write32(MEMORY_SIZE_BYTES - 2, 0).is_err());
        assert!(memory.write32(MEMORY_SIZE_BYTES - 1, 0).is_err());
        assert!(memory.write32(MEMORY_SIZE_BYTES, 0).is_err());
    }
}
