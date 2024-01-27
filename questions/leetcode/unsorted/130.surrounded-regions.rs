/*
 * @lc app=leetcode.cn id=130 lang=rust
 *
 * [130] 被围绕的区域
 *
 * https://leetcode.cn/problems/surrounded-regions/description/
 *
 * algorithms
 * Medium (46.29%)
 * Likes:    1071
 * Dislikes: 0
 * Total Accepted:    256.3K
 * Total Submissions: 553.3K
 * Testcase Example:  '[["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]'
 *
 * 给你一个 m x n 的矩阵 board ，由若干字符 'X' 和 'O' ，找到所有被 'X' 围绕的区域，并将这些区域里所有的 'O' 用 'X'
 * 填充。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：board =
 * [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
 * 输出：[["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
 * 解释：被围绕的区间不会存在于边界上，换句话说，任何边界上的 'O' 都不会被填充为 'X'。 任何不在边界上，或不与边界上的 'O' 相连的 'O'
 * 最终都会被填充为 'X'。如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。
 *
 *
 * 示例 2：
 *
 *
 * 输入：board = [["X"]]
 * 输出：[["X"]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == board.length
 * n == board[i].length
 * 1
 * board[i][j] 为 'X' 或 'O'
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        let bottom = board.len() - 1;
        let right = board[0].len() - 1;

        for i in 0..=bottom {
            if board[i][0] == 'O' {
                Self::_solve(board, &i, &0)
            }
            if board[i][right] == 'O' {
                Self::_solve(board, &i, &right)
            }
        }
        for i in 0..=right {
            if board[0][i] == 'O' {
                Self::_solve(board, &0, &i)
            }
            if board[bottom][i] == 'O' {
                Self::_solve(board, &bottom, &i)
            }
        }
        for i in 0..=bottom {
            for j in 0..=right {
                match board[i][j] {
                    'O' => board[i][j] = 'X',
                    'M' => board[i][j] = 'O',
                    _ => {}
                }
            }
        }
    }

    fn _solve(board: &mut Vec<Vec<char>>, x: &usize, y: &usize) {
        board[*x][*y] = 'M';
        if *x > 0 && board[*x - 1][*y] == 'O' {
            Self::_solve(board, &(x - 1), y);
        };
        if *x < board.len() - 1 && board[x + 1][*y] == 'O' {
            Self::_solve(board, &(x + 1), y);
        };

        if *y > 0 && board[*x][y - 1] == 'O' {
            Self::_solve(board, x, &(y - 1));
        };
        if *y < board[*x].len() - 1 && board[*x][y + 1] == 'O' {
            Self::_solve(board, x, &(y + 1));
        };
    }
}
// @lc code=end
