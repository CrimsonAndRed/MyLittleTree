use std::rc::Rc;
use std::cell::RefCell;

use std::cmp::{Ord, Eq, PartialEq, Ordering};

fn main() {
    let mut tree = Tree::new();
    assert_eq!(tree.size, 0);
    assert_eq!(tree.insert(0, 0), None);
    assert_eq!(tree.insert(0, 0), Some(0));
    assert_eq!(tree.insert(1, 23), None);
    assert_eq!(tree.insert(1, 20), Some(23));
}

struct Tree {
  size: usize,
  root: Option<Rc<RefCell<TreeNode>>>
}

impl Tree {
  fn new() -> Self {
    Tree{size: 0, root: None}
  }

  fn insert(&mut self, key: i64, value: i64) -> Option<i64> {
    let mut new_node = TreeNode::new(key, value);

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

  fn inner_insert(&mut self, mut new_node: TreeNode, parent_ref: Rc<RefCell<TreeNode>>) -> Option<i64> {
    let mut parent = parent_ref.borrow_mut();
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

        let tmp = parent.value;
        parent.value = new_node.value;
        Some(tmp)
      }
    }
  }
}

#[derive(Debug)]
struct TreeNode {
  key: i64,
  value: i64,
  parent: Option<Rc<RefCell<TreeNode>>>,
  right: Option<Rc<RefCell<TreeNode>>>,
  left: Option<Rc<RefCell<TreeNode>>>
}


impl TreeNode {
  fn new(key: i64, value: i64) -> Self {
    TreeNode{key: key, value: value, parent: None, left: None, right: None}
  }
}