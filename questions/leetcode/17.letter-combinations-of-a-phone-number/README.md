# 17.电话号码的字母组合

<https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/description/>

给定一个仅包含数字`2-9`的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。

给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。

![200px-telephone-keypad2svg.png](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/11/09/200px-telephone-keypad2svg.png)

示例 1：

```txt
输入：digits = "23"
输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
```

示例 2：

```txt
输入：digits = ""
输出：[]
```

示例 3：

```txt
输入：digits = "2"
输出：["a","b","c"]
```

提示：

- `0 <= digits.length <= 4`
- `digits[i]` 是范围 `['2', '9']` 的一个数字。

## 总结

条件

- 【TODO】TODO

分析

- 【TODO】TODO

### 回溯法

分析

- 【回溯法】题目不能通过简单遍历解决
  - 决策分支：
    - 策略集合：当前处理的输入号码对应有几个字母就有几种选择策略
    - 决策条件：当前处理的输入号码对应的各决策分支没有前提条件
  - 决策路径：
    - 终点叶子：输入号码数组为空时即为回溯终点、决策树叶子节点
    - 结果记录：每次选择时输入当前选择的字母到缓冲区，到达终点叶子时，缓冲区内容就是完整的决策路径，将其记录到最终结果
  - 决策回滚：每次递归回溯回来后，从缓冲区删除刚刚插入的选择

实现

```rust
fn letter_combinations(digits: String) -> Vec<String> {
    let alpha = HashMap::<_, _>::from_iter([
        (b'2', "abc".to_owned()),
        (b'3', "def".to_owned()),
        (b'4', "ghi".to_owned()),
        (b'5', "jkl".to_owned()),
        (b'6', "mno".to_owned()),
        (b'7', "pqrs".to_owned()),
        (b'8', "tuv".to_owned()),
        (b'9', "wxyz".to_owned()),
    ]);
    let mut ret = Vec::with_capacity(3usize.pow(digits.len() as u32));
    backtracking(
        &digits,
        &alpha,
        &mut ret,
        &mut String::with_capacity(digits.len()),
    );
    ret
}

fn backtracking(
    digits: &str,
    alpha: &HashMap<u8, String>,
    result: &mut Vec<String>,
    buf: &mut String,
) {
    if digits.is_empty() {
        // 终点叶子
        if !buf.is_empty() {
            // 结果记录
            result.push(buf.clone()) // 此时buf包含完整的决策路径
        }
    } else {
        for c in alpha.get(&digits.as_bytes()[0]).unwrap().chars() {
            // 决策分支
            buf.push(c);
            backtracking(&digits[1..], alpha, result, buf);
            // 决策回滚
            buf.pop();
        }
    }
}
```

复杂度

- 时间：O(TODO)
- 空间：O(TODO)
