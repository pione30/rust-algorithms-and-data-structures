#[derive(Debug, PartialEq, Eq)]
pub struct BinarySearchTree<T: Ord> {
    val: T,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, new_val: T) {
        if new_val <= self.val {
            match &mut self.left {
                None => {
                    self.left = Some(Box::new(BinarySearchTree::new(new_val)));
                }
                Some(box_bst) => {
                    box_bst.add(new_val);
                }
            }
        } else {
            match &mut self.right {
                None => {
                    self.right = Some(Box::new(BinarySearchTree::new(new_val)));
                }
                Some(box_bst) => {
                    box_bst.add(new_val);
                }
            }
        }
    }

    pub fn contains(&self, val: &T) -> bool {
        if self.val == *val {
            true
        } else {
            if let Some(bst) = &self.left {
                if bst.contains(val) {
                    return true;
                }
            };

            if let Some(bst) = &self.right {
                if bst.contains(val) {
                    return true;
                }
            };

            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_in_same_order() {
        let mut bst1 = BinarySearchTree::new(1);
        bst1.add(1);
        bst1.add(2);

        let mut bst2 = BinarySearchTree::new(1);
        bst2.add(1);
        bst2.add(2);

        assert_eq!(bst1, bst2);
    }

    #[test]
    fn add_in_different_order() {
        let mut bst1 = BinarySearchTree::new(8);
        bst1.add(5);
        bst1.add(10);
        bst1.add(5);
        bst1.add(3);
        bst1.add(5);
        bst1.add(6);
        bst1.add(8);
        bst1.add(9);
        bst1.add(15);

        let mut bst2 = BinarySearchTree::new(8);
        bst2.add(10);
        bst2.add(5);
        bst2.add(15);
        bst2.add(9);
        bst2.add(6);
        bst2.add(5);
        bst2.add(8);
        bst2.add(3);
        bst2.add(5);

        assert_eq!(bst1, bst2);
    }

    #[test]
    fn contains() {
        let mut bst = BinarySearchTree::new(8);
        bst.add(5);
        bst.add(10);
        bst.add(5);
        bst.add(3);
        bst.add(5);
        bst.add(6);
        bst.add(8);
        bst.add(9);
        bst.add(15);

        assert_eq!(bst.contains(&0), false);
        assert_eq!(bst.contains(&5), true);
        assert_eq!(bst.contains(&10), true);
        assert_eq!(bst.contains(&9), true);
        assert_eq!(bst.contains(&15), true);
        assert_eq!(bst.contains(&16), false);
    }
}
