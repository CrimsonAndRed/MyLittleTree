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

    let x1 = paper_tree.find_node(&101);
    assert!(x1.is_none());
    paper_tree.insert(10, 10);
    paper_tree.insert(70, 70);
    paper_tree.insert(60, 60);
    paper_tree.insert(99, 99);

    paper_tree.insert(200, 200);
    paper_tree.insert(115, 115);
    paper_tree.insert(300, 300);

    println!("{}", paper_tree.least_node().unwrap().borrow().key);
//    println!("{}", tree.least_node().unwrap().borrow().key);

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
        match new_node.key.cmp(&parent.key) {
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
        TreeIterator {current_node: self.least_node()}
    }

    fn find_node(&self, f: &K) -> Option<Rc<RefCell<TreeNode<K, V>>>> {
        if self.root.is_none() {
            return None;
        }

        let root = self.root.as_ref().unwrap();
        root.borrow().find_node_r(Rc::clone(root), f)
    }

    fn least_node(&self) -> Option<Rc<RefCell<TreeNode<K, V>>>> {
        match &self.root {
            None => None,
            Some(root) => {
                Some(root.borrow().least_node_r(root))

                // This iterative code does not work :(

                // let mut ref_node: &Rc<RefCell<TreeNode<K, V>>> = &root;
                // while let Some(lq) = ref_node.borrow().left {
                // 	ref_node = lq;
                // }
                // Some(Rc::clone(ref_node))
            }
        }
    }
}

// Iteration
struct TreeIterator<K: Ord, V> {
    current_node: Option<Rc<RefCell<TreeNode<K, V>>>>,
}

impl<K: Ord, V> TreeIterator<K, V> {

    fn find_next(&mut self) {
        // this was called so current_node is not None
        let ref_curr = Rc::clone(self.current_node.as_ref().unwrap());
        let curr = ref_curr.borrow();

        // Check if we have right child
        match &curr.right {
            Some(lq) => {
                // If so, get lowest from right child
                self.current_node =  Some(lq.borrow().least_node_r(lq));
            }
            None => {
//                let parent = &curr.parent;

                let mut this = Rc::clone(&ref_curr);

                loop {
                    let cp = Rc::clone(&this);
                    let parent = &cp.borrow().parent;

                    if parent.is_none() {
                        std::mem::swap(&mut self.current_node, &mut None);
                        break;
                    } else {
                        let b = Rc::clone(&this);
                        let x = b.borrow().parent_sure();
                        let a = Rc::clone(&this);

                        if a.borrow().key.cmp(&x.borrow().key) == Ordering::Greater {
                            this = Rc::clone(&x);
                            continue;
                        } else {
                            let _ = std::mem::replace(&mut self.current_node, Some(Rc::clone(&x)));
                            break;
                        }
                    }
                }

                // else check if we have parent
//                match parent {
//                    // If so - go up while we are in right-childs branch -> first parent of left-child relation is next node
//                    Some(p) => {
//
//                        let cpble = &curr.key;
//                        let mut cmp_ref = Rc::clone(p);
//
//                        while cmp_ref.borrow().parent.is_some()  {
//                            // If childs key > parent key - it was right child -> go up
//                            if cpble.cmp(&cmp_ref.borrow().key) == Ordering::Greater {
//                                let x = &cmp_ref.borrow().parent_sure();
//                                cmp_ref = Rc::clone(x);
//                                continue;
//                            } else {
//
//                                // Else childs key < parent key - next node is parent
//                                let _ = std::mem::replace(&mut self.current_node, Some(Rc::clone(&cmp_ref)));
//                                return;
//                            }
//                        }
//
//
//                        // Here we come in case we were in root from its right branch
//                        std::mem::swap(&mut self.current_node, &mut None);
//                    }
//                    // otherwise we dont have right child and parent -> we are root
//                    None => {
//                        // We are in root
//                        std::mem::swap(&mut self.current_node, &mut None);
//                    },
//                }
            }
        }
    }
}

impl<K: Ord, V> Iterator for TreeIterator<K, V> {
    type Item = Rc<RefCell<TreeNode<K, V>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current_node {
            None => None,
            Some(ref lq) => {
                let result = Rc::clone(lq);

                self.find_next();

                Some(result)
            }
        }
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

    fn left_sure(&self) -> Rc<RefCell<TreeNode<K, V>>> {
        Rc::clone(self.left.as_ref().unwrap())
    }

    fn right_sure(&self) -> Rc<RefCell<TreeNode<K, V>>> {
        Rc::clone(self.right.as_ref().unwrap())
    }

    fn parent_sure(&self) -> Rc<RefCell<TreeNode<K, V>>> {
        Rc::clone(self.parent.as_ref().unwrap())
    }

    fn find_node_r(
        &self,
        myself: Rc<RefCell<TreeNode<K, V>>>,
        f: &K,
    ) -> Option<Rc<RefCell<TreeNode<K, V>>>> {
        match &self.key.cmp(f) {
            Ordering::Greater => match &self.right {
                Some(lq) => lq.borrow().find_node_r(Rc::clone(lq), f),
                None => None,
            },
            Ordering::Less => match &self.left {
                Some(lq) => lq.borrow().find_node_r(Rc::clone(lq), f),
                None => None,
            },
            Ordering::Equal => Some(myself),
        }
    }


    fn least_node_r(&self, myself: &Rc<RefCell<TreeNode<K, V>>>) -> Rc<RefCell<TreeNode<K, V>>> {
        match &self.left {
            None => Rc::clone(myself),
            Some(lq) => lq.borrow().least_node_r(lq),
        }
    }
}