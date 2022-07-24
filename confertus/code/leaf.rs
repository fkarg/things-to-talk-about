/// Leaf element of [`DynamicBitVector`]
/// ...
#[derive(PartialEq, Clone, Default)]
pub struct Leaf {
    /// reference to parent [`Node`]
    pub parent: usize, // 8 bytes
    /// container for actual bit values
    pub value: u128, // 16 bytes
    /// number of bits used in `value`-container
    pub nums: u8, // 1 byte
    // realistically below u128::BITS,
    // so u8::MAX = 255 is sufficient
}
