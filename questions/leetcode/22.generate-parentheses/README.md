# 22.括号生成

<https://leetcode-cn.com/problems/generate-parentheses/description/>

数字 `n`代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

示例 1：

```txt
输入：n = 3
输出：["((()))","(()())","(())()","()(())","()()()"]
```

示例 2：

```txt
输入：n = 1
输出：["()"]
```

提示：

- `1 <= n <= 8`

## 总结

条件

- 【TODO】TODO

分析

- 【TODO】

### 回溯

分析

- 【回溯法】
  - 决策分支：
    - 策略集合：策略集合是静态的，要么放左括号，要么放右括号
    - 决策条件：
      - 左括号决策前提：左括号数量 小于 括号总对数
      - 右括号决策前提：右括号数量 小于 左括号数量
  - 决策路径：
    - 终点叶子：左括号数量、右括号数量、括号总对数三者相等
    - 结果记录：每次决策都把决策写入缓冲区，到达终点叶子时，缓冲区包含完整的决策路径，整个写入最终结果
  - 决策回滚：每次回溯都从缓冲区删除刚刚插入的决策

实现

```rust
fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut ret = Vec::with_capacity(n as usize); // 最终结果
    let mut buf = String::with_capacity(2 * (n as usize)); // 缓冲区
    backtracking(n as usize, 0, 0, &mut ret, &mut buf);
    ret
}
fn backtracking(max: usize, left: usize, right: usize, result: &mut Vec<String>, buf: &mut String) {
    if left == max && left == right {
        // 终点叶子
        result.push(buf.clone()); // 结果记录
    } else {
        if left < max {
            // 决策条件
            buf.push('('); // 策略集合
            backtracking(max, left + 1, right, result, buf);
            buf.pop(); // 决策回滚
        }
        if right < left {
            // 决策条件
            buf.push(')'); // 策略集合
            backtracking(max, left, right + 1, result, buf);
            buf.pop(); // 决策回滚
        }
    }
}
```

复杂度

- 时间：O(TODO)
- 空间：O(TODO)
