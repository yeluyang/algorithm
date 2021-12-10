# 278.第一个错误的版本

<https://leetcode-cn.com/problems/first-bad-version/description/>

你是产品经理，目前正在带领一个团队开发新的产品。不幸的是，你的产品的最新版本没有通过质量检测。由于每个版本都是基于之前的版本开发的，所以错误的版本之后的所有版本都是错的。

假设你有 `n` 个版本 `[1, 2, ..., n]`，你想找出导致之后所有版本出错的第一个错误的版本。

你可以通过调用 `bool isBadVersion(version)` 接口来判断版本号 `version` 是否在单元测试中出错。实现一个函数来查找第一个错误的版本。你应该尽量减少对调用 API 的次数。

示例 1：

```txt
输入：n = 5, bad = 4
输出：4
解释：
调用 isBadVersion(3) -> false
调用 isBadVersion(5) -> true
调用 isBadVersion(4) -> true
所以，4 是第一个错误的版本。
```

示例 2：

```txt
输入：n = 1, bad = 1
输出：1
```

提示：

- `1 <= bad <= n <= 2^31 - 1`

## 总结

条件

- 【数组】
- 【有序】

分析

- 【TODO】

### 二分查找最先出现

分析

- 【TODO】

实现

1. 从中间元素开始二分查找：
   1. API返回false则向后查找
   1. API返回true则向前查找
1. 全部元素查找完后，返回二分查找的左边界

复杂度

- 时间：O(TODO)
- 空间：O(TODO)
