# 347.前 K 个高频元素

<https://leetcode-cn.com/problems/top-k-frequent-elements/description/>

给你一个整数数组 `nums` 和一个整数 `k` ，请你返回其中出现频率前 `k` 高的元素。你可以按 任意顺序 返回答案。

示例 1:

```txt
输入: nums = [1,1,1,2,2,3], k = 2
输出: [1,2]
```

示例 2:

```txt
输入: nums = [1], k = 1
输出: [1]
```

提示：

- `1 <= nums.length <= 10^5`
- `k` 的取值范围是 `[1, 数组中不相同的元素的个数]`
- 题目数据保证答案唯一，换句话说，数组中前 `k` 个高频元素的集合是唯一的

进阶：你所设计算法的时间复杂度 必须 优于 $O(n log n)$ ，其中 `n` 是数组大小。

## 总结

条件

- 【TODO】

### k-堆

分析

- 【TODO】
  - 维护一个只有k个元素的最小堆，将统计出来的元素出现次数放入堆中：
    - 堆节点小于k个：直接入堆
    - 堆节点等于k个：比较堆顶和当前遍历的元素出现次数：
      - 堆顶更大：跳过当前遍历的元素出现次数
      - 堆顶更小：取出堆顶后入堆。因为当前遍历的元素出现次数比堆顶元素更多，更有资格进入top-K
  - 遍历结束后，堆就是要找的前k个元素

算法

```TODO
```

复杂度

- 时间：O(TODO)
- 空间：O(TODO)

### 快速选择

分析

- 【TODO】
  - 统计元素出现次数组成次数数组后，除了直接排序来确定前k元外，可以借鉴快排的思想
  - 借鉴快排的维护有序区间的思想，每次选择一个元素作为有序区间的守卫元，开辟并一个比守卫元小的有序区间，然后比较有序区间的长度$l$和$k$：
    - $l=k$：找到了top-K，返回结果
    - $l<k$：在守卫元的右侧有序区间中选择一个新的守卫元，并仅对该区间数据再进行一次类似快排的步骤
    - $l>k$：在守卫元的左侧有序区间中选择一个新的守卫元，并仅对该区间数据再进行一次类似快排的步骤

算法

```TODO
```

复杂度

- 时间：O(TODO)
- 空间：O(TODO)