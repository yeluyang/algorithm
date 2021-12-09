# 105.从前序与中序遍历序列构造二叉树

<https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/>

给定一棵树的前序遍历 `preorder` 与中序遍历  `inorder`。请构造二叉树并返回其根节点。

示例 1:
![tree.jpg](https://assets.leetcode.com/uploads/2021/02/19/tree.jpg)

```txt
Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
Output: [3,9,20,null,null,15,7]
```

示例 2:

```txt
Input: preorder = [-1], inorder = [-1]
Output: [-1]
```

提示:

- `1 <= preorder.length <= 3000`
- `inorder.length == preorder.length`
- `-3000 <= preorder[i], inorder[i] <= 3000`
- `preorder` 和 `inorder` 均无重复元素
- `inorder` 均出现在 `preorder`
- `preorder` 保证为二叉树的前序遍历序列
- `inorder` 保证为二叉树的中序遍历序列
