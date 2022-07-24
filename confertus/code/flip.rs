impl DynamicBitVector {
    ...

    #[inline]
    pub fn flip(&mut self, index: usize) {
        let leaf = self.apply(Self::flip_leaf, index);
        self.update_left_values(self[leaf].parent, leaf);
    }

    #[inline]
    fn flip_leaf(&mut self, leaf: isize, index: usize) -> isize {
        self[leaf].flip(index);
        leaf
    }
}
