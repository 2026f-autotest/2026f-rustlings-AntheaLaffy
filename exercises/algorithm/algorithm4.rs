/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

#[derive(Debug)]
struct Tree<T>
where
    T: Ord,
{
    node: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Tree<T>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: Tree::new(),
            right: Tree::new(),
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: Tree::new(), }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }
    
    fn search(&self, value: T) -> bool{
        self.root.search(&value)
    }
}

impl<T> Tree<T>
where
    T: Ord,
{
    fn new() -> Self {
        Tree { node: None }
    }

    fn insert(&mut self, value: T) {
        match &mut self.node {
            None => {
                self.node = Some(Box::new(TreeNode::new(value)));
            }

            Some(current) => {
                if value < current.value {
                    current.left.insert(value);
                } else if value > current.value {
                    current.right.insert(value);
                }
            }
        }
    }

    fn search(&self, value: &T) -> bool {
        match &self.node {
            None => false,

            Some(current) => {
                if value == &current.value {
                    true
                } else if value < &current.value {
                    current.left.search(value)
                } else {
                    current.right.search(value)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root.node {
            Some(ref node) => {
                assert!(node.left.node.is_none());
                assert!(node.right.node.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


