# 169.多数元素

<https://leetcode-cn.com/problems/majority-element/description/>

给定一个大小为 *n* 的数组，找到其中的多数元素。多数元素是指在数组中出现次数 大于 `⌊ n/2 ⌋` 的元素。
你可以假设数组是非空的，并且给定的数组总是存在多数元素。

示例 1：

```text
输入：[3,2,3]
输出：3
```

示例 2：

```text
输入：[2,2,1,1,1,2,2]
输出：2
```

进阶：

- 尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。

## 总结

条件

- 【TODO】

分析

- 【TODO】

### Boyer-Moore投票算法

分析

- 【TODO】

  n个数的众数m的数量是大于$\lfloor\frac{n}{2}\rfloor$, 即众数m的数量比其他元素数量的总和还要多(不是一样多, 更不可能少). 这意味着:

  - 众数的出现次数减去其他所有元素出现次数总和的差仍是正整数
  - 非众数的出现次数减去其他所有元素(包括众数)出现次数总和的差必定是负整数

  综上, 我们可以把问题变成一个抵消游戏, 最终留下的就是众数.

  只记录一个数的当前出现次数, 然后遍历数组:

  - 每当遍历到相同的数: 当前出现次数+1
  - 每当遍历到不同的数: 当前出现次数-1, 如果当前出现次数为0, 就改为记录下一次遍历到的数, 并将出现次数改为1

实现

```rust
fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidata: Option<(i32, usize)> = None;
    for n in nums {
        if let Some(c) = &mut candidata {
            if c.0 == n {
                c.1 += 1;
            } else {
                c.1 -= 1;
                if c.1 == 0 {
                    candidata = None;
                }
            }
        } else {
            candidata = Some((n, 1));
        }
    }
    candidata.unwrap().0
}
```

复杂度

- 时间：O(n)
- 空间：O(1)
