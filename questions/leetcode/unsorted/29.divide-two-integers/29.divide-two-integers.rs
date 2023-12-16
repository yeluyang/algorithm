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
    impl Solution {
        const NEGATIVE_MASK: i32 = 0b10000000__00000000__00000000__00000000u32 as i32;
        pub fn divide(dividend: i32, divisor: i32) -> i32 {
            let positive = dividend & Self::NEGATIVE_MASK == divisor & Self::NEGATIVE_MASK;
            let dividend = if dividend < 0 {
                !(dividend as u32) + 1
            } else {
                dividend as u32
            };
            let divisor = if divisor < 0 {
                !(divisor as u32) + 1
            } else {
                divisor as u32
            };

            let mut last_mid = 0;
            let mut left = 0;
            let mut right = dividend;
            while left <= right {
                let mid = (right - left) / 2 + left;
                let r = Self::multiple(divisor, mid);
                if r > dividend {
                    right = mid - 1;
                } else {
                    last_mid = mid;
                    if r == dividend {
                        break;
                    }
                    left = mid + 1;
                };
            }
            (if positive {
                last_mid // positive
            } else {
                !(last_mid - 1) // negative
            }) as i32
        }

        fn multiple(num: u32, times: u32) -> u32 {
            let mut x = num;
            let mut mask = 1u32;

            let mut result = 0;
            for _ in 0..32 {
                if mask > times {
                    break;
                }
                if times & mask != 0 {
                    result += x;
                }
                x += x;
                mask <<= 1;
            }
            result
        }
    }
}
// @lc code=end
