use std::cmp::PartialOrd;

use crate::Tree; 

#[cfg(test)]
#[macro_use]
use crate::ps::*; 

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BinarySearchTree<T>(Tree<T>); 

impl<T> BinarySearchTree<T> {
    pub fn new(node: T) -> Self {
        Self ( Tree::new(node) )
    }

    pub fn default() -> Self 
    where
        T: Default 
    {
        Self ( Tree::default() )
    }

    pub fn push(&mut self, new_node: T) 
    where
        T: PartialOrd
    {
        match (self.get_left(), self.get_right()) {
            (None, None) => {
                if new_node < self.0.node {
                    self.set_left(new_node)
                } else {
                    self.set_right(new_node) 
                }
            },
            (Some(left), None) => {
                if new_node > self.0.node {
                    self.set_right(new_node);
                } else {
                    let mut _left = left; 

                    _left.push(new_node);

                    self.set_left_subtree(_left);
                }
            }
            (None, Some(right)) => {
                if new_node < self.0.node {
                    self.set_left(new_node);
                } else {
                    let mut _right = right; 

                    _right.push(new_node);

                    self.set_right_subtree(_right);
                }
            },
            (Some(left), Some(right)) => {
                let (_left, _right) = (left, right); 
                if new_node > self.0.node {
                    self.set_right_subtree(_right);
                } else {
                    self.set_left_subtree(_left);
                } 
            },
        };      
    }
    /* 
     * TODO: Re-write the following accessors and mutators to implement their
     *       own logic instead of acting as a wrapper for the `Tree<T>`
     */
    pub fn get_left(&self) -> Option<Tree<T>> 
    where 
        T: Clone
    {
        self.0.get_left()
    }

    pub fn get_right(&self) -> Option<Tree<T>> 
    where 
        T: Clone
    {
        self.0.get_right()
    }

    pub fn set_left_subtree(&mut self, new_left: Tree<T>) {
        self.0.set_left_subtree(new_left)
    }
    
    pub fn set_right_subtree(&mut self, new_right: Tree<T>) {
        self.0.set_right_subtree(new_right)
    }

    pub fn set_left(&mut self, new_right: T) 
    where
        T: Clone
    {
        self.0.set_left(new_right)
    }
    
    pub fn set_right(&mut self, new_right: T) 
    where
        T: Clone
    {
        self.0.set_right(new_right)
    }
}

#[cfg(test)]
mod test {
    use crate::bst::*;
    use crate::pretty_assert_eq;
    use crate::pretty_assert;
    #[test]
    fn new_test() {
        let mut bst = BinarySearchTree::new(50.0); 

        pretty_assert_eq!(
            bst,
            BinarySearchTree(Tree {
                node: 50.0,
                l_child: None,
                r_child: None
            })
        )
    }

    #[test]
    fn default_test() {
        let mut bst: BinarySearchTree<f64> = BinarySearchTree::default();

        pretty_assert_eq!(
            bst,
            BinarySearchTree(Tree::new(0.0))
        )
    }

    #[test]
    fn push_test() {
        let mut bst: BinarySearchTree<f64> = BinarySearchTree::new(8.0);
        bst.push(3.0);
        bst.push(1.0);
        bst.push(6.0);
        bst.push(4.0);
        bst.push(7.0);
        bst.push(10.0);
        bst.push(14.0);
        bst.push(13.0);

        pretty_assert_eq!(bst, BinarySearchTree (Tree {
            node: 8.0,
            l_child: Some(Box::new(Tree {
                node: 3.0,
                l_child: Some(Box::new(Tree::new(1.0))),
                r_child: Some(Box::new(Tree {
                    node: 6.0, 
                    l_child: Some(Box::new(Tree::new(4.0))),
                    r_child: Some(Box::new(Tree::new(7.0))),
                }))
            })),
            r_child: Some(Box::new(Tree {
                node: 10.0,
                l_child: None,
                r_child: Some(Box::new(Tree::new(13.0)))
            })),
        }))
    }
}
