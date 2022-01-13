# 78.子集

<https://leetcode-cn.com/problems/subsets/description/>

给你一个整数数组`nums` ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。

解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。

示例 1：

```txt
输入：nums = [1,2,3]
输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
```

示例 2：

```txt
输入：nums = [0]
输出：[[],[0]]
```

提示：

- `1 <= nums.length <= 10`
- `-10 <= nums[i] <= 10`
- `nums` 中的所有元素 互不相同

## 总结

条件

- 【TODO】

分析

- 【TODO】

### 回溯法

分析

- 【回溯法】
  - 决策分支
    - 策略集合：可选策略集合是混合的，对当前遍历的元素，只有“选中”或“不选”两种策略
    - 决策条件：无条件决策
  - 决策路径
    - 终点叶子：当遍历完所有元素时，就是到达了终点叶子
    - 结果记录：每次选择策略时，选择了“选中”就将元素插入缓冲区，选择了“不选”就插入缓冲区。到达叶子后，整个缓冲区记录到最终结果
  - 决策回滚：
    - 被回溯的策略如果是“选中”，则从缓冲区删除刚刚插入的元素

实现

```rust
fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut result = Vec::new(); // 最终结果
    let mut buf = Vec::new(); // 缓冲区
    backtracking(nums.as_mut_slice(), &mut result, &mut buf);
    result
}
fn backtracking(nums: &mut [i32], result: &mut Vec<Vec<i32>>, buf: &mut Vec<i32>) {
    if nums.is_empty() {
        // 终点叶子
        result.push(buf.clone()); // 结果记录
    } else {
        // 决策分支
        backtracking(&mut nums[1..], result, buf);
        // 决策分支
        buf.push(nums[0]);
        backtracking(&mut nums[1..], result, buf);
        buf.pop(); // 决策回滚
    }
}
```

复杂度

- 时间：O($n \times 2^n$)
- 空间：O(n)
