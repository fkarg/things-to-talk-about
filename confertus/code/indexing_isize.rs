/// Return [`Leaf`] for `isize` indexing
impl Index<isize> for DynamicBitVector {
    type Output = Leaf;
    fn index(&self, index: isize) -> &Self::Output {
        let uidx = if index < 0 { -index as usize
        } else { index as usize };
        &self.leafs[uidx]
    }
}

impl IndexMut<isize> for DynamicBitVector {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        let uidx = if index < 0 { -index as usize
        } else { index as usize };
        &mut self.leafs[uidx]
    }
}

