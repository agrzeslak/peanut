const MEMORY_SIZE_BYTES: usize = 1024 * 1024;

// Placed on the heap as the stack will otherwise overflow. Uses a `Box`ed array rather than a `Vec`
// because it better encapsulates the idea that this is an exact, fixed amount of memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Memory(Box<[u8; MEMORY_SIZE_BYTES]>);

impl Default for Memory {
    fn default() -> Self {
        Self(Box::new([0; MEMORY_SIZE_BYTES]))
    }
}
