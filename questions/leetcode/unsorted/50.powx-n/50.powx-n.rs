/*
 * @lc app=leetcode.cn id=50 lang=rust
 *
 * [50] Pow(x, n)
 *
 * https://leetcode.cn/problems/powx-n/description/
 *
 * algorithms
 * Medium (38.06%)
 * Likes:    1282
 * Dislikes: 0
 * Total Accepted:    412.4K
 * Total Submissions: 1.1M
 * Testcase Example:  '2.00000\n10'
 *
 * 实现 pow(x, n) ，即计算 x 的整数 n 次幂函数（即，x^n^ ）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：x = 2.00000, n = 10
 * 输出：1024.00000
 *
 *
 * 示例 2：
 *
 *
 * 输入：x = 2.10000, n = 3
 * 输出：9.26100
 *
 *
 * 示例 3：
 *
 *
 * 输入：x = 2.00000, n = -2
 * 输出：0.25000
 * 解释：2^-2 = 1/2^2 = 1/4 = 0.25
 *
 *
 *
 *
 * 提示：
 *
 *
 * -100.0 < x < 100.0
 * -2^31 <= n <= 2^31-1
 * n 是一个整数
 * 要么 x 不为零，要么 n > 0 。
 * -10^4 <= x^n <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    // 递归
    // pub fn my_pow(x: f64, n: i32) -> f64 {
    //     if n < 0 {
    //         Self::_my_pow(1.0 / x, -n)
    //     } else {
    //         Self::_my_pow(x, n)
    //     }
    // }
    // fn _my_pow(x: f64, n: i32) -> f64 {
    //     match n {
    //         0 => 1.0,
    //         1 => x,
    //         _ => {
    //             let mut r = Self::_my_pow(x, n / 2);
    //             r *= r;
    //             if n % 2 != 0 {
    //                 r *= x;
    //             }
    //             r
    //         }
    //     }
    // }
    // 迭代
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let (mut x, n) = if n < 0 {
            (1.0 / x, !(n as u32) + 1)
        } else {
            (x, n as u32)
        };
        let mut r = 1.0;
        let mut m = 1;
        for _ in 0..32 {
            if m > n {
                break;
            }
            if (n & m) > 0 {
                r *= x;
            }
            x *= x;
            m <<= 1;
        }
        r
    }
}
// @lc code=end
