use std::cell::RefCell;
use std::rc::Rc;

use std::cmp::{Ord, Ordering};

pub struct Tree<K: Ord, V> {
    size: usize,
    root: Option<Rc<RefCell<TreeNode<K, V>>>>,
}

impl<K: Ord, V> Tree<K, V> {
    pub fn new() -> Self {
        Tree {
            size: 0,
            root: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
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

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter_node(self) -> TreeNodeIterator<K, V> {
        TreeNodeIterator {
            current_node: self.least_node(),
        }
    }

    pub fn find_node(&self, f: &K) -> Option<Rc<RefCell<TreeNode<K, V>>>> {
        if self.root.is_none() {
            return None;
        }

        let root = self.root.as_ref().unwrap();
        root.borrow().find_node_r(Rc::clone(root), f)
    }

    pub fn least_node(&self) -> Option<Rc<RefCell<TreeNode<K, V>>>> {
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

    pub fn delete(&mut self, key: &K) -> Option<Rc<RefCell<TreeNode<K, V>>>> {

    	let node = self.find_node(key);
    	match node {
            None => None,
            Some(ref node) => {
                let tree_node = node.borrow();

                let parent = &tree_node.parent;
                if parent.is_none() {
                    // deleting root
                    match (tree_node.left.is_some(), tree_node.right.is_some()) {
                        (false, false) => {
                            self.root = None;
                        },
                        (true, false) => {
                            let new_root = tree_node.left_sure();
                            new_root.borrow_mut().parent = None;
                            self.root = Some(Rc::clone(&new_root));
                        },
                        (false, true) => {
                            let new_root = tree_node.right_sure();
                            new_root.borrow_mut().parent = None;
                            self.root = Some(Rc::clone(&new_root));
                        },
                        // Both exists, lets be lazy and go once right then full left 
                        // (pretty sure this will disbalance tree even more, but no one cares)
                        (true, true) => {
                            let shift_ref = tree_node.left.as_ref().unwrap();
                            let mut shift = shift_ref.borrow_mut();

                            let new_root_ref = tree_node.right_sure();
                            let mut new_root = new_root_ref.borrow_mut();
                            
                            let new_parent_ref = new_root.least_node_r(&new_root_ref);

                            // Now things are getting complicated
                            // It is possible, that new_parent is the same as new_root
                            // and we can not borrow it twice
                            if Rc::ptr_eq(&new_root_ref, &new_parent_ref) {
                                self.root = Some(Rc::clone(&new_root_ref));
                                new_root.parent = None;

                                shift.parent = Some(Rc::clone(&new_parent_ref));
                                new_root.left = Some(Rc::clone(shift_ref));
                            } else {
                                let mut new_parent = new_parent_ref.borrow_mut(); 

                                self.root = Some(Rc::clone(&new_root_ref));
                                new_root.parent = None;

                                shift.parent = Some(Rc::clone(&new_parent_ref));
                                new_parent.left = Some(Rc::clone(shift_ref));
                            }
                        }
                    }
                } else {
                    let parent_ref = tree_node.parent_sure();
                    let mut parent = parent_ref.borrow_mut();

                    // messy but i still like it
                    match (tree_node.left.is_some(), tree_node.right.is_some(), tree_node.key.cmp(&parent.key)) {
                        (_, _, Ordering::Equal) => {
                            unreachable!("Parent has same key with its child. Shount not have happened");
                        },
                        (false, false, Ordering::Less) => {
                            parent.left = None;
                        },
                        (false, false, Ordering::Greater) => {
                            parent.right = None;
                        },
                        (true, false, Ordering::Less) => {
                            let shift_ref = tree_node.left.as_ref().unwrap();
                            let mut shift = shift_ref.borrow_mut();

                            shift.parent = Some(Rc::clone(&parent_ref));
                            parent.left = Some(Rc::clone(shift_ref));
                        },
                        (true, false, Ordering::Greater) => { 
                            let shift_ref = tree_node.left.as_ref().unwrap();
                            let mut shift = shift_ref.borrow_mut();

                            shift.parent = Some(Rc::clone(&parent_ref));
                            parent.right = Some(Rc::clone(shift_ref));
                        },
                        (false, true, Ordering::Less) => { 
                            let shift_ref = tree_node.right.as_ref().unwrap();
                            let mut shift = shift_ref.borrow_mut();

                            shift.parent = Some(Rc::clone(&parent_ref));
                            parent.left = Some(Rc::clone(shift_ref));
                        },
                        (false, true, Ordering::Greater) => { 
                            let shift_ref = tree_node.right.as_ref().unwrap();
                            let mut shift = shift_ref.borrow_mut();

                            shift.parent = Some(Rc::clone(&parent_ref));
                            parent.right = Some(Rc::clone(shift_ref));
                        },
                        // Both exists, lets be lazy and go once right then full left 
                        // (pretty sure this will disbalance tree even more, but no one cares)
                        (true, true, Ordering::Less) => { 
                            let shift_ref = &tree_node.left_sure();
                            let mut shift = shift_ref.borrow_mut();
                            
                            let shifter_ref = tree_node.right_sure();
                            let mut shifter = shifter_ref.borrow_mut();

                            let new_parent_ref = shifter.least_node_r(&shifter_ref);

                            // Now things are getting complicated
                            // It is possible, that new_parent is the same as new_root
                            // and we can not borrow it twice
                            if Rc::ptr_eq(&shifter_ref, &new_parent_ref) {
                                
                                shifter.left = Some(Rc::clone(shift_ref));
                                shift.parent = Some(Rc::clone(&new_parent_ref));

                                parent.left = Some(Rc::clone(&shifter_ref));
                                shifter.parent = Some(Rc::clone(&parent_ref));
                            } else {
                                let mut new_parent = new_parent_ref.borrow_mut(); 

                                new_parent.left = Some(Rc::clone(shift_ref));
                                shift.parent = Some(Rc::clone(&new_parent_ref));

                                parent.left = Some(Rc::clone(&shifter_ref));
                                shifter.parent = Some(Rc::clone(&parent_ref));
                            }
                        },
                        (true, true, Ordering::Greater) => { 
                            let shift_ref = &tree_node.left_sure();
                            let mut shift = shift_ref.borrow_mut();

                            let shifter_ref = tree_node.right_sure();
                            let mut shifter = shifter_ref.borrow_mut();

                            let new_parent_ref = shifter.least_node_r(&shifter_ref);

                            // Now things are getting complicated
                            // It is possible, that new_parent is the same as new_root
                            // and we can not borrow it twice
                            if Rc::ptr_eq(&shifter_ref, &new_parent_ref) {
                                
                                shifter.left = Some(Rc::clone(shift_ref));
                                shift.parent = Some(Rc::clone(&new_parent_ref));

                                parent.right = Some(Rc::clone(&shifter_ref));
                                shifter.parent = Some(Rc::clone(&parent_ref));
                            } else {
                                let mut new_parent = new_parent_ref.borrow_mut(); 

                                new_parent.left = Some(Rc::clone(shift_ref));
                                shift.parent = Some(Rc::clone(&new_parent_ref));

                                parent.right = Some(Rc::clone(&shifter_ref));
                                shifter.parent = Some(Rc::clone(&parent_ref));
                            }
                        },
                    }
                }

                self.size -= 1;
                Some(Rc::clone(node))
            }
        }
    }
}

// Iteration
pub struct TreeNodeIterator<K: Ord, V> {
    current_node: Option<Rc<RefCell<TreeNode<K, V>>>>,
}

impl<K: Ord, V> TreeNodeIterator<K, V> {
    fn find_next(&mut self) {
        // this was called so current_node is not None

        // Copy of current node
        let ref_curr = Rc::clone(self.current_node.as_ref().unwrap());
        let curr = ref_curr.borrow();

        // Check if we have right child
        match &curr.right {
            Some(lq) => {
                // If so, get lowest from right child
                self.current_node = Some(lq.borrow().least_node_r(lq));
            }
            None => {
                // Otherwise we are going up
                let mut this = Rc::clone(&ref_curr);

                loop {
                    // borrow checker, please
                    let cp = Rc::clone(&this);
                    let parent = &cp.borrow().parent;

                    match &parent {
                        None => {
                            // If there was no parent then there are no other nodes in tree
                            std::mem::swap(&mut self.current_node, &mut None);
                            break;
                        }
                        Some(ref p) => {
                            // If there was a parent node then we have to check if this relation is left or right
                            if this.borrow().key.cmp(&p.borrow().key) == Ordering::Greater {
                                // If this is right child relation then we are looking higher
                                this = Rc::clone(p);
                                continue;
                            } else {
                                // If this is left child relation then parent is next node
                                std::mem::swap(&mut self.current_node, &mut Some(Rc::clone(p)));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<K: Ord, V> Iterator for TreeNodeIterator<K, V> {
    type Item = Rc<RefCell<TreeNode<K, V>>>;

    // This iterator is a little bit odd
    // We compute next value on the same iteration with "this" value
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
    type IntoIter = TreeNodeIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_node()
    }
}

pub struct TreeNode<K: Ord, V> {
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
        match f.cmp(&self.key) {
            Ordering::Greater => {
                match &self.right {
                    Some(lq) => lq.borrow().find_node_r(Rc::clone(lq), f),
                    None => None,
                }
            },
            Ordering::Less => {
                match &self.left {
                    Some(lq) => lq.borrow().find_node_r(Rc::clone(lq), f),
                    None => None,
                }
            },
            Ordering::Equal => {
                Some(myself)
            },
        }
    }

    fn least_node_r(&self, myself: &Rc<RefCell<TreeNode<K, V>>>) -> Rc<RefCell<TreeNode<K, V>>> {
        match &self.left {
            None => Rc::clone(myself),
            Some(lq) => lq.borrow().least_node_r(lq),
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty_tree() {
	    let tree: Tree<i64, i64> = Tree::new();
	    assert_eq!(tree.size, 0);
	}

	#[test]
	fn insert_find() {
	    let mut tree: Tree<i64, i64> = Tree::new();
		assert_eq!(tree.insert(0, 0), None);
	    assert_eq!(tree.insert(0, 0), Some(0));
	    
	    assert_eq!(tree.find_node(&0).unwrap().borrow().value, 0);
	}

	#[test]
	fn iterate() {
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

	    let ppr_tree_size = paper_tree.size;
	    let i = paper_tree.into_iter().count();

	    assert_eq!(ppr_tree_size, i);
	    assert_eq!(ppr_tree_size, 9);
	}

	#[test]
	fn delete_find() {
	    let mut tree: Tree<i64, i64> = Tree::new();
		assert_eq!(tree.insert(0, 0), None);
	    assert!(tree.delete(&0).is_some());
		assert!(tree.delete(&0).is_none());
	}

    #[test]
    fn delete_root_both_same() {
		let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(50, 50);
	
	    paper_tree.insert(200, 200);
	
        let deleted = paper_tree.delete(&100);
        let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();

        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
    }

    #[test]
    fn delete_root_both_diff() {
		let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(50, 50);
	
	    paper_tree.insert(200, 200);
        paper_tree.insert(150, 150);
	
        let deleted = paper_tree.delete(&100);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
    }

    #[test]
    fn delete_root_l() {
		let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(50, 50);
	
        let deleted = paper_tree.delete(&100);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
    }

    #[test]
    fn delete_root_r() {
		let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(150, 150);
	
        let deleted = paper_tree.delete(&100);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
    }
    
    #[test]
    fn delete_root_none() {
		let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	
        let deleted = paper_tree.delete(&100);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
    }

    
    #[test]
    fn delete_miss() {
		let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	
        let deleted = paper_tree.delete(&1000);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        assert_eq!(true, deleted.is_none());
        assert_eq!(cnt, tree_cnt);
    }

    #[test]
	fn delete_node_none() {
	    let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(50, 50);
	    
        let deleted = paper_tree.delete(&50);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
	}

    #[test]
	fn delete_node_l() {
	    let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(50, 50);
        paper_tree.insert(10, 10);
	    
        let deleted = paper_tree.delete(&50);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
	}

    #[test]
	fn delete_node_r() {
	    let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(50, 50);
        paper_tree.insert(70, 70);
	    
        let deleted = paper_tree.delete(&50);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
	}

    #[test]
	fn delete_node_both_l_same() {
	    let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(50, 50);
        paper_tree.insert(10, 10);
        paper_tree.insert(70, 70);
	    
        let deleted = paper_tree.delete(&50);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
	}

    #[test]
	fn delete_node_both_r_same() {
	    let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(150, 150);
        paper_tree.insert(110, 110);
        paper_tree.insert(170, 170);
	    
        let deleted = paper_tree.delete(&150);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
	}

    #[test]
	fn delete_node_both_l_diff() {
	    let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(50, 50);
        paper_tree.insert(10, 10);
        paper_tree.insert(70, 70);
        paper_tree.insert(60, 60);
	    
        let deleted = paper_tree.delete(&50);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
	}

    #[test]
	fn delete_node_both_r_diff() {
	    let mut paper_tree: Tree<i64, i64> = Tree::new();
	    paper_tree.insert(100, 100);
	    paper_tree.insert(150, 150);
        paper_tree.insert(110, 110);
        paper_tree.insert(170, 170);
        paper_tree.insert(160, 160);
	    
        let deleted = paper_tree.delete(&150);
	    let tree_cnt = paper_tree.size;
        let cnt = paper_tree.into_iter().count();
        
        assert_eq!(true, deleted.is_some());
        assert_eq!(cnt, tree_cnt);
	}
}