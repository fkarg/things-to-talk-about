/// Functions associated with dynamic bit vectors.
pub trait DynBitVec: StaticBitVec {
    /// Insert `bit` at position `index` in underlying container
    fn insert(&mut self, index: usize, bit: bool) -> Result<(), &'static str>;

    /// Remove bit value at position `index`
    fn delete(&mut self, index: usize) -> Result<(), &'static str>;

    /// Flip bit at position `index`, updates `ones` and `num` values accordingly
    fn flip(&mut self, index: usize);

    /// Return used capacity of underlying container
    fn nums(&self) -> usize;
}
