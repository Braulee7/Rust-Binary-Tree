use std::{
    cell::RefCell, cmp::{PartialEq, PartialOrd}, fmt::{Debug, Error}, rc::Rc
        };
use dll::dll::List;

// types used for the link
type Link<T> = Rc<RefCell<Node<T>>>;

// private basic struct of a node
#[derive(Debug)]
struct Node<T: Clone + PartialOrd + PartialEq + Debug>
{
    data: T,
    left: Option<Link<T>>,
    right: Option<Link<T>>
}

// implementation of Node to create a new one
impl<T: Clone + PartialOrd + PartialEq + Debug> Node<T>
{
    pub fn new(data: T) -> Self
    {
        Node { data, left: None, right: None}
    }
}

impl<T: Clone + PartialOrd + PartialEq + Debug>  PartialEq for Node<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }

    fn ne(&self, other: &Self) -> bool {
        self.data != other.data
    }
}

/// allows us to compare nodes much easier by using
/// the relational operators
impl<T: Clone + PartialOrd + Debug> PartialOrd for Node<T>
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
pub struct Tree<T: Clone + PartialOrd + PartialEq + Debug>
{
    root: Option<Link<T>>,
    size: u32
}

pub struct TreeIterator<T: Clone + PartialOrd + PartialEq + Debug>
{
    // container for a doubly linked list
    // opted to use this over a vec since it has
    // push front and back methods
    nodes: List<Link<T>>,
}

// api of the tree
impl<T: Clone + PartialOrd + PartialEq + Debug> Tree<T>
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

    ///  recursively copies the data from src to the destination node
    /// which in turn copies the entire tree. Meant to be used with the 
    /// Clone trait
    fn copy_r(src: &Option<Link<T>>, dst: &mut Option<Link<T>>) -> u32
    {
        match src {
            None => 0,
            Some(src_node) => {
                // copy src node's data to a new node
                let new_node = Rc::new(RefCell::new(Node::new(src_node.borrow().data.clone())));
                let mut number_of_nodes = 0;

                { // new scope to abide by borrow checker with the new node
                    let mut mut_borrow_node = new_node.borrow_mut();
                    // copy the left and right children
                    number_of_nodes += Self::copy_r(&src_node.borrow().left, &mut mut_borrow_node.left) +
                                Self::copy_r(&src_node.borrow().right, &mut mut_borrow_node.right) +
                                1;
                }

                *dst = Some(new_node);
                
                number_of_nodes
            }
        }
    }


    /// traverses the tree in order, copying over all the data into the 
    /// itterator so it can be used as an itterator
    fn get_nodes_in_order(src: &Option<Link<T>>, dst: &mut TreeIterator<T>) -> u32
    {
        match src {
            None => 0,
            Some(src_node) => {
                let mut number_of_nodes = 0;
                // go left
                number_of_nodes += Self::get_nodes_in_order(&src_node.borrow().left, dst);
                // copy current node
                dst.nodes.push_back(src_node.clone());
                // go right
                number_of_nodes += Self::get_nodes_in_order(&src_node.borrow().right, dst);

                number_of_nodes + 1
            }
        }
    }

    /// displays the data in the tree in order
    pub fn display(&self) -> u32
    {
        Self::display_r(&self.root)
    }

    /// recursive function ot traverse the tree in order
    /// and display all the data
    fn display_r(node: &Option<Link<T>>) -> u32
    {
        match node {
            None => 0,
            Some(node) => {
                let mut total: u32 = 0;
                // go left, then node, then right
                total += Self::display_r(&node.borrow().left);

                println!("{:?}", node.borrow().data);
                total += 1;

                total += Self::display_r(&node.borrow().right);

                total
            }
        }
    }

}

/// implements all the necessary methods for the tree to be
/// cloned properly. Performs a deep copy of the tree allocating
/// new memory for the new tree.
impl<T: Clone + PartialOrd + PartialEq + Debug> Clone for Tree<T>
{
    fn clone(&self) -> Self {
        let mut new = Self::new();
        assert_eq!(self.size, Self::copy_r(&self.root, &mut new.root));
        // make sure the new tree's size is also kept correctly
        new.size = self.size;
        new
    }
}

/// implementing the into iter method so that the tree can be itterated over
/// in a for loop
impl<T: Clone + PartialOrd + PartialEq + Debug> IntoIterator for Tree<T>
{
    type IntoIter = TreeIterator<T>;
    type Item = T;

    fn into_iter(self) -> TreeIterator<T>
    {
        let mut iter = TreeIterator::new();
        assert_eq!(self.size, Self::get_nodes_in_order(&self.root, &mut iter));
        iter
    }
}

/// creates a new tree iterator object which is just a container
/// for a doubly linked list
impl<T: Clone + PartialOrd + PartialEq + Debug> TreeIterator<T>
{
    pub fn new() -> Self
    {
        Self{nodes: List::new()}
    }
}

/// properly implements the iterator trait so that the iterator
/// can be used properly
impl<T: Clone + PartialOrd + PartialEq + Debug> Iterator for TreeIterator<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_node = self.nodes.pop_front();

        match next_node {
            None => None,
            Some(node) => Some(node.borrow().data.clone())
        }
    }
}