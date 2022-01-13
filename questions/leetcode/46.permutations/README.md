# 46.全排列

<https://leetcode-cn.com/problems/permutations/description/>

给定一个不含重复数字的数组 `nums` ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。

示例 1：

```txt
输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
```

示例 2：

```txt
输入：nums = [0,1]
输出：[[0,1],[1,0]]
```

示例 3：

```txt
输入：nums = [1]
输出：[[1]]
```

提示：

- `1 <= nums.length <= 6`
- `-10 <= nums[i] <= 10`
- `nums` 中的所有整数 互不相同

## 总结

条件

- 【TODO】

分析

- 【TODO】

### 回溯法

分析

- 【回溯法】
  - 决策分支
    - 策略集合：可选策略集合是动态的，包含输入数据中尚未被选择的元素
      - 动态维护：每次选中策略后，都把策略与可选策略集合的首元素互换，然后将首元素以外的其他元素传给下一层决策
    - 决策条件：只要是可选策略集合里有的，都可以无条件决策
  - 决策路径
    - 终点叶子：可选策略集合为空时就到达了终点叶子
    - 结果记录：每次决策都把选中策略写入缓冲区，到达终点叶子时将整个缓冲区写入最终结果
  - 决策回滚：
    - 缓冲回溯：每次回溯都把刚刚插入的被选中的策略删除
    - 策略回溯：对被首元素置换过的动态可选策略集合进行逆运算恢复

实现

```rust
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut result = Vec::new();
    let mut buf = Vec::with_capacity(nums.len());
    backtracking(nums.as_mut_slice(), &mut result, &mut buf);
    result
}
fn backtracking(nums: &mut [i32], result: &mut Vec<Vec<i32>>, buf: &mut Vec<i32>) {
    if nums.is_empty() {
        result.push(buf.clone());
    } else {
        for i in 0..nums.len() {
            nums.swap(0, i);
            buf.push(nums[0]);
            backtracking(&mut nums[1..], result, buf);
            buf.pop();
            nums.swap(0, i);
        }
    }
}
```

复杂度

- 时间：O(n * n!)
- 空间：O(n)
