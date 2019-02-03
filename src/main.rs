use std::rc::Rc;
use std::cell::RefCell;

use std::cmp::{Ord, Ordering};

fn main() {
    let mut tree: Tree<i64, i64> = Tree::new();
    assert_eq!(tree.size, 0);
    assert_eq!(tree.insert(0, 0), None);
    assert_eq!(tree.insert(0, 0), Some(0));
    assert_eq!(tree.insert(1, 23), None);
    assert_eq!(tree.insert(1, 20), Some(23));
}

struct Tree<T: Eq + Ord, V> {
  size: usize,
  root: Option<Rc<RefCell<TreeNode<T, V>>>>
}

impl<T: Ord, V> Tree<T, V> {
  fn new() -> Self {
    Tree{size: 0, root: None}
  }

  fn insert(&mut self, key: T, value: V) -> Option<V> {
    let new_node = TreeNode::new(key, value);

    match &self.root {
      None => {
        self.root = Some(Rc::new(RefCell::new(new_node)));
        self.size += 1;
        return None;
      },
      Some(link) => {
        self.inner_insert(new_node, Rc::clone(&link))
      }
    }
  }

  fn inner_insert(&mut self, mut new_node: TreeNode<T, V>, parent_ref: Rc<RefCell<TreeNode<T, V>>>) -> Option<V> {
    let mut parent = &mut parent_ref.borrow_mut();
    match parent.key.cmp(&new_node.key) {
      Ordering::Less => {

        match &parent.left {
          None => {
            new_node.parent = Some(Rc::clone(&parent_ref));
            let link = Rc::new(RefCell::new(new_node));
            parent.left = Some(link);
            self.size += 1;
            None
          },
          Some(link) => {
            self.inner_insert(new_node, Rc::clone(link))
          }
        }
      },
      Ordering::Greater => {

        match &parent.right {
          None => {
            new_node.parent = Some(Rc::clone(&parent_ref));
            let link = Rc::new(RefCell::new(new_node));
            parent.left = Some(link);
            self.size += 1;
            None
          },
          Some(link) => {
            self.inner_insert(new_node, Rc::clone(link))
          }
        }
      },
      Ordering::Equal => {
        Some(std::mem::replace( parent.value, new_node.value))
      }
    }
  }
}

#[derive(Debug)]
struct TreeNode<T: Eq + Ord, V> {
  key: T,
  value: V,
  parent: Option<Rc<RefCell<TreeNode<T, V>>>>,
  right: Option<Rc<RefCell<TreeNode<T, V>>>>,
  left: Option<Rc<RefCell<TreeNode<T, V>>>>
}


impl<T: Ord, V> TreeNode<T, V> {
  fn new(key: T, value: V) -> Self {
    TreeNode{key: key, value: value, parent: None, left: None, right: None}
  }
}