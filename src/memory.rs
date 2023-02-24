use crate::error::Error;

const MEMORY_SIZE_BYTES: usize = 1024 * 1024;

// Placed on the heap as the stack will otherwise overflow. Uses a `Box`ed array rather than a `Vec`
// because it better encapsulates the idea that this is an exact, fixed amount of memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Memory(Box<[u8; MEMORY_SIZE_BYTES]>);

impl Memory {
    pub fn read8(&self, index: usize) -> Result<u8, Error> {
        match self.0.get(index) {
            Some(n) => Ok(*n),
            None => Err(Error::InvalidMemoryAddress(format!("{index}").into()))
        }
    }

    // TODO: Test
    pub fn read16(&self, index: usize) -> Result<u16, Error> {
        let Some(n) = self.0.get(index) else {
            return Err(Error::InvalidMemoryAddress(format!("{index}").into()));
        };
        let mut result = *n as u16;

        let Some(n) = self.0.get(index + 1) else {
            return Err(Error::InvalidMemoryAddress(format!("{}", index + 1).into()));
        };
        result |= (*n as u16) << 8;

        Ok(result)
    }

    pub fn read32(&self, index: usize) -> Result<u32, Error> {
        let Some(n) = self.0.get(index) else {
            return Err(Error::InvalidMemoryAddress(format!("{index}").into()));
        };
        let mut result = *n as u32;

        let Some(n) = self.0.get(index + 1) else {
            return Err(Error::InvalidMemoryAddress(format!("{}", index + 1).into()));
        };
        result |= (*n as u32) << 8;

        let Some(n) = self.0.get(index + 2) else {
            return Err(Error::InvalidMemoryAddress(format!("{}", index + 2).into()));
        };
        result |= (*n as u32) << 16;

        let Some(n) = self.0.get(index + 3) else {
            return Err(Error::InvalidMemoryAddress(format!("{}", index + 3).into()));
        };
        result |= (*n as u32) << 24;

        Ok(result)
    }

    pub fn write8(&self, index: usize) -> Result<(), Error> {todo!()}

    pub fn write16(&self, index: usize) -> Result<(), Error> {todo!()}

    pub fn write32(&self, index: usize) -> Result<(), Error> {todo!()}
}

impl Default for Memory {
    fn default() -> Self {
        Self(Box::new([0; MEMORY_SIZE_BYTES]))
    }
}
