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

- 【TODO】

  遍历矩阵每一个元素，假设以该元素为起点去寻找目标序列。如果当前元素能够和目标序列匹配，就将当前元素修改为空，然后寻找周围元素有无匹配的，有则以相邻匹配元素进行递归，无则恢复被修改的当前元素，结束递归去遍历下一矩阵元素

实现

```TODO
```

复杂度

- 时间：O(TODO)
- 空间：O(TODO)
