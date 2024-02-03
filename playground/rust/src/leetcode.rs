use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn options_from_str(s: &str) -> Vec<Option<i32>> {
    serde_json::from_str(s).unwrap()
}

pub fn i32_from_str(s: &str) -> Vec<i32> {
    serde_json::from_str(s).unwrap()
}

pub fn matrix_i32_from_str(s: &str) -> Vec<Vec<i32>> {
    serde_json::from_str(s).unwrap()
}

fn tree_from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if v.is_empty() {
        return None;
    }
    let mut iter = v.into_iter();
    let mut q = VecDeque::new();
    let mut root = None;
    if let Some(first) = iter.next().unwrap() {
        root = Some(Rc::new(RefCell::new(TreeNode::new(first))));
        q.push_back(root.clone());
    } else {
        return None;
    }

    while let Some(node) = q.pop_front() {
        if iter.is_empty() {
            break;
        }
        if node.is_none() {
            iter.nth(2);
            q.push_back(None);
            q.push_back(None);
            continue;
        }

        let node = node.unwrap();

        let mut left = None;
        if let Some(val) = iter.next().unwrap() {
            left.replace(Rc::new(RefCell::new(TreeNode::new(val))));
            node.borrow_mut().left = left.clone();
        }
        q.push_back(left);

        if iter.is_empty() {
            break;
        }
        let mut right = None;
        if let Some(val) = iter.next().unwrap() {
            right.replace(Rc::new(RefCell::new(TreeNode::new(val))));
            node.borrow_mut().right = right.clone();
        }
        q.push_back(right);
    }

    root
}

pub fn tree_from_str(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
    tree_from_vec(options_from_str(s))
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
