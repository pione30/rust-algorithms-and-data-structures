#[derive(Debug, PartialEq, Eq)]
pub struct UnionFindTree {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFindTree {
    pub fn new(nodes_num: usize) -> Self {
        let mut parent = Vec::with_capacity(nodes_num);
        let mut rank = Vec::with_capacity(nodes_num);

        for i in 0..nodes_num {
            parent.push(i);
            rank.push(0);
        }

        Self {
            parent: parent,
            rank: rank,
        }
    }

    fn find_root_id(&mut self, node_id: usize) -> usize {
        if self.parent[node_id] == node_id {
            return node_id;
        } else {
            let root_id = self.find_root_id(self.parent[node_id]);
            self.parent[node_id] = root_id;
            return root_id;
        }
    }

    pub fn unite_group(&mut self, node_id_x: usize, node_id_y: usize) {
        let root_id_x = self.find_root_id(node_id_x);
        let root_id_y = self.find_root_id(node_id_y);
        if root_id_x == root_id_y {
            return;
        }

        if self.rank[root_id_x] < self.rank[root_id_y] {
            self.parent[root_id_x] = root_id_y;
        } else {
            self.parent[root_id_y] = root_id_x;
            if self.rank[root_id_x] == self.rank[root_id_y] {
                self.rank[root_id_x] += 1;
            }
        }
    }

    pub fn same_group(&mut self, node_id_x: usize, node_id_y: usize) -> bool {
        self.find_root_id(node_id_x) == self.find_root_id(node_id_y)
    }
}

mod test {
    use super::UnionFindTree;

    #[test]
    fn it_works() {
        let mut uft = UnionFindTree::new(8);

        uft.unite_group(1, 2);
        uft.unite_group(1, 5);
        assert!(uft.same_group(1, 2));
        assert!(uft.same_group(1, 5));
        assert!(!uft.same_group(1, 3));

        uft.unite_group(6, 4);
        uft.unite_group(4, 7);
        assert!(uft.same_group(6, 4));
        assert!(uft.same_group(6, 7));
        assert!(!uft.same_group(2, 4));

        uft.unite_group(1, 7);
        assert!(uft.same_group(1, 7));
        assert!(uft.same_group(2, 4));
    }
}
