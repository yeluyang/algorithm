# 461.汉明距离

<https://leetcode-cn.com/problems/hamming-distance/description/>

两个整数之间的 [汉明距离](https://baike.baidu.com/item/%E6%B1%89%E6%98%8E%E8%B7%9D%E7%A6%BB) 指的是这两个数字对应二进制位不同的位置的数目。

给你两个整数 `x` 和 `y`，计算并返回它们之间的汉明距离。

示例 1：

```txt
输入：x = 1, y = 4
输出：2
解释：
1   (0 0 0 1)
4   (0 1 0 0)
       ↑   ↑
上面的箭头指出了对应二进制位不同的位置。
```

示例 2：

```txt
输入：x = 3, y = 1
输出：1
```

提示：

- `0 <= x, y <= 2^31 - 1`