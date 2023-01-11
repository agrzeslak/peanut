const RAM_SIZE_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Ram([u8; RAM_SIZE_BYTES]);

impl Default for Ram {
    fn default() -> Self {
        Self([0; RAM_SIZE_BYTES])
    }
}
