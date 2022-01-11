/*
 * @lc app=leetcode.cn id=280 lang=rust
 *
 * [280] 摆动排序
 *
 * https://leetcode-cn.com/problems/wiggle-sort/description/
 *
 * algorithms
 * Medium (68.37%)
 * Likes:    85
 * Dislikes: 0
 * Total Accepted:    6.3K
 * Total Submissions: 9.2K
 * Testcase Example:  '[3,5,2,1,6,4]'
 *
 * 给你一个无序的数组 nums, 将该数字 原地 重排后使得 nums[0] <= nums[1] >= nums[2] <= nums[3]...。
 *
 * 示例:
 *
 * 输入: nums = [3,5,2,1,6,4]
 * 输出: 一个可能的解答是 [3,5,1,6,2,4]
 *
 */

// @lc code=start
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut less = false;
        for i in 0..nums.len() - 1 {
            if (less && nums[i] < nums[i + 1]) || (!less && nums[i] > nums[i + 1]) {
                nums.swap(i, i + 1);
            }
            less = !less;
        }
    }
}
// @lc code=end
