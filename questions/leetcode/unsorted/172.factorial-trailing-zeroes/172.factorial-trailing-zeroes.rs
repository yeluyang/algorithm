/*
 * @lc app=leetcode.cn id=172 lang=rust
 *
 * [172] 阶乘后的零
 *
 * https://leetcode-cn.com/problems/factorial-trailing-zeroes/description/
 *
 * algorithms
 * Medium (48.17%)
 * Likes:    670
 * Dislikes: 0
 * Total Accepted:    138.7K
 * Total Submissions: 287.8K
 * Testcase Example:  '3'
 *
 * 给定一个整数 n ，返回 n! 结果中尾随零的数量。
 *
 * 提示 n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1
 *
 *
 * 示例 1：
 *
 * 输入：n = 3
 * 输出：0
 * 解释：3! = 6 ，不含尾随 0
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 5
 * 输出：1
 * 解释：5! = 120 ，有一个尾随 0
 *
 *
 * 示例 3：
 *
 *
 * 输入：n = 0
 * 输出：0
 *
 *
 * 提示：
 *
 * 0 <= n <= 10^4
 *
 *
 * 进阶：你可以设计并实现对数时间复杂度的算法来解决此问题吗？
 *
 */

// @lc code=start
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut count = 0;
        while n > 0 {
            n /= 5;
            count += n;
        }
        count
    }
}
// @lc code=end
