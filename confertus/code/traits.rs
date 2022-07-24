/// Functions associated with static bit vectors. ...
pub trait StaticBitVec {
    ...

    /// Return number of on-bits in container or on left half for tree-based elements
    fn ones(&self) -> usize;

    /// Access bit value at position `index`
    fn access(&self, index: usize) -> bool;

    /// Returns number of `bit`-values up to `index` in container
    fn rank(&self, bit: bool, index: usize) -> usize;

    /// Return index of `n`-th `bit`-value in container
    fn select(&self, bit: bool, n: usize) -> usize;
}
