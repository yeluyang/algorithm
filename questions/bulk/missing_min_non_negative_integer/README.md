# 消失的最小非负整数

找出数组中最小的、没有出现的非负整数

- 示例1
  - 输入：[-1, 0, 1, 12]
  - 输出：2

- 示例2
  - 输入：[0, 1, 2]
  - 输出：3

- 示例3
  - 输入：[1, 2, 3]
  - 输出：0

## 总结

条件

- 【TODO】

分析

- 【TODO】

### TODO

分析

- 【TODO】

实现

```rust
fn missing_min_non_negative_int(mut nums: Vec<i32>) -> u32 {
    for i in 0..nums.len() {
        // 舍弃数组长度范围外的元素和负数，然后遍历处理元素值与下标值不相等的元素
        if nums[i] >= 0 && nums[i] < nums.len() as i32 && (nums[i] as usize) != i {
            let mut j = nums[i];
            nums[i] = -1;
            // 元素归位，令元素值等同于下标值
            while j >= 0 && (j as usize) < nums.len() && nums[j as usize] != j {
                swap(&mut nums[j as usize], &mut j);
            }
        }
    }
    for i in 0..nums.len() {
        // 检查没有对应元素的位置
        if nums[i] != i as i32 {
            return i as u32;
        }
    }
    // 所有位置上都有对应元素
    nums.len() as u32
}
```

复杂度

- 时间：O(n)
- 空间：O(1)
