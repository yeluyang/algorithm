/*
 * @lc app=leetcode.cn id=287 lang=rust
 *
 * [287] 寻找重复数
 *
 * https://leetcode.cn/problems/find-the-duplicate-number/description/
 *
 * algorithms
 * Medium (64.05%)
 * Likes:    2318
 * Dislikes: 0
 * Total Accepted:    352.3K
 * Total Submissions: 550K
 * Testcase Example:  '[1,3,4,2,2]'
 *
 * 给定一个包含 n + 1 个整数的数组 nums ，其数字都在 [1, n] 范围内（包括 1 和 n），可知至少存在一个重复的整数。
 *
 * 假设 nums 只有 一个重复的整数 ，返回 这个重复的数 。
 *
 * 你设计的解决方案必须 不修改 数组 nums 且只用常量级 O(1) 的额外空间。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,3,4,2,2]
 * 输出：2
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,1,3,4,2]
 * 输出：3
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 10^5
 * nums.length == n + 1
 * 1 <= nums[i] <= n
 * nums 中 只有一个整数 出现 两次或多次 ，其余整数均只出现 一次
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 如何证明 nums 中至少存在一个重复的数字?
 * 你可以设计一个线性级时间复杂度 O(n) 的解决方案吗？
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let bits_max = {
            // 19 bits already over 10^5
            let mut bits_max = 19u8;
            let mut left = 0u8;
            let mut right = 18u8;
            while left <= right {
                let mid: u8 = ((right - left) >> 2) + left;
                let mask = 1 << mid;
                if mask >= nums.len() {
                    bits_max = mid + 1;
                    if mask == nums.len() {
                        break;
                    }
                    right = mid - 1;
                } else {
                    left = mid + 1;
                };
            }
            bits_max
        };

        let mut result = 0i32;
        for k in 0..bits_max {
            let mask: i32 = 1 << k;
            let mut range_bit_count = 0u32;
            let mut value_bit_count = 0u32;
            for (i, n) in nums.iter().enumerate() {
                if i as i32 & mask != 0 {
                    range_bit_count += 1;
                }
                if *n & mask != 0 {
                    value_bit_count += 1;
                }
            }
            if value_bit_count > range_bit_count {
                result |= mask;
            }
        }

        result
    }
}
// @lc code=end
