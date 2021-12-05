#![allow(unused_attributes, unused_imports, unused_mut)]

use std::cmp::PartialOrd;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::Tree; 

#[cfg(test)]
#[macro_use]
use crate::macros::*;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BinarySearchTree<'a, T> {
    node: T, 
    l_child: Option<Box<BinarySearchTree<'a, T>>>,
    r_child: Option<Box<BinarySearchTree<'a, T>>>,
    _phantom: PhantomData<&'a T>
}

impl<'a, T> BinarySearchTree<'a, T> 
where
    T: PartialOrd
{
    pub fn new(node: T) -> Self {
        Self {
            node,
            l_child: None, 
            r_child: None,
            _phantom: PhantomData
        }
    }

    fn set_empty_left_child(&mut self, new_left_child: BinarySearchTree<'a, T>) {
        if let None = self.l_child {
            self.l_child = Some(Box::new(new_left_child));
        } else {
            panic!("Attempted to set left child despite it already being set");
        }
    }
    
    fn set_empty_right_child(&mut self, new_right_child: BinarySearchTree<'a, T>) {
        if let None = self.r_child {
            self.r_child = Some(Box::new(new_right_child));
        } else {
            panic!("Attempted to set right child despite it already being set");
        }
    }

    fn reset_left_child(&'a mut self, new_left_child: &'a BinarySearchTree<'a, T>) {
        let new_left_child = BinarySearchTree {
            node: *new_left_child.get_node(),
            l_child: new_left_child.get_left_tree().map(|x| Box::new(*x)),
            r_child: new_left_child.get_right_tree().map(|x| Box::new(*x)),
            _phantom: PhantomData
        };

        self.l_child = Some(Box::new(new_left_child));
    }

    fn reset_right_child(&'a mut self, new_right_child: &'a BinarySearchTree<'a, T>) {
        let new_right_child = BinarySearchTree {
            node: *new_right_child.get_node(),
            l_child: new_right_child.get_left_tree().map(|x| Box::new(*x)),
            r_child: new_right_child.get_right_tree().map(|x| Box::new(*x)),
            _phantom: PhantomData
        };

        self.r_child = Some(Box::new(new_right_child));
    }

    pub fn get_node(&'a self) -> &'a T {
        &self.node
    }
    
    pub fn get_node_mut(&'a mut self) -> &'a mut T {
        &mut self.node
    }

    pub fn push(&'a mut self, node: &'a T) -> &'a T 
    where
        T: Clone
    {
        match (self.get_left_tree(), self.get_right_tree()) {
            (Some(l), Some(r)) => {
                if *node > *self.get_node() {
                    let inserted: &'a T = r.push(node);
                    self.reset_right_child(&r); 
                    return inserted;
                } else {
                    let inserted: &'a T = l.push(node);
                    self.reset_left_child(&l); 
                    return inserted;
                }
            },
            (Some(l), None) => {
                if *node > *self.get_node() {
                    self.set_empty_right_child(BinarySearchTree::new(*node));
                    return self.get_right().unwrap();
                } else {
                    let inserted: &'a T = l.push(node);
                    self.reset_left_child(&l);
                    return inserted;
                }
            },
            (None, Some(r)) => {
                if *node < *self.get_node() {
                    self.set_empty_left_child(BinarySearchTree::new(*node));
                    return self.get_left().unwrap();
                } else {
                    let mut new_right: BinarySearchTree<'a, T> = *r;

                    let inserted = new_right.push(node);

                    self.reset_right_child(&new_right);

                    todo!()
                }
            },
            (None, None) => {
                if *node > *self.get_node() {
                    self.set_empty_left_child(BinarySearchTree {
                        node: *node,
                        l_child: None,
                        r_child: None,
                        _phantom: PhantomData
                    });
                    return self.get_left().unwrap();
                } else {
                    self.set_empty_right_child(BinarySearchTree {
                        node: *node,
                        l_child: None,
                        r_child: None,
                        _phantom: PhantomData
                    });
                    return self.get_right().unwrap();
                }
            },
        };
    }

    pub fn push_mut(&'a mut self, node: &'a mut T) -> &'a T {
        todo!()
    }

    pub fn push_pair(
        &'a mut self,
        left_node: &'a T,
        right_node: &'a T
    ) -> (&'a T, &'a T) {
        todo!() 
    }
    pub fn push_pair_mut(
        &'a mut self,
        left_node: &'a mut T,
        right_node: &'a mut T
    ) -> (&'a mut T, &'a mut T) {
        todo!()
    }

    pub fn get_left(&'a self) -> Option<&'a T> {
        todo!()
    }

    pub fn get_left_tree(&'a self) -> Option<&'a Self> {
        if self.l_child.is_some() {
            self.l_child.as_ref().map(|l| l.deref())
        } else {
            None
        }
    }
    
    pub fn get_left_mut(&'a mut self) -> Option<&'a mut T> {
        if let Some(l_child) = &mut self.l_child {
            let mut l_child: &mut BinarySearchTree<T> = l_child;
            Some(l_child.get_node_mut())
        } else {
            None
        }
    }
    
    pub fn get_left_tree_mut(&'a mut self) -> Option<&'a mut Self> {
        if self.l_child.is_some() {
            Some(&mut *self.l_child.as_mut().unwrap())
        } else {
            None
        }
    }

    pub fn get_right(&'a self) -> Option<&'a T> {
        todo!()
    }

    pub fn get_right_tree(&'a mut self) -> Option<&'a Self> {
        if self.r_child.is_some() {
            Some(self.r_child.as_ref().unwrap())
        } else {
            None
        }
    }
    
    pub fn get_right_mut(&'a mut self) -> Option<&'a mut T> {
        todo!()
    }
}

impl<'a, T: PartialOrd + Default> Default for BinarySearchTree<'a, T> {
    fn default() -> Self {
        Self {
            node: Default::default(),
            l_child: None,
            r_child: None,
            _phantom: PhantomData
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bst::*;
    use crate::{ echo, pretty_assert, pretty_assert_eq };

    #[test]
    fn new_test() {
        let mut bst = BinarySearchTree::new(50.0); 

        pretty_assert_eq!(
            bst,
            BinarySearchTree {
                node: 50.0,
                l_child: None,
                r_child: None,
                _phantom: PhantomData
            }
        )
    }

    #[test]
    fn default_test() {
        let mut bst: BinarySearchTree<f64> = BinarySearchTree::default();

        pretty_assert_eq!(
            bst,
            BinarySearchTree::new(0.0)
        )
    }
}
