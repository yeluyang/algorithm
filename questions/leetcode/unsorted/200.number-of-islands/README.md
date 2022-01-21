# 200.岛屿数量

<https://leetcode-cn.com/problems/number-of-islands/description/>

给你一个由 `'1'`（陆地）和 `'0'`（水）组成的的二维网格，请你计算网格中岛屿的数量。
岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。
此外，你可以假设该网格的四条边均被水包围。

示例 1：

```text
输入：grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
输出：1
```

示例 2：

```text
输入：grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
输出：3
```

提示：

- `m == grid.length`
- `n == grid[i].length`
- `1 <= m, n <= 300`
- `grid[i][j]` 的值为 `'0'` 或 `'1'`

## 总结

条件

- 【TODO】

分析

- 【TODO】

### 深度遍历

分析

- 【TODO】

实现

```rust
fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '1' {
                count += 1;
                travel(&mut grid, row, col)
            }
        }
    }
    count as i32
}
fn travel(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
    grid[row][col] = '0';
    if grid[row - 1][col] == '1' {
        travel(grid, row - 1, col);
    }
    if grid[row][col - 1] == '1' {
        travel(grid, row, col - 1);
    }
    if grid[row + 1][col] == '1' {
        travel(grid, row + 1, col);
    }
    if grid[row][col + 1] == '1' {
        travel(grid, row, col + 1);
    }
}
```

复杂度

m: 行, n: 列

- 时间：O(m*n)
- 空间：O(m*n)

### 广度遍历

分析

- 【TODO】

实现

```rust
fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '1' {
                grid[row][col] = '0';
                count += 1;
                let mut que = std::collections::VecDeque::new();
                que.push_back((row, col));
                while let Some((i, j)) = que.pop_front() {
                    if grid[i - 1][j] == '1' {
                        grid[i - 1][j] = '0';
                        que.push_back((i - 1, j));
                    }
                    if grid[i][j - 1] == '1' {
                        grid[i][j - 1] = '0';
                        que.push_back((i, j - 1));
                    }
                    if grid[i + 1][j] == '1' {
                        grid[i + 1][j] = '0';
                        que.push_back((i + 1, j));
                    }
                    if grid[i][j + 1] == '1' {
                        grid[i][j + 1] = '0';
                        que.push_back((i, j + 1));
                    }
                }
            }
        }
    }
    count as i32
}
```

复杂度

m: 行, n: 列

- 时间：O(m*n)
- 空间：O(min(m, n))
