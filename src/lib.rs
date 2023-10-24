use std::cmp::Ordering;

/// Simple binary search tree
///
/// For every node of value `v`, all elements in the left sub-tree are smaller
/// than `v` and all elements in the right sub-tree are larger than `v`.
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

    /// Returns a tree containing a single value
    fn leaf(value: i32) -> Self {
        Tree(Some(Box::new(Node { value: value, left: Tree(None), right: Tree(None) })))
    }

    /// Inserts `value` into the tree.
    /// Returns `false` iff the `value` was already contained in the tree.
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

    /// Returns true if and only if `value` belongs to the tree.
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

    /// Deletes `value` from the tree.
    /// When the value is not found the tree, `false` is returned.
    pub fn delete(&mut self, value: i32) {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_be_empty() {
        assert!(true)
    }

    #[test]
    fn should_be_one_tree() {
        assert!(true)
    }
}