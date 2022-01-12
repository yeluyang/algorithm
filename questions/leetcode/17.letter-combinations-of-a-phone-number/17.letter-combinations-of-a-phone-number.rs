/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 *
 * https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (57.44%)
 * Likes:    1665
 * Dislikes: 0
 * Total Accepted:    381.5K
 * Total Submissions: 663.4K
 * Testcase Example:  '"23"'
 *
 * 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。
 *
 * 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：digits = "23"
 * 输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 *
 * 示例 2：
 *
 *
 * 输入：digits = ""
 * 输出：[]
 *
 *
 * 示例 3：
 *
 *
 * 输入：digits = "2"
 * 输出：["a","b","c"]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= digits.length <= 4
 * digits[i] 是范围 ['2', '9'] 的一个数字。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut alpha = std::collections::HashMap::new();
        alpha.insert(b'2', "abc".to_owned());
        alpha.insert(b'3', "def".to_owned());
        alpha.insert(b'4', "ghi".to_owned());
        alpha.insert(b'5', "jkl".to_owned());
        alpha.insert(b'6', "mno".to_owned());
        alpha.insert(b'7', "pqrs".to_owned());
        alpha.insert(b'8', "tuv".to_owned());
        alpha.insert(b'9', "wxyz".to_owned());
        let mut ret = Vec::with_capacity(3usize.pow(digits.len() as u32));
        backtracking(
            &digits,
            &alpha,
            &mut ret,
            &mut String::with_capacity(digits.len()),
        );
        ret
    }
}
fn backtracking(
    digits: &str,
    alpha: &std::collections::HashMap<u8, String>,
    result: &mut Vec<String>,
    buf: &mut String,
) {
    if digits.is_empty() {
        if !buf.is_empty() {
            result.push(buf.clone())
        }
    } else {
        for c in alpha.get(&digits.as_bytes()[0]).unwrap().chars() {
            buf.push(c);
            backtracking(&digits[1..], alpha, result, buf);
            buf.pop();
        }
    }
}
// @lc code=end
