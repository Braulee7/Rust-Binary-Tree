use binary_tree::tree::Tree;

fn main() {
    // TESTING
    let mut tree = Tree::new();

    // ACT
    tree.insert(3);
    tree.insert(1);
    tree.insert(2);
    tree.insert(4);
    tree.insert(5);

    println!("{:?}", tree);
    
    // remove a leaf
    println!("{:?}", tree.remove(5).unwrap());
    // remove an inner node
    println!("{:?}", tree.remove(1).unwrap());
    println!("After removing One: {:?}", tree);
    // remove the root
    println!("{:?}", tree.remove(3).unwrap());
    println!("After removing Three: {:?}", tree);

     println!("{:?}", tree);
}
