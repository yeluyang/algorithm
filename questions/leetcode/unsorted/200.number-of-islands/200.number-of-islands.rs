/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 *
 * https://leetcode-cn.com/problems/number-of-islands/description/
 *
 * algorithms
 * Medium (56.52%)
 * Likes:    1502
 * Dislikes: 0
 * Total Accepted:    380.4K
 * Total Submissions: 672.9K
 * Testcase Example:  '[["1","1","1","1","0"],["1","1","0","1","0"],["1","1","0","0","0"],["0","0","0","0","0"]]'
 *
 * 给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。
 *
 * 岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。
 *
 * 此外，你可以假设该网格的四条边均被水包围。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：grid = [
 * ⁠ ["1","1","1","1","0"],
 * ⁠ ["1","1","0","1","0"],
 * ⁠ ["1","1","0","0","0"],
 * ⁠ ["0","0","0","0","0"]
 * ]
 * 输出：1
 *
 *
 * 示例 2：
 *
 *
 * 输入：grid = [
 * ⁠ ["1","1","0","0","0"],
 * ⁠ ["1","1","0","0","0"],
 * ⁠ ["0","0","1","0","0"],
 * ⁠ ["0","0","0","1","1"]
 * ]
 * 输出：3
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1
 * grid[i][j] 的值为 '0' 或 '1'
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut count = 0usize;
        let mut que = std::collections::VecDeque::with_capacity(grid.len());
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == '1' {
                    grid[row][col] = '0';
                    count += 1;
                    que.push_back((row, col));
                    while let Some((i, j)) = que.pop_front() {
                        if i > 0 && grid[i - 1][j] == '1' {
                            grid[i - 1][j] = '0';
                            que.push_back((i - 1, j));
                        }
                        if j > 0 && grid[i][j - 1] == '1' {
                            grid[i][j - 1] = '0';
                            que.push_back((i, j - 1));
                        }
                        if i + 1 < grid.len() && grid[i + 1][j] == '1' {
                            grid[i + 1][j] = '0';
                            que.push_back((i + 1, j));
                        }
                        if j + 1 < grid[row].len() && grid[i][j + 1] == '1' {
                            grid[i][j + 1] = '0';
                            que.push_back((i, j + 1));
                        }
                    }
                }
            }
        }
        count as i32
    }
}
// @lc code=end
