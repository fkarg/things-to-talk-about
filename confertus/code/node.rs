pub struct Node {
    /// index of parent Node
    pub parent: Option<usize>, // 8 bytes + 1bit
    /// left side subtree child index
    pub left: Option<isize>, // 8 bytes + 1bit
    /// right side subtree child index
    pub right: Option<isize>, // 8 bytes + 1bit
    /// number of filled bits on the left subtree
    pub nums: usize, // 8 bytes
    /// number of ones on the left subtree
    pub ones: usize, // 8 bytes
    /// difference of height between left and
    /// right subtree. Valid values are -1, 0, 1
    pub rank: i8, // 1 byte
}
