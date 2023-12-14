# 621.任务调度器

<https://leetcode-cn.com/problems/task-scheduler/description/>

给你一个用字符数组 `tasks` 表示的 CPU 需要执行的任务列表。其中每个字母表示一种不同种类的任务。任务可以以任意顺序执行，并且每个任务都可以在 1 个单位时间内执行完。在任何一个单位时间，CPU 可以完成一个任务，或者处于待命状态。
然而，两个 相同种类 的任务之间必须有长度为整数 `n` 的冷却时间，因此至少有连续 `n` 个单位时间内 CPU 在执行不同的任务，或者在待命状态。
你需要计算完成所有任务所需要的 最短时间 。

示例 1：

```text
输入：tasks = ["A","A","A","B","B","B"], n = 2
输出：8
解释：A -> B -> (待命) -> A -> B -> (待命) -> A -> B
     在本示例中，两个相同类型任务之间必须间隔长度为 n = 2 的冷却时间，而执行一个任务只需要一个单位时间，所以中间出现了（待命）状态。
```

示例 2：

```text
输入：tasks = ["A","A","A","B","B","B"], n = 0
输出：6
解释：在这种情况下，任何大小为 6 的排列都可以满足要求，因为 n = 0
["A","A","A","B","B","B"]
["A","B","A","B","A","B"]
["B","B","B","A","A","A"]
...
诸如此类
```

示例 3：

```text
输入：tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
输出：16
解释：一种可能的解决方案是：
     A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> (待命) -> (待命) -> A -> (待命) -> (待命) -> A
```

提示：

- `1 <= task.length <= 10^4^`
- `tasks[i]` 是大写英文字母
- `n` 的取值范围为 `[0, 100]`

## 总结

条件

- 【TODO】

分析

- 【TODO】

### 构造法

分析

- 【TODO】

  总执行时间是由任务时间和等待时间构成的, 其中任务时间是固定的, 有多少任务就有多少任务时间. 因此最短执行时间意味着要规划出最短的等待时间.

  直觉上, 我们会先安排次数最多的任务, 并在该任务后面留出空格给冷却时间(X, ?, ?, X, ?, ? ...), 然后将其他任务填到这些冷却空格中(X, Y, Z, X, Y, Z ...). 如果我们按照A-Z给任务编号, 次数越多的任务越优先编号, 并将任务序列整理成矩阵, 我们有:

  $$
  \begin{bmatrix}
  A & B & C \\
  A & B & C \\
  A & B & ? \\
  A & ? & ? \\
  \end{bmatrix}
  $$

  因为任务都在同一列上, 从左到右从上到下去数, 同一列任务间隔必然是满足冷却要求的.

  矩阵中每一个问号 $?$ 都是冷却空格, 可是最后一行的冷却空格应该是要去掉的, 因为这一行的冷却空格是最长任务——A已经执行完毕, 之后也没有其他任务可执行而遗留下来的冷却空格, 此时整个任务调度应该已经结束, 自然没有必要等待冷却. 整理后有:

  $$
  \begin{bmatrix}
  \begin{array}{ccc|}
  A & B & C \\
  A & B & C \\
  A & B & ? \\
  \hline
  A \\
  \end{array}
  \end{bmatrix}
  $$

  可以看出左上角的子矩阵就是无论有无任务都必须执行或等待的任务矩阵, 算上最后一行的任务, 我们可得任务调度的最短时间下限: 令 $n$ 为冷却间隔时间, 令 $m$ 为最长任务的执行次数(即A的执行次数), 有 $(m-1) \cdot (n+1) + 1$

  $(m-1)$ 是因为最后一行不在必须矩阵内, $(n+1)$ 是除了每一单位冷却时间表示的列以外, 必须矩阵还包含了A所在的列. $+1$ 是因为最后一列还有一个任务A.

  当执行次数为$m$的任务不止A一个的时候, 我们有:

  $$
  \begin{bmatrix}
  \begin{array}{ccc|}
  A & B & C \\
  A & B & C \\
  A & B & ? \\
  \hline
  A & B \\
  \end{array}
  \end{bmatrix}
  $$

  此时就不能只 $+1$ 了, 令 $k$ 为最长任务数量(次数和A一样多的任务数量), 我们有更逼近的最短时间下界: $(m-1) \cdot (n+1) + k$

  当任务种类数量多于冷却间隔时, 我们有:

  $$
  \begin{bmatrix}
  \begin{array}{ccc|cc}
  A & B & C &D &E \\
  A & B & C &? &? \\
  A & B & ? &? &? \\
  \hline
  A & B \\
  \end{array}
  \end{bmatrix}
  $$

  从上面可以看出, 多出来的右上角矩阵中的冷却空格都是不必要的, 因为执行到这一部分时, 不等待直接执行下一任务都必然是满足冷却间隔. 而且必须矩阵内的冷却空格可以被右上矩阵的任务填充. 整理后我们有:

  $$
  \begin{bmatrix}
  \begin{array}{ccc|c}
  A & B & C &E\\
  A & B & C \\
  A & B & D \\
  \hline
  A & B \\
  \end{array}
  \end{bmatrix}
  $$

  从以上可以看出, 插入任务时, 执行次数为m的任务(次数最多的任务)可以直接构成一个$m\times k$的矩阵:

  $$
  \begin{bmatrix}
  \begin{array}{cc}
  A & B \\
  A & B \\
  A & B \\
  \hline
  A & B \\
  \end{array}
  \end{bmatrix}
  $$

  然后开始插入其他执行次数较少的任务, 每次插入都应该从第一行不包含该任务的行开始遍历到第`m-1`行(因为子矩阵行数=`m-1`, 所以填小数量任务时不能填到第m行), 每一行从第一列开始遍历到最后一列, 在找到的第一个冷却空格或空格(非必须矩阵)中填入任务, 如果没有找到则构建新的一列, 该列首元素为要插入的任务, 其余元素为空格

  该构建法保证:

  1. 每种任务在每行只有一个
  1. 每种任务的间隔必然满足冷却
  1. 如果必须矩阵外有任务被填入的话, 必须矩阵内必定被填满, 没有等待冷却的空格

  第一点和第三点是显然的, 第二点的证明如下:

  - 从第一种要插入的任务开始, 因为前面的任务都填满了`m`列, 所以该任务必然在同一列上.
  - 此时所有任务的数量最多为`m-1`:
    - 如果该任务插入`m-1`行, 则下一任务必然也在同一列上
    - 如果该任务次数为 `i`, 插不满`m-1`行, 则下一任务次数`j`有$j \leq i$. 因为要从下一任务中取$m-1-i$个任务填入上一任务所在的列, 下一任务即使不在同一列, 相隔也必然满足冷却间隔. 即:

      $$
      \begin{bmatrix}
      \begin{array}{cc}
      ... & A & B \\
      ... & A &   \\
      ... & B &   \\
      \hline
      ... &   &   \\
      \end{array}
      \end{bmatrix}
      $$

  由此我们可知, 对于`N`个任务中有`m`个任务具有最多的执行次数`k`, 且任务间隔冷却时间为`n`的情况下, 最短时间是以下二者的较大值:

  - $(m-1) \cdot (n+1) + k$: 必须矩阵外没有任务要执行时的最短执行时间
  - N: 必须矩阵外有任务要执行时, 此时没有任何必须等待冷却的空格, 所以最短执行时间就是所有任务执行时间

实现

```rust
fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let n = n as usize;
    let mut tasks_map = std::collections::HashMap::<char, usize>::new();
    for key in tasks.clone() {
        *tasks_map.entry(key).or_default() += 1;
    }
    let mut max_exec = 0usize;
    let mut max_exec_tasks = 0usize;
    for (_, v) in tasks_map {
        if v > max_exec {
            max_exec = v;
            max_exec_tasks = 1;
        } else if v == max_exec {
            max_exec_tasks += 1;
        }
    }
    std::cmp::max(tasks.len(), (max_exec - 1) * (n + 1) + max_exec_tasks) as i32
}
```

复杂度

- 时间：O(TODO)
- 空间：O(TODO)