/*
 * @lc app=leetcode.cn id=79 lang=rust
 *
 * [79] 单词搜索
 *
 * https://leetcode-cn.com/problems/word-search/description/
 *
 * algorithms
 * Medium (45.94%)
 * Likes:    1137
 * Dislikes: 0
 * Total Accepted:    252.6K
 * Total Submissions: 548.9K
 * Testcase Example:  '[["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]\n"ABCCED"'
 *
 * 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false
 * 。
 *
 * 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word =
 * "ABCCED"
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word =
 * "SEE"
 * 输出：true
 *
 *
 * 示例 3：
 *
 *
 * 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word =
 * "ABCB"
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == board.length
 * n = board[i].length
 * 1
 * 1
 * board 和 word 仅由大小写英文字母组成
 *
 *
 *
 *
 * 进阶：你可以使用搜索剪枝的技术来优化解决方案，使其在 board 更大的情况下可以更快解决问题？
 *
 */

// @lc code=start
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let cs = word.as_bytes();
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == cs[0] as char && backtracking(&mut board, &cs[1..], i, j) {
                    return true;
                }
            }
        }
        false
    }
}
fn backtracking(board: &mut Vec<Vec<char>>, word: &[u8], i: usize, j: usize) -> bool {
    if word.is_empty() {
        true
    } else {
        let tmp = board[i][j];
        board[i][j] = '\0';
        let ret = if (i > 0
            && board[i - 1][j] == word[0] as char
            && backtracking(board, &word[1..], i - 1, j))
            || (i < board.len() - 1
                && board[i + 1][j] == word[0] as char
                && backtracking(board, &word[1..], i + 1, j))
            || (j > 0
                && board[i][j - 1] == word[0] as char
                && backtracking(board, &word[1..], i, j - 1))
            || (j < board[i].len() - 1
                && board[i][j + 1] == word[0] as char
                && backtracking(board, &word[1..], i, j + 1))
        {
            true
        } else {
            false
        };
        board[i][j] = tmp;
        ret
    }
}
// @lc code=end
