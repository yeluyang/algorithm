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
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let is_positive = dividend.signum() == divisor.signum();
        let dividend = if dividend.is_positive() {
            -dividend
        } else {
            dividend
        };
        let divisor = if divisor.is_positive() {
            -divisor
        } else {
            divisor
        };

        let mut quotient = 0;
        let mut left = 0;
        let mut right = dividend.unsigned_abs();
        while left <= right {
            let mid = ((right - left) >> 1) + left;
            let checked = Self::check(dividend, divisor, mid);
            if checked {
                quotient = mid;
                if let Some(l) = mid.checked_add(1) {
                    left = l;
                } else {
                    break;
                }
            } else {
                if let Some(r) = mid.checked_sub(1) {
                    right = r;
                } else {
                    break;
                }
            };
        }
        match (is_positive, quotient >= i32::MIN.unsigned_abs()) {
            (true, true) => i32::MAX,
            (true, false) => quotient as i32,
            (false, true) => i32::MIN,
            (false, false) => -(quotient as i32),
        }
    }

    fn check(dividend: i32, divisor: i32, quotient: u32) -> bool {
        let mut acc = divisor;
        let mut quotient = quotient;

        let mut product: i32 = 0;
        while quotient > 0 {
            if quotient & 0b1 != 0 {
                if let Some(p) = product.checked_add(acc) {
                    if p < dividend {
                        return false;
                    }
                    product = p;
                } else {
                    return false;
                };
            }

            quotient >>= 1;
            if quotient == 0 {
                return true;
            }

            if let Some(a) = acc.checked_add(acc) {
                if a < dividend {
                    return false;
                }
                acc = a;
            } else {
                return false;
            }
        }
        true
    }
}
// @lc code=end
