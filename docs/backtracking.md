# 回溯法

具有回溯法性质的问题是不能直接用多层循环解决的，因为回溯问题都有一个特性，那就是循环层数是不定的、动态的，而循环代码是静态的、有固定的循环层数。

回溯法的重点在构建决策树：

- 定义决策树有几种分叉
- 定义决策树每种分叉的前提

回溯法的实现重点在：

- 定义每次决策的可选策略动态集合及其维护方式
- 定义撤销操作的回溯方法
- 在决策树叶子节点记录结果

相关题目：

- [17.电话号码的字母组合](questions/leetcode/17.letter-combinations-of-a-phone-number/README.md)
- [22.括号生成](questions/leetcode/22.generate-parentheses/README.md)
- [46.全排列](questions/leetcode/46.permutations/README.md)
- [78.子集](questions/leetcode/78.subsets/README.md)
- [79.单词搜索](questions/leetcode/79.word-search/README.md)
