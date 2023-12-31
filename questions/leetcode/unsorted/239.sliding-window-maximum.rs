/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 *
 * https://leetcode.cn/problems/sliding-window-maximum/description/
 *
 * algorithms
 * Hard (49.20%)
 * Likes:    2646
 * Dislikes: 0
 * Total Accepted:    526.4K
 * Total Submissions: 1.1M
 * Testcase Example:  '[1,3,-1,-3,5,3,6,7]\n3'
 *
 * 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k
 * 个数字。滑动窗口每次只向右移动一位。
 *
 * 返回 滑动窗口中的最大值 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
 * 输出：[3,3,5,5,6,7]
 * 解释：
 * 滑动窗口的位置                最大值
 * ---------------             -----
 * [1  3  -1] -3  5  3  6  7       3
 *  1 [3  -1  -3] 5  3  6  7       3
 *  1  3 [-1  -3  5] 3  6  7       5
 *  1  3  -1 [-3  5  3] 6  7       5
 *  1  3  -1  -3 [5  3  6] 7       6
 *  1  3  -1  -3  5 [3  6  7]      7
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1], k = 1
 * 输出：[1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 * 1 <= k <= nums.length
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut increments: std::collections::VecDeque<(usize, i32)> =
            std::collections::VecDeque::new();
        for i in 0..k {
            while !increments.is_empty() && increments.back().unwrap().1 <= nums[i] {
                increments.pop_back();
            }
            increments.push_back((i, nums[i]));
        }
        let mut result: Vec<i32> = Vec::with_capacity(nums.len() - k + 1);
        result.push(increments.front().unwrap().1);
        for i in k..nums.len() {
            while !increments.is_empty() && increments.back().unwrap().1 <= nums[i] {
                increments.pop_back();
            }
            increments.push_back((i, nums[i]));

            if increments.front().unwrap().0 < i - k + 1 {
                increments.pop_front();
            };

            result.push(increments.front().unwrap().1);
        }

        result
    }
}
// @lc code=end
