use float_ord::FloatOrd;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub struct Node {
    pub idx: usize,
    pub cost: f64,
    pub path_length: i32,
}

impl Node {
    pub fn new(idx: usize, cost: f64, path_length: i32) -> Node {
        Node {
            idx: idx,
            cost: cost,
            path_length: path_length,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        FloatOrd(self.cost).cmp(&FloatOrd(other.cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        FloatOrd(self.cost) == FloatOrd(other.cost)
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
        FloatOrd(self.cost).hash(state);
        self.path_length.hash(state);
    }
}
