pub fn apply<T>(&mut self, mut f: FnMut(...) -> T, index: usize) -> T {
    self.apply_node(self.root, f, index)
}
fn apply_node<T>(&mut self, node: usize, mut f: FnMut(...) -> T, index: usize) -> T {
    if self[node].nums <= index {
        let right_id = self[node].right.unwrap();
        if right_id >= 0 {
            self.apply_node(right_id as usize, f, index - self[node].nums)
        } else { 
            f(self, right_id, index - self[node].nums)
        }
    } else {
        let left_id = self[node].left.unwrap();
        if left_id >= 0 {
            self.apply_node(left_id as usize, f, index)
        } else { 
            f(self, left_id, index) 
        }
    }
}
