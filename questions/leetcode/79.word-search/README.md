# 79.单词搜索

<https://leetcode-cn.com/problems/word-search/description/>

给定一个`m x n` 二维字符网格`board` 和一个字符串单词`word` 。如果`word` 存在于网格中，返回 `true` ；否则，返回 `false` 。

单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。

示例 1：
![word2.jpg](https://assets.leetcode.com/uploads/2020/11/04/word2.jpg)

```txt
输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
输出：true
```

示例 2：
![word-1.jpg](https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg)

```txt
输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
输出：true
```

示例 3：
![word3.jpg](https://assets.leetcode.com/uploads/2020/10/15/word3.jpg)

```txt
输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
输出：false
```

提示：

- `m == board.length`
- `n = board[i].length`
- `1 <= m, n <= 6`
- `1 <= word.length <= 15`
- `board` 和 `word` 仅由大小写英文字母组成

进阶：你可以使用搜索剪枝的技术来优化解决方案，使其在 `board` 更大的情况下可以更快解决问题？

## 总结

条件

- 【TODO】

分析

- 【TODO】

### 回溯法

分析

- 【回溯法】
  - 决策分支
    - 策略集合: 策略集合是混合的, 可选策略是当前遍历矩阵元素的上下左右四个选择
    - 策略条件: 每个决策的前提条件是被选中元素等于下一个寻找的字符
  - 决策路径
    - 终点叶子: 当要寻找的字符集合为空时, 就到了终点叶子
    - 结果记录: 没有决策序列要记录, 直接返回是否存在子序列的布尔值即可
  - 决策回滚: 每次决策时都把当前元素修改为特殊值, 决策回溯后恢复原值

实现

```rust
fn exist(board: Vec<Vec<char>>, word: String) -> bool {
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
fn backtracking(board: &mut Vec<Vec<char>>, word: &[u8], i: usize, j: usize) -> bool {
    if word.is_empty() {
        // 终点叶子
        true
    } else {
        let tmp = board[i][j];
        board[i][j] = '\0';
        // 决策分支
        let ret = if (i > 0 // 决策条件
            && board[i - 1][j] == word[0] as char
            && backtracking(board, &word[1..], i - 1, j)) // 策略集合
            || (i < board.len() - 1 // 决策条件
                && board[i + 1][j] == word[0] as char
                && backtracking(board, &word[1..], i + 1, j)) // 策略集合
            || (j > 0 // 决策条件
                && board[i][j - 1] == word[0] as char
                && backtracking(board, &word[1..], i, j - 1)) // 策略集合
            || (j < board[i].len() - 1 // 决策条件
                && board[i][j + 1] == word[0] as char
                && backtracking(board, &word[1..], i, j + 1)) // 策略集合
        {
            true
        } else {
            false
        };
        // 决策回滚
        board[i][j] = tmp;
        ret
    }
}
```

复杂度

- 时间：O(TODO)
- 空间：O(TODO)
