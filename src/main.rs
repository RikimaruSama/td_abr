use std::cmp::{Ordering, self};

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
        if self.0.as_ref().is_none(){
            return false;
        }
        let order = &self.0.as_ref().unwrap().value.cmp(&value);
        match order {
            Ordering::Equal => return true,
            Ordering::Greater => return self.0.as_ref().unwrap().left.contains(value),
            Ordering::Less => return self.0.as_ref().unwrap().right.contains(value),
            _ => panic!("[Erreur]: Difference entre deux choses inconnus"),
        }
    }

    pub fn insert(&mut self, value: i32) -> bool {
        match &mut self.0 {
            Some(ref mut n) => match value.cmp(&n.value){
                Ordering::Less => return n.left.insert(value),
                Ordering::Equal => return false,
                Ordering::Greater => return n.right.insert(value),
            }
            None => {
                *self = Tree::leaf(value);
                return true;
            }
        }
    }

    pub fn delete(&mut self, value: i32) {
        match &mut self.0 {
            Some(ref mut n) => match value.cmp(&n.value){
                Ordering::Less => return n.left.insert(value),
                Ordering::Equal => {

                },
                Ordering::Greater => return n.right.insert(value),
            }
            None => ,
        }
    }
    
}


fn main(){
    let mut t = Tree::new();
    t = Tree::leaf(4);
    t = Tree(Some(Box::new(Node {
        value: 12,
        left: Tree(Some(Box::new(Node{value: 8, left: Tree(None), right: Tree(None) }))),
        right: Tree(Some(Box::new(Node{value: 27, left: Tree(None), right: Tree(None) }))),
        })));
    t.insert(7);
    println!("{:?}", t);
}