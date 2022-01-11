# 生成扫雷游戏地图

给定行数m、列数n和地雷数k，生成一个含有k个地雷的`m x n`的随机矩阵代表扫雷游戏地图

- 示例1
  - 输入：m=5, n=4, k=3
  - 输出：

    ```text
    [
        [1,  -1, 1,  0]
        [2,  2,  2,  0]
        [1,  -1, 2,  1]
        [1,  2,  -1, 1]
        [0,  1,  1,  1]
    ]
    ```

## 总结

条件

- 【TODO】

分析

- 【TODO】

### TODO

分析

- 【不放回抽样】

  随机选择地雷本身时不放回抽样，如果代码实现时简单随机就会变成重试放回抽样直到抽出不重复选择，当随机量大时，时间复杂度会高。

  将所有样本构造好来模拟不放回抽样，每次做出的选择都放在样本尾部，并在下一次随机选择时缩小样本空间

实现

```rust
fn gen_minesweeper(m: usize, n: usize, k: usize) -> Vec<Vec<i32>> {
    let mine = {
        let mut buf = Vec::with_capacity(m * n);
        for i in 0..m * n {
            buf.push(i);
        }
        let mut ret = Vec::with_capacity(k);
        for i in 0..k {
            // 每一次随机选择都会比上次多排除掉尾部一个元素
            let r = rand::random::<usize>() % (m * n - i);
            ret.push(buf[r]);
            // 将本次选择到的元素置换到尾部，这样下次随机时会排除掉本次的选择
            buf.swap(r, m * n - i - 1)
        }
        ret
    };
    let mut ret = vec![vec![0; n]; m];
    for l in mine {
        let i = l / n;
        let j = l % n;
        // 设置地雷
        ret[i][j] = -1;
        for i in i - 1..=i + 1 {
            for j in j - 1..=j + 1 {
                // 地雷周围的数字+1
                if ret[i][j] >= 0 {
                    ret[i][j] += 1;
                }
            }
        }
    }
    ret
}
```

复杂度

- 时间：O(m*n)
- 空间：O(m*n)
