#[derive(Clone, Copy, Default)]
pub struct Address(u32);
pub struct Offset {
    base: Address,
    offset: i32,
}

