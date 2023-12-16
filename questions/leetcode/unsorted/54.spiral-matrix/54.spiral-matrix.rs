/*
 * @lc app=leetcode.cn id=54 lang=rust
 *
 * [54] 螺旋矩阵
 *
 * https://leetcode.cn/problems/spiral-matrix/description/
 *
 * algorithms
 * Medium (49.92%)
 * Likes:    1558
 * Dislikes: 0
 * Total Accepted:    436.5K
 * Total Submissions: 873.9K
 * Testcase Example:  '[[1,2,3],[4,5,6],[7,8,9]]'
 *
 * 给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * 输出：[1,2,3,6,9,8,7,4,5]
 *
 *
 * 示例 2：
 *
 *
 * 输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
 * 输出：[1,2,3,4,8,12,11,10,9,5,6,7]
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == matrix.length
 * n == matrix[i].length
 * 1
 * -100
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut cells = Vec::with_capacity(rows * cols);

        let (mut left, mut top, mut right, mut bottom) = (0, 0, cols - 1, rows - 1);
        while left <= right && top <= bottom {
            // iter top margin
            for j in left..=right {
                cells.push(matrix[top][j]);
            }
            // iter right margin
            for i in (top + 1)..=bottom {
                cells.push(matrix[i][right]);
            }
            if left < right && top < bottom {
                // iter bottom margin
                for j in (left..=(right - 1)).rev() {
                    cells.push(matrix[bottom][j]);
                }
                // iter left margin
                for i in ((top + 1)..=(bottom - 1)).rev() {
                    cells.push(matrix[i][left]);
                }
            }

            left += 1;
            top += 1;
            right = right.saturating_sub(1);
            bottom = bottom.saturating_sub(1);
        }

        cells
    }
}
// @lc code=end
