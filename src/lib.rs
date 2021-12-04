#![allow(dead_code, unused_macros, unused_variables)]
#![feature(box_into_inner)]

use std::cmp::PartialEq;
use std::fmt::Debug;

#[cfg(test)]
#[macro_use]
mod ps;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
struct Tree<T> {
    node: T,
    l_child: Option<Box<Tree<T>>>,
    r_child: Option<Box<Tree<T>>>,
}

impl<T> Tree<T> {
    /// Create a new tree, based on T's default value
    /// if T does not implement Default, use `Tree::new` instead.
    pub fn default() -> Self
    where
        T: Default,
    {
        Self {
            node: Default::default(),
            l_child: None,
            r_child: None,
        }
    }

    /// Create a new tree with a root node value
    pub fn new(node: T) -> Self {
        Self {
            node,
            l_child: None,
            r_child: None,
        }
    }
    /// Check if the left child is set.
    pub fn has_left(&self) -> bool {
        self.l_child.is_some()
    }

    /// Check if the right child is set.
    pub fn has_right(&self) -> bool {
        self.r_child.is_some()
    }

    /// Check if either the left or right children are set
    pub fn has_children(&self) -> bool {
        self.has_right() || self.has_left()
    }

    /// Get the left child tree. If the left child hasn't been set,
    /// `None` is returned instead
    pub fn get_left(&self) -> Option<Tree<T>>
    where
        T: Clone,
        Option<Box<Tree<T>>>: Clone,
    {
        self.l_child.clone().map(|bt| *bt)
    }

    /// Get the right child tree. If the right child hasn't been set,
    /// `None` is returned instead
    pub fn get_right(&self) -> Option<Tree<T>>
    where
        T: Clone,
        Option<Box<Tree<T>>>: Clone,
    {
        self.r_child.clone().map(|bt| *bt)
    }
    /// Push a new node to the leftmost postition in the tree.
    pub fn push_left(&mut self, node: T)
    where
        T: Clone,
    {
        if let Some(mut l_tree) = self.get_left() {
            l_tree.push_left(node)
        } else {
            self.l_child = Some(Box::new(Tree::new(node)))
        }
    }

    /// Push a new node to the rightmost postition in the tree.
    pub fn push_right(&mut self, node: T)
    where
        T: Clone,
    {
        if let Some(mut r_tree) = self.get_right() {
            r_tree.push_right(node)
        } else {
            self.r_child = Some(Box::new(Tree::new(node)))
        }
    }
    /// Push a value to the right, and leftmost postitions in the tree.  
    pub fn push_left_right(&mut self, left_node: T, right_node: T)
    where
        T: Clone + Debug + PartialEq + From<f64>,
    {
        match (self.l_child.as_mut(), self.r_child.as_mut()) {
            (Some(l), Some(r)) => {
                l.push_left(left_node);
                r.push_right(right_node);
            }
            (None, Some(r)) => {
                r.push_right(right_node);
                self.set_left(left_node)
            }
            (Some(l), None) => {
                l.push_left(left_node);
                self.set_right(right_node)
            }
            (None, None) => {
                self.set_left(left_node);
                self.set_right(right_node)
            }
        };
    }

    /// Set the left child of a tree.
    pub fn set_left(&mut self, node: T)
    where
        T: Clone,
    {
        if let Some(mut l_child) = self.get_left() {
            l_child.node = node;
        } else {
            self.l_child = Some(Box::new(Tree::new(node)));
        }
    }

    /// Set the right child of a tree.
    pub fn set_right(&mut self, node: T)
    where
        T: Clone,
    {
        if let Some(mut r_child) = self.get_right() {
            r_child.node = node;
        } else {
            self.r_child = Some(Box::new(Tree::new(node)));
        }
    }

    /// Set the left tree.
    pub fn set_left_subtree(&mut self, new_l_child: Tree<T>) {
        self.l_child = Some(Box::new(new_l_child));
    }

    /// Set the right tree.
    pub fn set_right_subtree(&mut self, new_r_child: Tree<T>) {
        self.r_child = Some(Box::new(new_r_child));
    }

    /// Perform inorder traversal of a tree
    pub fn inorder_traversal(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut nodes: Vec<Option<T>> = Vec::new();
        let mut l_subset: Vec<Option<T>> = match self.get_left() {
            Some(l_tree) => {
                let l_tree_nodes: Vec<Option<T>> = l_tree
                    .inorder_traversal()
                    .iter()
                    .map(|x| Some(x.clone()))
                    .collect::<Vec<Option<T>>>();
                l_tree_nodes
            }
            None => (0..1).map(|_| None).collect::<Vec<Option<T>>>(),
        };

        let mut r_subset: Vec<Option<T>> = match self.get_right() {
            Some(r_tree) => {
                let r_tree_nodes: Vec<Option<T>> = r_tree
                    .inorder_traversal()
                    .iter()
                    .map(|x| Some(x.clone()))
                    .collect::<Vec<Option<T>>>();
                r_tree_nodes
            }
            None => (0..1).map(|_| None).collect::<Vec<Option<T>>>(),
        };

        nodes.append(&mut l_subset);
        nodes.push(Some(self.node.clone()));
        nodes.append(&mut r_subset);

        nodes
            .iter()
            .filter(|x| x.is_some())
            .map(|node| node.as_ref().unwrap().clone())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::Tree;

    #[test]
    fn default_init_test() {
        let tree: Tree<f64> = Tree::default();
        pretty_assert_eq!(
            tree,
            Tree {
                node: 0.0,
                l_child: None,
                r_child: None,
            }
        )
    }

    #[test]
    fn new_init_test() {
        let tree: Tree<f64> = Tree::new(5.0);
        pretty_assert_eq!(
            tree,
            Tree {
                node: 5.0,
                l_child: None,
                r_child: None,
            }
        )
    }

    #[test]
    fn push_left_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        pretty_assert_eq!(tree.get_left(), None);
        tree.push_left(50.0);
        pretty_assert_eq!(tree.get_left(), Some(Tree::new(50.0)));
    }

    #[test]
    fn push_right_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        pretty_assert_eq!(tree.get_right(), None);
        tree.push_right(50.0);
        pretty_assert_eq!(tree.get_right(), Some(Tree::new(50.0)));
    }

    #[test]
    fn push_left_right_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        tree.push_left_right(50.0, 2.0);

        pretty_assert_eq!(
            tree,
            Tree {
                node: 100.0,
                l_child: Some(Box::new(Tree::new(50.0))),
                r_child: Some(Box::new(Tree::new(2.0)))
            }
        );

        match tree.get_left() {
            Some(mut l_tree) => {
                l_tree.push_left_right(25.0, 2.0);
                tree.set_left_subtree(l_tree);
            }
            None => unreachable!(),
        };

        pretty_assert_eq!(
            tree,
            Tree {
                node: 100.0,
                l_child: Some(Box::new(Tree {
                    node: 50.0,
                    l_child: Some(Box::new(Tree::new(25.0))),
                    r_child: Some(Box::new(Tree::new(2.0))),
                })),
                r_child: Some(Box::new(Tree::new(2.0))),
            }
        )
    }

    #[test]
    fn get_left_test() {
        let mut tree: Tree<f64> = Tree::new(100.0); 
        tree.set_left(50.0); 
        tree.set_right(2.0); 

        pretty_assert_eq!(
            tree.get_left(),
            Some(Tree::new(50.0))
        )
    }
    
    #[test]
    fn get_right_test() {
        let mut tree: Tree<f64> = Tree::new(100.0); 
        tree.set_left(50.0); 
        tree.set_right(2.0); 

        pretty_assert_eq!(
            tree.get_right(),
            Some(Tree::new(2.0))
        )
    }

    #[test]
    fn inorder_traversal_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);

        pretty_assert_eq!(tree.inorder_traversal(), vec![100.0]);
        tree.push_left_right(50.0, 2.0);
        pretty_assert_eq!(tree.inorder_traversal(), vec![50.0, 100.0, 2.0]);
        match tree.get_left() {
            Some(left_child) => {
                let mut _left_child = left_child;

                _left_child.push_left_right(25.0, 2.0);
                tree.set_left_subtree(_left_child);
            }
            None => unreachable!(),
        };
        pretty_assert_eq!(tree.inorder_traversal(), vec![25.0, 50.0, 2.0, 100.0, 2.0]);
        match tree.get_left() {
            Some(left_child) => {
                let mut _left_child = left_child;

                match _left_child.get_left() {
                    Some(left_sub_child) => {
                        let mut _left_sub_child = left_sub_child;

                        _left_sub_child.push_left_right(5.0, 5.0);

                        _left_child.set_left_subtree(_left_sub_child);
                    }
                    None => unreachable!(),
                };

                tree.set_left_subtree(_left_child);
            }
            None => unreachable!(),
        };
        pretty_assert_eq!(
            tree.inorder_traversal(),
            vec![5.0, 25.0, 5.0, 50.0, 2.0, 100.0, 2.0]
        );
    }

    #[test]
    fn set_left_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        tree.set_left(50.0);

        pretty_assert_eq!(tree, Tree { 
            node: 100.0,
            l_child: Some(Box::new(Tree {
                node: 50.0,
                l_child: None, 
                r_child: None,
            })),
            r_child: None,
        })
    }
    
    #[test]
    fn set_right_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        tree.set_right(2.0);

        pretty_assert_eq!(tree, Tree { 
            node: 100.0,
            l_child: None,
            r_child: Some(Box::new(Tree {
                node: 2.0,
                l_child: None,
                r_child: None,
            })),
        })
    }

    #[test]
    fn set_left_subtree_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        let mut left_child: Tree<f64> = Tree::new(50.0);

        left_child.push_left_right(25.0, 2.0);

        tree.set_left_subtree(left_child);
        tree.push_right(2.0);

        pretty_assert_eq!(
            tree,
            Tree {
                node: 100.0, 
                l_child: Some(Box::new(Tree { 
                    node: 50.0,
                    l_child: Some(Box::new(Tree::new(25.0))),
                    r_child: Some(Box::new(Tree::new(2.0))),
                })),
                r_child: Some(Box::new(Tree { 
                    node: 2.0,
                    l_child: None,
                    r_child: None
                })),
            }
        )
    }

    #[test]
    fn set_right_subtree_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        let mut right_child: Tree<f64> = Tree::new(150.0);

        tree.push_left(50.0);

        pretty_assert_eq!(
            tree,
            Tree {
                node: 100.0,
                l_child: Some(Box::new(Tree::new(50.0))),
                r_child: None, 
            }
        );

        right_child.push_left_right(125.0, 200.0); 
        tree.set_right_subtree(right_child);

        pretty_assert_eq!(
            tree,
            Tree {
                node: 100.0,
                l_child: Some(Box::new(Tree::new(50.0))),
                r_child: Some(Box::new(Tree { 
                    node: 150.0,
                    l_child: Some(Box::new(Tree::new(125.0))),
                    r_child: Some(Box::new(Tree::new(200.0)))
                })),
            }
        )
    }

    #[test]
    fn has_children_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        tree.push_left_right(50.0, 2.0);

        pretty_assert!(tree.has_children())
    }

    #[test]
    fn has_left_child_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        tree.push_left(50.0);

        pretty_assert!(tree.has_left())
    }
    
    #[test]
    fn has_right_child_test() {
        let mut tree: Tree<f64> = Tree::new(100.0);
        tree.push_right(2.0);

        pretty_assert!(tree.has_right())
    }
}
