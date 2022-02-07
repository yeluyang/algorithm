# 371.两整数之和

<https://leetcode-cn.com/problems/sum-of-two-integers/description/>

给你两个整数 `a` 和 `b` ，不使用 运算符 `+` 和 `-` ​​​​​​​，计算并返回两整数之和。

示例 1：

```text
输入：a = 1, b = 2
输出：3
```

示例 2：

```text
输入：a = 2, b = 3
输出：5
```

提示：

- `-1000 <= a, b <= 1000`

## 总结

条件

- 【TODO】

分析

- 【TODO】

### 位运算

分析

- 【TODO】

  加法运算在硬件层面也是门电路组成的位运算电路. 从逻辑上看, 一次加法运算包括位加法和进位:

  - 加法结果 = 位异或结果
  - 进位结果 = 位与并左移一位的结果

  换言之, 一次加法运算可以拆解为`a ^ b`和`(a & b) << 1`的和

实现

```rust
fn get_sum(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let carry = (a & b) << 1;
        a = a ^ b;
        b = carry;
    }
    a
}
```

复杂度

- 时间：O(TODO)
- 空间：O(TODO)
