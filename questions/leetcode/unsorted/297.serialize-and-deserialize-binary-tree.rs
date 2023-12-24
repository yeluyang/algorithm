/*
 * @lc app=leetcode.cn id=297 lang=rust
 *
 * [297] 二叉树的序列化与反序列化
 *
 * https://leetcode.cn/problems/serialize-and-deserialize-binary-tree/description/
 *
 * algorithms
 * Hard (59.08%)
 * Likes:    1189
 * Dislikes: 0
 * Total Accepted:    229.9K
 * Total Submissions: 389.2K
 * Testcase Example:  '[1,2,3,null,null,4,5]'
 *
 *
 * 序列化是将一个数据结构或者对象转换为连续的比特位的操作，进而可以将转换后的数据存储在一个文件或者内存中，同时也可以通过网络传输到另一个计算机环境，采取相反方式重构得到原数据。
 *
 * 请设计一个算法来实现二叉树的序列化与反序列化。这里不限定你的序列 /
 * 反序列化算法执行逻辑，你只需要保证一个二叉树可以被序列化为一个字符串并且将这个字符串反序列化为原始的树结构。
 *
 * 提示: 输入输出格式与 LeetCode 目前使用的方式一致，详情请参阅 LeetCode
 * 序列化二叉树的格式。你并非必须采取这种方式，你也可以采用其他的方法解决这个问题。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：root = [1,2,3,null,null,4,5]
 * 输出：[1,2,3,null,null,4,5]
 *
 *
 * 示例 2：
 *
 *
 * 输入：root = []
 * 输出：[]
 *
 *
 * 示例 3：
 *
 *
 * 输入：root = [1]
 * 输出：[1]
 *
 *
 * 示例 4：
 *
 *
 * 输入：root = [1,2]
 * 输出：[1,2]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 树中结点数在范围 [0, 10^4] 内
 * -1000
 *
 *
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut buff = vec![];
        if let Some(root) = root {
            buff.push(root.borrow().val.to_string());
            Self::_serialize(&root, &mut buff);
        };
        "[".to_owned() + buff.join(",").trim_end_matches(",null") + "]"
    }
    fn _serialize(root: &Rc<RefCell<TreeNode>>, buff: &mut Vec<String>) {
        buff.push(if let Some(n) = &root.borrow().left {
            n.borrow().val.to_string()
        } else {
            "null".to_owned()
        });
        buff.push(if let Some(n) = &root.borrow().right {
            n.borrow().val.to_string()
        } else {
            "null".to_owned()
        });

        if let Some(left) = &root.borrow().left {
            Self::_serialize(&left, buff);
        };
        if let Some(right) = &root.borrow().right {
            Self::_serialize(&right, buff);
        };
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() || &data == "[]" {
            return None;
        }

        let mut data = data
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(",")
            .map(|v| match v {
                "null" => None,
                _ => Some(v.parse::<i32>().unwrap()),
            })
            .collect::<std::collections::VecDeque<_>>();

        let root = Rc::new(RefCell::new(TreeNode::new(
            data.pop_front().unwrap().unwrap(),
        )));

        Self::_deserialize(&root, &mut data);

        Some(root)
    }
    fn _deserialize(
        root: &Rc<RefCell<TreeNode>>,
        que: &mut std::collections::VecDeque<Option<i32>>,
    ) {
        if que.is_empty() {
            return;
        }
        if let Some(Some(v)) = que.pop_front() {
            root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        };
        if let Some(Some(v)) = que.pop_front() {
            root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        };

        if let Some(left) = &root.borrow().left {
            Self::_deserialize(&left, que);
        };
        if let Some(right) = &root.borrow().right {
            Self::_deserialize(&right, que);
        };
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
// @lc code=end
