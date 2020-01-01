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
            Self::add_value_into_one_side(&mut self.left, new_val);
        } else {
            Self::add_value_into_one_side(&mut self.right, new_val);
        }
    }

    fn add_value_into_one_side(branch: &mut Option<Box<BinarySearchTree<T>>>, new_val: T) {
        match branch {
            Some(bst) => {
                bst.add(new_val);
            }
            None => {
                *branch = Some(Box::new(BinarySearchTree::new(new_val)));
            }
        }
    }

    pub fn contains(&self, val: &T) -> bool {
        if self.val == *val {
            true
        } else if *val < self.val && self.left.is_some() {
            self.left
                .as_ref()
                .expect("self.left should be Some")
                .contains(val)
        } else if self.val < *val && self.right.is_some() {
            self.right
                .as_ref()
                .expect("self.right should be Some")
                .contains(val)
        } else {
            false
        }
    }

    pub fn delete(&mut self, val: &T) {
        if self.val == *val {
            // such a node should not be deleted
            return;
        }
        let next_node = if *val < self.val {
            &mut self.left
        } else {
            &mut self.right
        };

        if let Some(bst) = next_node {
            if bst.val == *val {
                match bst.left.as_mut() {
                    None => {
                        *next_node = bst.right.take();
                    }
                    Some(bst_left) => match bst_left.right.as_mut() {
                        None => {
                            bst_left.right = bst.right.take();
                            *next_node = bst.left.take();
                        }
                        Some(_) => {
                            let mut aiming_node = bst_left;

                            while let Some(right) = aiming_node.right.as_mut() {
                                if right.right.is_none() {
                                    let temp_right = right.left.take();

                                    right.left = bst.left.take();
                                    right.right = bst.right.take();
                                    *next_node = aiming_node.right.take();

                                    aiming_node.right = temp_right;

                                    break;
                                }
                                aiming_node = right;
                            }
                        }
                    },
                }
            } else {
                bst.delete(val);
            }
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
        assert_eq!(bst.contains(&5), true);
        assert_eq!(bst.contains(&10), true);
        assert_eq!(bst.contains(&9), true);
        assert_eq!(bst.contains(&15), true);
        assert_eq!(bst.contains(&16), false);
    }
}
