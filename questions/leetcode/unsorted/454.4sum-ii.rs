/*
 * @lc app=leetcode.cn id=454 lang=rust
 *
 * [454] 四数相加 II
 *
 * https://leetcode.cn/problems/4sum-ii/description/
 *
 * algorithms
 * Medium (64.27%)
 * Likes:    944
 * Dislikes: 0
 * Total Accepted:    240.7K
 * Total Submissions: 374.3K
 * Testcase Example:  '[1,2]\n[-2,-1]\n[-1,2]\n[0,2]'
 *
 * 给你四个整数数组 nums1、nums2、nums3 和 nums4 ，数组长度都是 n ，请你计算有多少个元组 (i, j, k, l)
 * 能满足：
 *
 *
 * 0 <= i, j, k, l < n
 * nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
 * 输出：2
 * 解释：
 * 两个元组如下：
 * 1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) +
 * (-1) + 2 = 0
 * 2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) +
 * (-1) + 0 = 0
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums1 = [0], nums2 = [0], nums3 = [0], nums4 = [0]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == nums1.length
 * n == nums2.length
 * n == nums3.length
 * n == nums4.length
 * 1 <= n <= 200
 * -2^28 <= nums1[i], nums2[i], nums3[i], nums4[i] <= 2^28
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let len = nums1.len();
        let buf_len = len.pow(2);
        let mut nums12 = std::collections::HashMap::with_capacity(buf_len);
        let mut nums34 = Vec::with_capacity(buf_len);
        for i in 0..len {
            for j in 0..len {
                let sum = nums1[i] + nums2[j];
                if let Some(count) = nums12.get_mut(&sum) {
                    *count += 1;
                } else {
                    nums12.insert(sum, 1);
                }
                nums34.push(nums3[i] + nums4[j]);
            }
        }
        let mut result = 0i32;
        for i in 0..buf_len {
            if let Some(count) = nums12.get(&-nums34[i]) {
                result += count;
            };
        }
        result
    }
}
// @lc code=end
