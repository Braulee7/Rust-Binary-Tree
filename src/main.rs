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

    println!("Data in Tree <Total of {:?} nodes in tree>:", tree.display());

    let mut new_tree = tree.clone();


    for item in new_tree {
        println!("The current item is : {:?}", item);
    }
     println!("{:?}", tree);
}
