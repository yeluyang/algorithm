/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长递增子序列
 *
 * https://leetcode-cn.com/problems/longest-increasing-subsequence/description/
 *
 * algorithms
 * Medium (52.36%)
 * Likes:    2166
 * Dislikes: 0
 * Total Accepted:    416.3K
 * Total Submissions: 794.4K
 * Testcase Example:  '[10,9,2,5,3,7,101,18]'
 *
 * 给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。
 *
 * 子序列是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7]
 * 的子序列。
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [10,9,2,5,3,7,101,18]
 * 输出：4
 * 解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0,1,0,3,2,3]
 * 输出：4
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [7,7,7,7,7,7,7]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * -10^4
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 你可以设计时间复杂度为 O(n^2) 的解决方案吗？
 * 你能将算法的时间复杂度降低到 O(n log(n)) 吗?
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            0
        } else {
            let mut dp = Vec::with_capacity(nums.len());
            dp.push(nums[0]);
            for i in 1..nums.len() {
                if *dp.last().unwrap() < nums[i] {
                    dp.push(nums[i]);
                } else {
                    let mut l = 0;
                    let mut r = dp.len() - 1;
                    while l + 1 < r {
                        let m = (r - l) / 2 + l;
                        if dp[m] < nums[i] {
                            l = m;
                        } else {
                            r = m - 1;
                        }
                    }
                    if dp[r] < nums[i] {
                        dp[r + 1] = nums[i];
                    } else if dp[l] < nums[i] {
                        dp[l + 1] = nums[i];
                    } else {
                        dp[0] = nums[i];
                    }
                };
            }

            dp.len() as i32
        }
    }
}
// @lc code=end
