use std::cell::RefCell;
use std::rc::Rc;

use std::cmp::{Ord, Ordering};

fn main() {
    let mut tree: Tree<i64, i64> = Tree::new();
    assert_eq!(tree.size, 0);
    assert_eq!(tree.insert(0, 0), None);
    assert_eq!(tree.insert(0, 0), Some(0));
    assert_eq!(tree.insert(1, 23), None);
    assert_eq!(tree.insert(1, 20), Some(23));
    assert_eq!(tree.insert(-12, -1), None);

    assert!(tree.find_node(&2).is_none());
    assert_eq!(tree.find_node(&0).unwrap().borrow().value, 0);

    let mut paper_tree: Tree<i64, i64> = Tree::new();
    paper_tree.insert(100, 100);
    paper_tree.insert(50, 50);
    paper_tree.insert(10, 10);
    paper_tree.insert(70, 70);
    paper_tree.insert(60, 60);
    paper_tree.insert(99, 99);

    paper_tree.insert(200, 200);
    paper_tree.insert(115, 115);
    paper_tree.insert(300, 300);

    println!("{}", paper_tree.least_node().unwrap().borrow().key);
    println!("{}", tree.least_node().unwrap().borrow().key);

    for x in paper_tree {
        println!("{} -> {}", x.borrow().key, x.borrow().value);
    }
}

struct Tree<K: Ord, V> {
    size: usize,
    root: Option<Rc<RefCell<TreeNode<K, V>>>>,
}

impl<K: Ord, V> Tree<K, V> {
    fn new() -> Self {
        Tree {
            size: 0,
            root: None,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let new_node = TreeNode::new(key, value);

        match &self.root {
            None => {
                self.root = Some(Rc::new(RefCell::new(new_node)));
                self.size += 1;
                return None;
            }
            Some(link) => self.inner_insert(new_node, Rc::clone(&link)),
        }
    }

    fn inner_insert(
        &mut self,
        mut new_node: TreeNode<K, V>,
        parent_ref: Rc<RefCell<TreeNode<K, V>>>,
    ) -> Option<V> {
        let parent = &mut parent_ref.borrow_mut();
        match parent.key.cmp(&new_node.key) {
            Ordering::Less => match &parent.left {
                None => {
                    new_node.parent = Some(Rc::clone(&parent_ref));
                    let link = Rc::new(RefCell::new(new_node));
                    parent.left = Some(link);
                    self.size += 1;
                    None
                }
                Some(link) => self.inner_insert(new_node, Rc::clone(link)),
            },
            Ordering::Greater => match &parent.right {
                None => {
                    new_node.parent = Some(Rc::clone(&parent_ref));
                    let link = Rc::new(RefCell::new(new_node));
                    parent.right = Some(link);
                    self.size += 1;
                    None
                }
                Some(link) => self.inner_insert(new_node, Rc::clone(link)),
            },
            Ordering::Equal => Some(std::mem::replace(&mut parent.value, new_node.value)),
        }
    }

    fn iter(self) -> TreeIterator<K, V> {
        TreeIterator { tree: self }
    }

    fn find_node(&self, f: &K) -> Option<Rc<RefCell<TreeNode<K, V>>>> {
        if self.root.is_none() {
            return None;
        }

        self.find_node_r(Rc::clone(self.root.as_ref().unwrap()), f)
    }

    fn find_node_r(
        &self,
        node: Rc<RefCell<TreeNode<K, V>>>,
        f: &K,
    ) -> Option<Rc<RefCell<TreeNode<K, V>>>> {
        match &node.borrow().key.cmp(f) {
            Ordering::Greater => match &node.borrow().right {
                Some(lq) => self.find_node_r(Rc::clone(lq), f),
                None => None,
            },
            Ordering::Less => match &node.borrow().left {
                Some(lq) => self.find_node_r(Rc::clone(lq), f),
                None => None,
            },
            Ordering::Equal => Some(Rc::clone(&node)),
        }
    }

    fn least_node(&self) -> Option<Rc<RefCell<TreeNode<K, V>>>> {
        match &self.root {
            None => None,
            Some(root) => {
                Some(self.least_node_r(root))

                // This iterative code does not work :(

                // let mut ref_node: &Rc<RefCell<TreeNode<K, V>>> = &root;
                // while let Some(lq) = ref_node.borrow().left {
                // 	ref_node = lq;
                // }
                // Some(Rc::clone(ref_node))
            }
        }
    }

    fn least_node_r(&self, node: &Rc<RefCell<TreeNode<K, V>>>) -> Rc<RefCell<TreeNode<K, V>>> {
        match &node.borrow().left {
            None => Rc::clone(node),
            Some(lq) => self.least_node_r(lq),
        }
    }
}

// Iteration
struct TreeIterator<K: Ord, V> {
    tree: Tree<K, V>,
}

impl<K: Ord, V> TreeIterator<K, V> {}

impl<K: Ord, V> Iterator for TreeIterator<K, V> {
    type Item = Rc<RefCell<TreeNode<K, V>>>;

    fn next(&mut self) -> Option<Self::Item> {
        // // Tree is empty
        // if self.current_node.is_none() {
        // 	return None;
        // }
        // let current_node = Rc::clone(self.current_node.as_ref().unwrap());

        // // Tree is not empty and we just started
        // if prev_node.is_none() {
        // 	if
        // }
        None
    }
}

impl<K: Ord, V> IntoIterator for Tree<K, V> {
    type Item = Rc<RefCell<TreeNode<K, V>>>;
    type IntoIter = TreeIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
struct TreeNode<K: Ord, V> {
    key: K,
    value: V,
    parent: Option<Rc<RefCell<TreeNode<K, V>>>>,
    right: Option<Rc<RefCell<TreeNode<K, V>>>>,
    left: Option<Rc<RefCell<TreeNode<K, V>>>>,
}

impl<K: Ord, V> TreeNode<K, V> {
    fn new(key: K, value: V) -> Self {
        TreeNode {
            key: key,
            value: value,
            parent: None,
            left: None,
            right: None,
        }
    }
}
