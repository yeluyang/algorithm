# 280.摆动排序

<https://leetcode-cn.com/problems/wiggle-sort/description/>

给你一个无序的数组`nums`, 将该数字原地重排后使得`nums[0] <= nums[1] >= nums[2] <= nums[3]...`。

示例:

```txt
输入: nums = [3,5,2,1,6,4]
输出: 一个可能的解答是 [3,5,1,6,2,4]
```

## 总结

条件

- 【TODO】

分析

- 【TODO】

### 断言区间

分析

- 【断言区间】

  输入数据头部开辟一个符合“该区间内数据摆动排序”的断言区间，此时称区间最末尾的（被区间包含）的元素为中心元素，则中心元素的前一个（区间倒数第二个）元素称为前序元素，中心元素的后一个（区间外第一个）元素称为后序元素：

  - 区间最末是上升浪（前序小于中心）
    - 区间外是上升浪（中心小于后序）：交换中心元素和后序元素，使之称为下跌浪，然后扩大断言区间
    - 区间外是下跌浪（中心大于后序）：无需处理，直接扩大断言区间
  - 区间最末是下跌浪（前序大于中心）
    - 区间外是上升浪（中心小于后序）：无需处理，直接扩大断言区间
    - 区间外是下跌浪（中心大于后序）：交换中心元素和后序元素，使之称为上升浪，然后扩大断言区间

实现

```rust
pub fn wiggle_sort(nums: &mut Vec<i32>) {
    let mut less = false;
    for i in 0..nums.len() - 1 {
        if (less && nums[i] < nums[i + 1]) || (!less && nums[i] > nums[i + 1]) {
            nums.swap(i, i + 1);
        }
        less = !less;
    }
}
```

复杂度

- 时间：O(n)
- 空间：O(1)