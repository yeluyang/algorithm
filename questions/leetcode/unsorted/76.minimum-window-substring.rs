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
        let mut t_map = std::collections::HashMap::<char, usize>::new();
        for c in t.chars() {
            t_map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        let chars: Vec<char> = s.chars().collect();
        let mut left = None;
        let mut right = None;
        let mut s_map = std::collections::HashMap::<char, std::collections::VecDeque<usize>>::new();
        for (i, c) in chars.iter().enumerate() {
            if t_map.contains_key(c) {
                if left.is_none() {
                    left.replace(i);
                }
                right = Some(i);
                s_map
                    .entry(*c)
                    .and_modify(|indices| indices.push_back(i))
                    .or_insert(vec![i].into());
            }
        }
        if left.is_none() || right.is_none() {
            return "".to_owned();
        }

        let mut left = left.unwrap();
        let mut right = right.unwrap();
        while s_map[&chars[left]].len() > t_map[&chars[left]] {
            s_map.get_mut(&chars[left]).unwrap().pop_front();
            loop {
                left += 1;
                if t_map.contains_key(&chars[left]) {
                    break;
                }
            }
        }
        while s_map[&chars[right]].len() > t_map[&chars[right]] {
            s_map.get_mut(&chars[right]).unwrap().pop_back();
            loop {
                right -= 1;
                if t_map.contains_key(&chars[right]) {
                    break;
                }
            }
        }

        return if right - left + 1 < t.len() {
            String::new()
        } else {
            s[left..=right].to_owned()
        };
    }
}
// @lc code=end
