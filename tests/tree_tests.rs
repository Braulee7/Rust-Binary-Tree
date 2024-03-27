
#[cfg(test)]
pub mod binary_tree_tests 
{
    use binary_tree::tree::Tree;
    
    // tests if an element can be added to the tree properly
    #[test]
    fn test_insert()
    {
        // ARRANGE
        let mut tree: Tree<String> = Tree::new();

        // ACT
        tree.insert("first".to_owned());
        tree.insert("second".to_owned());
        tree.insert("third".to_owned());

        // ASSERT
        assert_eq!(tree.size(), 3);
        assert_eq!(tree.search("first".to_string()), true);
        assert_eq!(tree.search("second".to_string()), true);
        assert_eq!(tree.search("third".to_string()), true);
    }

    #[test]
    fn test_remove()
    {
        // ARRANGE
        let mut tree = Tree::new();

        // ACT
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        tree.insert(4);
        tree.insert(5);
        tree.insert(-5);
        tree.insert(80);

        // remove a leaf
        assert_eq!(tree.remove(5), Ok(6));
        // remove an inner node
        assert_eq!(tree.remove(1), Ok(5));
        // remove the root
        assert_eq!(tree.remove(3), Ok(4));

        // AsSERT
        assert_eq!(tree.search(1), false);
        assert_eq!(tree.search(2), true);
        assert_eq!(tree.search(3), false);
        assert_eq!(tree.search(4), true);
        assert_eq!(tree.search(5), false);
        assert_eq!(tree.search(-5), true);
        assert_eq!(tree.search(80), true);
        // make sure it fails when removing a non-existant node
        assert!(tree.remove(999).is_err());
    }
}