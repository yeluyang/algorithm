/*
 * @lc app=leetcode.cn id=124 lang=rust
 *
 * [124] 二叉树中的最大路径和
 *
 * https://leetcode.cn/problems/binary-tree-maximum-path-sum/description/
 *
 * algorithms
 * Hard (45.34%)
 * Likes:    2141
 * Dislikes: 0
 * Total Accepted:    368.5K
 * Total Submissions: 812.5K
 * Testcase Example:  '[1,2,3]'
 *
 * 二叉树中的 路径 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个
 * 节点，且不一定经过根节点。
 *
 * 路径和 是路径中各节点值的总和。
 *
 * 给你一个二叉树的根节点 root ，返回其 最大路径和 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：root = [1,2,3]
 * 输出：6
 * 解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6
 *
 * 示例 2：
 *
 *
 * 输入：root = [-10,9,20,null,null,15,7]
 * 输出：42
 * 解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42
 *
 *
 *
 *
 * 提示：
 *
 *
 * 树中节点数目范围是 [1, 3 * 10^4]
 * -1000 <= Node.val <= 1000
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        if let Some(root) = root {
            Self::_max_path_sum(root, &mut max_sum);
        }
        max_sum
    }
    fn _max_path_sum(root: Rc<RefCell<TreeNode>>, max_sum: &mut i32) -> i32 {
        let left_sum = if let Some(left) = root.borrow().left.clone() {
            Self::_max_path_sum(left, max_sum)
        } else {
            0
        };
        let right_sum = if let Some(right) = root.borrow().right.clone() {
            Self::_max_path_sum(right, max_sum)
        } else {
            0
        };
        *max_sum = std::cmp::max(
            *max_sum,
            root.borrow().val
                + match (left_sum > 0, right_sum > 0) {
                    (true, true) => left_sum + right_sum,
                    (true, false) => left_sum,
                    (false, true) => right_sum,
                    (false, false) => 0,
                },
        );
        std::cmp::max(
            root.borrow().val,
            std::cmp::max(root.borrow().val + left_sum, root.borrow().val + right_sum),
        )
    }
}
// @lc code=end
