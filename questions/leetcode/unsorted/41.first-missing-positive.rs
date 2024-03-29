/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 *
 * https://leetcode.cn/problems/first-missing-positive/description/
 *
 * algorithms
 * Hard (43.66%)
 * Likes:    2013
 * Dislikes: 0
 * Total Accepted:    341.1K
 * Total Submissions: 780K
 * Testcase Example:  '[1,2,0]'
 *
 * 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
 * 请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,0]
 * 输出：3
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,4,-1,1]
 * 输出：2
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [7,8,9,11,12]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * -2^31
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut i = 0usize;
        while i < nums.len() {
            if nums[i] <= 0 {
                i += 1;
            } else {
                let n = nums[i] as usize;
                if n > 0 && n <= nums.len() && nums[n - 1] != n as i32 {
                    nums.swap(i, n - 1);
                } else {
                    i += 1;
                }
            }
        }
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        return nums.len() as i32 + 1;
    }
}
// @lc code=end
