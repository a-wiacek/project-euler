use std::cmp::Ordering;

// Find union data structure. Vertices are indexed from 0.

#[derive(Clone)]
struct Node {
    parent: Option<usize>,
    rank: usize,
}

impl Node {
    fn new() -> Node {
        Node {
            parent: None,
            rank: 0,
        }
    }
}

pub struct FindUnion {
    nodes: Vec<Node>,
}

impl FindUnion {
    pub fn new(size: usize) -> FindUnion {
        FindUnion {
            nodes: vec![Node::new(); size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        match self.nodes[x].parent {
            None => x,
            Some(parent) => {
                let root = self.find(parent);
                self.nodes[x].parent = Some(root);
                root
            }
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        let rank_x = self.nodes[x].rank;
        let rank_y = self.nodes[y].rank;
        match rank_x.cmp(&rank_y) {
            Ordering::Less => self.nodes[root_x].parent = Some(root_y),
            Ordering::Equal => {
                if root_x != root_y {
                    self.nodes[root_y].parent = Some(root_x);
                    self.nodes[root_x].rank += 1;
                }
            }
            Ordering::Greater => self.nodes[root_y].parent = Some(root_x),
        }
    }
}
