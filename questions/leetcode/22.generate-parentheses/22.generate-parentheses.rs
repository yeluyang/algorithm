/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 *
 * https://leetcode-cn.com/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (77.29%)
 * Likes:    2236
 * Dislikes: 0
 * Total Accepted:    395.4K
 * Total Submissions: 511.6K
 * Testcase Example:  '3'
 *
 * 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 3
 * 输出：["((()))","(()())","(())()","()(())","()()()"]
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：["()"]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 8
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret = Vec::with_capacity(n as usize);
        let mut buf = String::with_capacity(2 * (n as usize));
        backtracking(n as usize, 0, 0, &mut ret, &mut buf);
        ret
    }
}
fn backtracking(max: usize, left: usize, right: usize, result: &mut Vec<String>, buf: &mut String) {
    if left == max && left == right {
        result.push(buf.clone());
    } else {
        if left < max {
            buf.push('(');
            backtracking(max, left + 1, right, result, buf);
            buf.pop();
        }
        if right < left {
            buf.push(')');
            backtracking(max, left, right + 1, result, buf);
            buf.pop();
        }
    }
}
// @lc code=end
