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
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        // if self.root.is_none() { self.root = Some(Box::new(TreeNode::new(value))); return; }
        // let mut cur = &self.root;
        // while cur.is_some() {
        //     match &value {
        //         value if value > &cur.unwrap().value => {
        //             if cur.unwrap().right.is_some() { cur = &cur.unwrap().right }
        //             else { cur.unwrap().right = Some(Box::new(TreeNode::new(value))); return; }
        //         },
        //         _ => {
        //             if cur.unwrap().left.is_some() { cur = &cur.unwrap().left }
        //             else { cur.unwrap().left = Some(Box::new(TreeNode::new(value))); return; }
        //         },
        //     }
        // }
        // TODO: need to clearly understand every part
        self.root = Self::insert_recursive(self.root.take(), value);
    }

    fn insert_recursive(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => Some(Box::new(TreeNode::new(value))),
            Some(mut node) => {
                if value < node.value {
                    node.left = Self::insert_recursive(node.left.take(), value);
                } else if value > node.value {
                    node.right = Self::insert_recursive(node.right.take(), value);
                }
                Some(node)
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        Self::search_recursive(&self.root, &value)
    }

    fn search_recursive(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(node) => {
                if *value == node.value { return true; }
                return Self::search_recursive(&node.left, &value) || Self::search_recursive(&node.right, &value)
            }
        }
    }
}

// impl<T> TreeNode<T>
// where
//     T: Ord,
// {
//     // Insert a node into the tree
//     fn insert(&mut self, value: T) {
//         //
//     }
// }


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

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


