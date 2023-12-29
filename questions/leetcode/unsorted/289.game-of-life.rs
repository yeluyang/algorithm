/*
 * @lc app=leetcode.cn id=289 lang=rust
 *
 * [289] 生命游戏
 *
 * https://leetcode.cn/problems/game-of-life/description/
 *
 * algorithms
 * Medium (75.90%)
 * Likes:    561
 * Dislikes: 0
 * Total Accepted:    90.2K
 * Total Submissions: 118.7K
 * Testcase Example:  '[[0,1,0],[0,0,1],[1,1,1],[0,0,0]]'
 *
 * 根据 百度百科 ， 生命游戏 ，简称为 生命 ，是英国数学家约翰·何顿·康威在 1970 年发明的细胞自动机。
 *
 * 给定一个包含 m × n 个格子的面板，每一个格子都可以看成是一个细胞。每个细胞都具有一个初始状态： 1 即为 活细胞 （live），或 0 即为
 * 死细胞 （dead）。每个细胞与其八个相邻位置（水平，垂直，对角线）的细胞都遵循以下四条生存定律：
 *
 *
 * 如果活细胞周围八个位置的活细胞数少于两个，则该位置活细胞死亡；
 * 如果活细胞周围八个位置有两个或三个活细胞，则该位置活细胞仍然存活；
 * 如果活细胞周围八个位置有超过三个活细胞，则该位置活细胞死亡；
 * 如果死细胞周围正好有三个活细胞，则该位置死细胞复活；
 *
 *
 * 下一个状态是通过将上述规则同时应用于当前状态下的每个细胞所形成的，其中细胞的出生和死亡是同时发生的。给你 m x n 网格面板 board
 * 的当前状态，返回下一个状态。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
 * 输出：[[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：board = [[1,1],[1,0]]
 * 输出：[[1,1],[1,1]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == board.length
 * n == board[i].length
 * 1 <= m, n <= 25
 * board[i][j] 为 0 或 1
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 你可以使用原地算法解决本题吗？请注意，面板上所有格子需要同时被更新：你不能先更新某些格子，然后使用它们的更新后的值再更新其他格子。
 * 本题中，我们使用二维数组来表示面板。原则上，面板是无限的，但当活细胞侵占了面板边界时会造成问题。你将如何解决这些问题？
 *
 *
 */

// @lc code=start
impl Solution {
    const DEATH_TO_LIVE: i32 = 2;
    const LIVE_TO_DEATH: i32 = 3;
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        for m in 0..board.len() {
            for n in 0..board[m].len() {
                let mut lives = 0u8;
                let top = m.checked_sub(1).or(Some(m)).unwrap();
                let bottom = if m + 1 >= board.len() { m } else { m + 1 };
                let left = n.checked_sub(1).or(Some(n)).unwrap();
                let right = if n + 1 >= board[m].len() { n } else { n + 1 };
                for i in top..=bottom {
                    for j in left..=right {
                        if i == m && j == n {
                            continue;
                        }
                        if board[i][j] == 1 || board[i][j] == Self::LIVE_TO_DEATH {
                            lives += 1;
                        }
                    }
                }
                let live_in_next_epoch = match board[m][n] {
                    0 => lives == 3,
                    1 => lives == 2 || lives == 3,
                    _ => unreachable!(),
                };
                board[m][n] = match (board[m][n] == 1, live_in_next_epoch) {
                    (false, false) => 0,                  // death to death
                    (false, true) => Self::DEATH_TO_LIVE, // death to live
                    (true, false) => Self::LIVE_TO_DEATH, // live to death
                    (true, true) => 1,                    // live to live
                };
            }
        }
        for m in 0..board.len() {
            for n in 0..board[m].len() {
                match board[m][n] {
                    Self::LIVE_TO_DEATH => {
                        board[m][n] = 0;
                    }
                    Self::DEATH_TO_LIVE => {
                        board[m][n] = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}
// @lc code=end
