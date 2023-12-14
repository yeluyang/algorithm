/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] 两数相除
 *
 * https://leetcode.cn/problems/divide-two-integers/description/
 *
 * algorithms
 * Medium (22.25%)
 * Likes:    1205
 * Dislikes: 0
 * Total Accepted:    225.9K
 * Total Submissions: 1M
 * Testcase Example:  '10\n3'
 *
 * 给你两个整数，被除数 dividend 和除数 divisor。将两数相除，要求 不使用 乘法、除法和取余运算。
 *
 * 整数除法应该向零截断，也就是截去（truncate）其小数部分。例如，8.345 将被截断为 8 ，-2.7335 将被截断至 -2 。
 *
 * 返回被除数 dividend 除以除数 divisor 得到的 商 。
 *
 * 注意：假设我们的环境只能存储 32 位 有符号整数，其数值范围是 [−2^31,  2^31 − 1] 。本题中，如果商 严格大于 2^31 − 1
 * ，则返回 2^31 − 1 ；如果商 严格小于 -2^31 ，则返回 -2^31^ 。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: dividend = 10, divisor = 3
 * 输出: 3
 * 解释: 10/3 = 3.33333.. ，向零截断后得到 3 。
 *
 * 示例 2:
 *
 *
 * 输入: dividend = 7, divisor = -3
 * 输出: -2
 * 解释: 7/-3 = -2.33333.. ，向零截断后得到 -2 。
 *
 *
 *
 * 提示：
 *
 *
 * -2^31 <= dividend, divisor <= 2^31 - 1
 * divisor != 0
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {}
    const NEGATIVE_MASK: i32 = 0b10000000__00000000__00000000__00000000u32 as i32;
    pub fn multiple(num: i32, times: i32) -> i32 {
        let sig = if num & Self::NEGATIVE_MASK == times & Self::NEGATIVE_MASK {
            0
        } else {
            Self::NEGATIVE_MASK
        };
        let num = if num < 0 {
            !(num as u32) + 1
        } else {
            num as u32
        };
        let times = if times < 0 {
            !(times as u32) + 1
        } else {
            times as u32
        };

        let mut x = num as u32;
        let mut m = 1u32;

        let mut r = 0;
        for _ in 0..32 {
            if m > times {
                break;
            }
            if times & m != 0 {
                r += x;
            }
            x += x;
            m <<= 1;
        }
        (if sig == 0 {
            r // positive
        } else {
            !(r - 1) // negative
        }) as i32
    }
}
// @lc code=end
