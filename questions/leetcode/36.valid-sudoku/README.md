# 36.有效的数独

<https://leetcode-cn.com/problems/valid-sudoku/description/>

请你判断一个`9 x 9` 的数独是否有效。只需要 根据以下规则 ，验证已经填入的数字是否有效即可。

1. 数字`1-9`在每一行只能出现一次。
1. 数字`1-9`在每一列只能出现一次。
1. 数字`1-9`在每一个以粗实线分隔的`3x3`宫内只能出现一次。（请参考示例图）

注意：

- 一个有效的数独（部分已被填充）不一定是可解的。
- 只需要根据以上规则，验证已经填入的数字是否有效即可。
- 空白格用`'.'`表示。

示例 1：

![250px-sudoku-by-l2g-20050714svg.png](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714svg.png)

```txt
输入：board =
[["5","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
输出：true
```

示例 2：

```txt
输入：board =
[["8","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
输出：false
解释：除了第一行的第一个数字从 5 改为 8 以外，空格内其他数字均与 示例1 相同。 但由于位于左上角的 3x3 宫内有两个 8 存在, 因此这个数独是无效的。
```

提示：

- `board.length == 9`
- `board[i].length == 9`
- `board[i][j]` 是一位数字（`1-9`）或者 `'.'`

## 总结

条件

- 【数组】数据存储结构为纯数组，可以任意顺序读取或随机读取。尾部插入删除操作开销低，其他位置插入删除操作开销大

分析

- 【集合环境依赖】
  - 数独是否有效的判定，可以解释为数独矩阵每个数字是否有效的判定
  - 数度矩阵每个元素是否有效的判定，可解释为该元素在三个集合中是否有效的判定
  - 每个元素在对应的三个集合中是否有效的判定，取决于三个集合内部其他元素的状态。具体来说该元素值对应的列集合、行集合和宫集合的其他元素有无重复值

- 【定长布尔状态集】
  - 因为三个元素状态集只需要记录元素是否存在即可，可解释为布尔值
  - 又因为元素的键值就是元素值本身而其值域只有1-9，是固定且较小的值域
  - 所以可以用一个长度合适的位串来记录元素状态，元素键值映射对应的位是0还是1来记录元素是否存在

### 【哈希】+【布尔串】

实现

- 构建三个哈希表：
  - 行哈希表：以行号为键，以位数大于等于列数n的位串为值
  - 列哈希表：以列号为键，以位数大于等于行数n的位串为值
  - 宫哈希表：以宫号为键，以位数大于等于宫数n的位串为值
- 遍历矩阵每个元素(i,j)：
  - 将1按位右移，偏移量等于该元素值，所得结果作为元素模式
  - 元素模式与行哈希表第i行对应的位串按位与操作：
    - 结果==0：将元素模式以按位或操作的方式更新到行哈希表第i行对应的位串
    - 结果!=0：判别为无效数独并返回结果
  - 元素模式与列哈希表第j列对应的位串按位与操作：
    - 结果==0：将元素模式以按位或操作的方式更新到列哈希表第j列对应的位串
    - 结果!=0：判别为无效数独并返回结果
  - 元素模式与宫哈希表第(i/3+j/3)宫对应的位串按位与操作：
    - 结果==0：将元素模式以按位或操作的方式更新到宫哈希表第(i/3+j/3)宫对应的位串
    - 结果!=0：判别为无效数独并返回结果

复杂度：n是数独矩阵的行数、列数和3宫格数

- 时间：O(n * n)
- 空间：O(n + n + n)
