/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并 K 个升序链表
 *
 * https://leetcode.cn/problems/merge-k-sorted-lists/description/
 *
 * algorithms
 * Hard (58.70%)
 * Likes:    2729
 * Dislikes: 0
 * Total Accepted:    742.3K
 * Total Submissions: 1.3M
 * Testcase Example:  '[[1,4,5],[1,3,4],[2,6]]'
 *
 * 给你一个链表数组，每个链表都已经按升序排列。
 *
 * 请你将所有链表合并到一个升序链表中，返回合并后的链表。
 *
 *
 *
 * 示例 1：
 *
 * 输入：lists = [[1,4,5],[1,3,4],[2,6]]
 * 输出：[1,1,2,3,4,4,5,6]
 * 解释：链表数组如下：
 * [
 * ⁠ 1->4->5,
 * ⁠ 1->3->4,
 * ⁠ 2->6
 * ]
 * 将它们合并到一个有序链表中得到。
 * 1->1->2->3->4->4->5->6
 *
 *
 * 示例 2：
 *
 * 输入：lists = []
 * 输出：[]
 *
 *
 * 示例 3：
 *
 * 输入：lists = [[]]
 * 输出：[]
 *
 *
 *
 *
 * 提示：
 *
 *
 * k == lists.length
 * 0 <= k <= 10^4
 * 0 <= lists[i].length <= 500
 * -10^4 <= lists[i][j] <= 10^4
 * lists[i] 按 升序 排列
 * lists[i].length 的总和不超过 10^4
 *
 *
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap_min = std::collections::BinaryHeap::with_capacity(lists.len());
        for (i, n) in lists.iter().enumerate() {
            if let Some(n) = n {
                heap_min.push(std::cmp::Reverse((n.val, i)));
            }
        }

        let mut result: Option<Box<ListNode>> = None;
        let mut last: &mut Option<Box<ListNode>> = &mut result;
        while let Some(std::cmp::Reverse(v)) = heap_min.pop() {
            // specify indicate
            let i = v.1;
            // take node from lists
            if let Some(mut current) = lists[i].take() {
                // update next node into lists
                lists[i] = current.next.take();
                if let Some(ref next) = lists[i] {
                    // push next node into heap
                    heap_min.push(std::cmp::Reverse((next.val, i)));
                };
                // update result
                if let Some(ref mut l) = last {
                    l.next = Some(current);
                    last = &mut l.next;
                } else {
                    last.replace(current);
                };
            };
        }

        result
    }
}
// @lc code=end
