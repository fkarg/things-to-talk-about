/// Implementation of Dynamic Bit Vector based on
/// a self-balancing [AVL tree]. ...
#[derive(Debug, PartialEq, Clone, Default)]
pub struct DynamicBitVector {
    /// index to root [`Node`]
    pub root: usize, // 8 bytes
    // positively indexed, usize
    /// Vector containing [`Node`]
    pub nodes: Vec<Node>, // 24 bytes
    // negatively indexed, isize
    /// Vector containing [`Leaf`]
    pub leafs: Vec<Leaf>, // 24 bytes
}
