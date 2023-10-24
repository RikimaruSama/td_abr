#[derive(Debug)]
pub struct Tree(Option<Box<Node>>);

/// Internal Node representation with a `value` and the left and right sub-trees.
#[derive(Debug)]
struct Node {
    value: i32,
    left: Tree,
    right: Tree,
}

impl Tree {
    /// Returns an empty tree
    pub fn new() -> Self {
        Tree(None)
    }

    fn leaf(value: i32) -> Self {
        Tree(Some(Box::new(Node { value: value, left: Tree(None), right: Tree(None) })))
    }

    pub fn contains(&self, value: i32) -> bool {
        println!("{:?}", self);
        false
    }
}


fn main(){
    let mut t = Tree::new();
    t = Tree::leaf(2);
    t.contains(0);
    println!("{:?}", t);
}