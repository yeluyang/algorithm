# 70.爬楼梯

<https://leetcode-cn.com/problems/climbing-stairs/description/>

假设你正在爬楼梯。需要 n阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

注意：给定 n 是一个正整数。

示例 1：

```txt
输入： 2
输出： 2
解释： 有两种方法可以爬到楼顶。
1.  1 阶 + 1 阶
2.  2 阶
```

示例 2：

```txt
输入： 3
输出： 3
解释： 有三种方法可以爬到楼顶。
1.  1 阶 + 1 阶 + 1 阶
2.  1 阶 + 2 阶
3.  2 阶 + 1 阶
```

## 总结

条件

- 【TODO】

分析

- 【可分治】问题的解可由子问题的解组合而来

### 动态规划

分析

- 【TODO】

实现

1. 检查n是否小于等于1：
   - 是：返回1
   - 否：
     1. 递归计算n-1阶的结果
     1. 递归计算n-2阶的结果
     1. 将两个递归结果相加并返回

复杂度

- 时间：O(TODO)
- 空间：O(TODO)

### 动态规划+记忆化搜索+记忆简化

分析

- 【TODO】

实现

1. 设置变量v0=1，表示爬0级台阶有1种方法；设置变量v1=1，表示爬1级台阶有1种方法
1. 要爬n级台阶则迭代n-2次，每次迭代：
   1. 设置临时变量，值等于v0+v1
   1. 将v1的值赋给v0
   1. 将临时变量的值赋给v1
1. 返回v0+v1的和

复杂度

- 时间：O(TODO)
- 空间：O(TODO)

### 动态规划+数学建模：矩阵快速幂

分析

- 【齐次线性递推式】
  - 动态规划递推式为：$f(n) = f(n-1) + f(n-2)$，这是齐次线性方程，可以直接转换为矩阵递推式：

    $
    \begin{bmatrix}
    f(n+1) \\
    f(n)
    \end{bmatrix} = \begin{bmatrix}
    f(n) + f(n-1) \\
    f(n)
    \end{bmatrix} = \begin{bmatrix}
    1 & 1 \\
    1 & 0
    \end{bmatrix} \begin{bmatrix}
    f(n) \\
    f(n-1)
    \end{bmatrix} = \begin{bmatrix}
    1 & 1 \\
    1 & 0
    \end{bmatrix}^n \begin{bmatrix}
    f(1) \\
    f(0)
    \end{bmatrix}
    $

  - 根据以上递推式结果，可以使用快速幂来求系数矩阵，从而求出$f(n)$，即爬n级有多少种方法的解

实现

1. TODO

复杂度

- 时间：O(TODO)
- 空间：O(TODO)

### 动态规划+数学建模：通项公式

分析

- 【TODO】
  - 对任意的$a_{n+2} = pa_{n+1} + qa_n$，都可以令$a_n = x^n$代入求得$x^2 = px + q$。该性质对$f(n) = f(n-1) + f(n-2)$也是成立的
  - 令$f(n) = x^n$，可得$x^2 = x + 1$，求解得$x_1 = \frac{1 + \sqrt{5}}{2}$，$x_2 = \frac{1 - \sqrt{5}}{2}$
  - 设通解为$f(n) = c_1 x^n_{1} + c_2 x^n_{2}$，代入递推初始条件$f(0) = 1$和$f(1) = 1$，可得$c_1 = \frac{1}{\sqrt{5}}$，$c_2 = -\frac{1}{\sqrt{5}}$
  - 则$f(n) = \frac{1}{\sqrt{5}} \left[ \left( \frac{1 + \sqrt{5}}{2} \right)^n - \left( \frac{1 - \sqrt{5}}{2} \right)^n \right]$

实现

1. TODO

复杂度

- 时间：O(TODO)
- 空间：O(TODO)
