use std::{
    cell::RefCell, cmp::{PartialEq, PartialOrd}, fmt::{Debug, Error}, rc::Rc
        };

// types used for the link
type Link<T> = Rc<RefCell<Node<T>>>;

// private basic struct of a node
#[derive(Debug)]
struct Node<T: Clone + PartialOrd + PartialEq>
{
    data: T,
    left: Option<Link<T>>,
    right: Option<Link<T>>
}

// implementation of Node to create a new one
impl<T: Clone + PartialOrd + PartialEq> Node<T>
{
    pub fn new(data: T) -> Self
    {
        Node { data, left: None, right: None}
    }
}

impl<T: Clone + PartialOrd + PartialEq>  PartialEq for Node<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }

    fn ne(&self, other: &Self) -> bool {
        self.data != other.data
    }
}

impl<T: Clone + PartialOrd> PartialOrd for Node<T>
{
    fn lt(&self, other: &Self) -> bool {
        self.data < other.data
    }

    fn le(&self, other: &Self) -> bool {
        self.data <= other.data
    }

    fn gt(&self, other: &Self) -> bool {
        self.data > other.data
    }

    fn ge(&self, other: &Self) -> bool {
        self.data >= other.data
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

// binary tree
#[derive(Debug)]
pub struct Tree<T: Clone + PartialOrd + PartialEq>
{
    root: Option<Link<T>>,
    size: u32
}

// api of the tree
impl<T: Clone + PartialOrd + PartialEq> Tree<T>
{
    // creates an empty tree
    pub fn new() -> Tree<T>
    {
        Tree{root: None, size: 0}
    }

    pub fn size(&self) -> u32
    {
        self.size
    }

    /// searches the tree for the data passed in
    /// and returns a boolean representing whether
    /// the data exists in the tree or not
    pub fn search(&self, data: T) -> bool
    {
        match &self.root {
            None => false,
            Some(_) => Self::search_r(&self.root, data)
        }
    }

    /// recursive function to search the tree and 
    /// find a value
    fn search_r(node: &Option<Link<T>>, data: T) -> bool
    {
        match &node {
            None => false,
            Some(inner_node) => {
                // check if the data is the same
                if inner_node.borrow().data == data {
                    true
                }
                // go toward the correct side of the tree
                else if inner_node.borrow().data > data {
                    Self::search_r(&inner_node.borrow().left, data)
                } else {
                    Self::search_r(&inner_node.borrow().right, data)
                }
            }
        }
    }

    /// Inserts the passed in data into the tree,
    /// basic binary search tree so no rebalancing done
    /// for data inserted
    pub fn insert(&mut self, data: T) -> u32
    {
        // all work done in the recursive function
        Self::insert_r(&mut self.root, data);
        self.size += 1;

        self.size
    }

    /// recursively traverses the tree to add the new
    /// data to the end of the tree
    fn insert_r(node: &mut Option<Link<T>>, data: T) -> ()
    {
        match node {
            None => {// create new node
                *node = Some(Rc::new(RefCell::new(Node::new(data))));
            },
            Some(inner_node) => {
                if inner_node.borrow().data > data {
                    Self::insert_r(&mut inner_node.borrow_mut().left, data);
                } else {
                    Self::insert_r(&mut inner_node.borrow_mut().right, data);
                }
            }
        }
    }

    /// removes a node from the tree that finds data
    /// returns a result a type on whether the data to 
    /// remove is in the tree or not. Ok variant holds
    /// the new size of the tree while the Error variant
    /// infers that the data wasn't in the tree to remove
    pub fn remove(&mut self, data: T) -> Result<u32, Error>
    {
        // all work gets done in recursive function
        // wrapper just propogates the error or 
        // returns the new size if successful
        Self::remove_r(&mut self.root, data)?;
        self.size -= 1;
        Ok(self.size)
    }

    /// recursively traverses the tree to search for the
    /// correct node to remove. Returns a Result<> type with
    /// a unit Ok variant. Meant to just propogate the error
    /// to the calling method so it knows the node wasn't found
    fn remove_r(node: &mut Option<Link<T>>, data: T) -> Result<(), Error>
    {
        match node.take() {
            None => Err(Error),
            Some(inner_node) => {
                let mut node_b = inner_node.borrow_mut();

                // check if the node is correct data
                if node_b.data == data {
                    // check which case is the node
                    
                    // delete the node based on the children
                    match (node_b.left.take(), node_b.right.take()) {
                        // node is a leaf
                        (None, None) => {
                            // node is just set to none
                            *node = None; // unneccessary since we do a take
                        },
                        // one child
                        (Some(left), None) => {
                            *node = Some(left); // take opposing child and consume it
                        },
                        (None, Some(right)) => {
                            *node = Some(right);
                        },
                        // both children exist
                        (Some(left), Some(right)) => {
                            // find in order successor of the right child to consume
                            let mut successor = None;
                            let mut curr = Some(right);

                            while let Some(curr_node) = curr {
                                if curr_node.borrow().left.is_none() {
                                    // curr is the ios
                                    successor = Some(curr_node);
                                    break;
                                } else {
                                    // continue searching
                                    curr = curr_node.borrow_mut().left.clone();
                                }
                            }
                            // make sure the new node keeps the left child
                            successor.as_mut().unwrap().borrow_mut().left = Some(left);
                            *node = successor.take();
                        }
                    }

                    // successfully deleted the node
                    return Ok(());
                }
                
                // return the node's data since this isn't the one we're removing
                *node = Some(inner_node.clone());

                // go the correct child based on relation
                if node_b.data > data {
                    Self::remove_r(&mut node_b.left, data)
                } else {
                    Self::remove_r(&mut node_b.right, data)
                }
            }
        }
    }

}