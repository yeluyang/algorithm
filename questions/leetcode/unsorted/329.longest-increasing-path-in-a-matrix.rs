/*
 * @lc app=leetcode.cn id=329 lang=rust
 *
 * [329] 矩阵中的最长递增路径
 *
 * https://leetcode.cn/problems/longest-increasing-path-in-a-matrix/description/
 *
 * algorithms
 * Hard (51.90%)
 * Likes:    821
 * Dislikes: 0
 * Total Accepted:    103.9K
 * Total Submissions: 199.9K
 * Testcase Example:  '[[9,9,4],[6,6,8],[2,1,1]]'
 *
 * 给定一个 m x n 整数矩阵 matrix ，找出其中 最长递增路径 的长度。
 *
 * 对于每个单元格，你可以往上，下，左，右四个方向移动。 你 不能 在 对角线 方向上移动或移动到 边界外（即不允许环绕）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：matrix = [[9,9,4],[6,6,8],[2,1,1]]
 * 输出：4
 * 解释：最长递增路径为 [1, 2, 6, 9]。
 *
 * 示例 2：
 *
 *
 * 输入：matrix = [[3,4,5],[3,2,6],[2,2,1]]
 * 输出：4
 * 解释：最长递增路径是 [3, 4, 5, 6]。注意不允许在对角线方向上移动。
 *
 *
 * 示例 3：
 *
 *
 * 输入：matrix = [[1]]
 * 输出：1
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
 * 0
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        Graph::new(&matrix).longest_increasing_path()
    }
}

struct Node {
    longest_path: std::cell::RefCell<i32>,
    adjacent_nodes: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Self {
            longest_path: std::cell::RefCell::new(0),
            adjacent_nodes: vec![],
        }
    }
}

struct Graph {
    nodes: Vec<Node>,
    longest_path: std::cell::RefCell<i32>,
}

impl Graph {
    fn new(matrix: &Vec<Vec<i32>>) -> Self {
        if matrix.is_empty() {
            return Self {
                nodes: Vec::with_capacity(0),
                longest_path: std::cell::RefCell::new(1),
            };
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut graph = Self {
            nodes: Vec::with_capacity(rows * cols),
            longest_path: std::cell::RefCell::new(1),
        };
        for row in 0..rows {
            for col in 0..cols {
                let mut node = Node::new();
                if let Some(r) = row.checked_sub(1) {
                    if matrix[row][col] < matrix[r][col] {
                        node.adjacent_nodes.push(r * cols + col);
                    }
                }
                if row + 1 < rows {
                    if matrix[row][col] < matrix[row + 1][col] {
                        node.adjacent_nodes.push((row + 1) * cols + col);
                    }
                }
                if let Some(c) = col.checked_sub(1) {
                    if matrix[row][col] < matrix[row][c] {
                        node.adjacent_nodes.push(row * cols + c);
                    }
                }
                if col + 1 < cols {
                    if matrix[row][col] < matrix[row][col + 1] {
                        node.adjacent_nodes.push(row * cols + col + 1);
                    }
                }

                graph.nodes.push(node);
            }
        }
        graph
    }

    fn dfs_update_longest_path(&self, indicate: usize) {
        let cur = &self.nodes[indicate];
        if *cur.longest_path.borrow() != 0 {
            return;
        }
        cur.longest_path.replace(1);
        for i in 0..self.nodes[indicate].adjacent_nodes.len() {
            let next_i = self.nodes[indicate].adjacent_nodes[i];
            let next = &self.nodes[next_i];
            if *next.longest_path.borrow() == 0 {
                self.dfs_update_longest_path(next_i);
            };
            let next_max = *next.longest_path.borrow() + 1;
            if next_max > *cur.longest_path.borrow() {
                cur.longest_path.replace(next_max);
                if *cur.longest_path.borrow() > *self.longest_path.borrow() {
                    self.longest_path.replace(*cur.longest_path.borrow());
                };
            };
        }
    }

    fn longest_increasing_path(&self) -> i32 {
        for i in 0..self.nodes.len() {
            self.dfs_update_longest_path(i);
        }
        *self.longest_path.borrow()
    }
}
// @lc code=end
