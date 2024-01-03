/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 *
 * https://leetcode.cn/problems/minimum-window-substring/description/
 *
 * algorithms
 * Hard (45.44%)
 * Likes:    2781
 * Dislikes: 0
 * Total Accepted:    496.3K
 * Total Submissions: 1.1M
 * Testcase Example:  '"ADOBECODEBANC"\n"ABC"'
 *
 * 给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 ""
 * 。
 *
 *
 *
 * 注意：
 *
 *
 * 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
 * 如果 s 中存在这样的子串，我们保证它是唯一的答案。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "ADOBECODEBANC", t = "ABC"
 * 输出："BANC"
 * 解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "a", t = "a"
 * 输出："a"
 * 解释：整个字符串 s 是最小覆盖子串。
 *
 *
 * 示例 3:
 *
 *
 * 输入: s = "a", t = "aa"
 * 输出: ""
 * 解释: t 中两个字符 'a' 均应包含在 s 的子串中，
 * 因此没有符合条件的子字符串，返回空字符串。
 *
 *
 *
 * 提示：
 *
 *
 * ^m == s.length
 * ^n == t.length
 * 1 <= m, n <= 10^5
 * s 和 t 由英文字母组成
 *
 *
 *
 * 进阶：你能设计一个在 o(m+n) 时间内解决此问题的算法吗？
 */

// @lc code=start
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_map = std::collections::HashMap::<char, u32>::new();
        for c in t.chars() {
            t_map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        let chars: Vec<char> = s.chars().collect();
        let mut next_indices = Vec::new();
        for (i, c) in chars.iter().enumerate() {
            if t_map.contains_key(c) {
                next_indices.push(i);
            }
        }

        let mut result: Option<std::ops::Range<usize>> = None;
        let mut li = 0usize;
        let mut counter = std::collections::HashMap::<char, u32>::new();
        for ri in 0..next_indices.len() {
            let right = next_indices[ri];
            counter
                .entry(chars[right])
                .and_modify(|count| *count += 1)
                .or_insert(1);
            while (|| {
                for (c, count) in t_map.iter() {
                    if let Some(current) = counter.get(&c) {
                        if *current < *count {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                return true;
            })() && li <= ri
            {
                let left = next_indices[li];
                if let Some(ref mut r) = result {
                    if right - left + 1 < r.len() {
                        r.start = left;
                        r.end = right + 1;
                    }
                } else {
                    result = Some(std::ops::Range {
                        start: left,
                        end: right + 1,
                    });
                }

                *counter.get_mut(&chars[left]).unwrap() -= 1;
                li += 1;
            }
        }

        return if let Some(result) = result {
            s[result].to_owned()
        } else {
            String::new()
        };
    }
}
// @lc code=end
