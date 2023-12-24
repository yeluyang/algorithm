/*
 * @lc app=leetcode.cn id=380 lang=rust
 *
 * [380] O(1) 时间插入、删除和获取随机元素
 *
 * https://leetcode.cn/problems/insert-delete-getrandom-o1/description/
 *
 * algorithms
 * Medium (52.39%)
 * Likes:    759
 * Dislikes: 0
 * Total Accepted:    133.5K
 * Total Submissions: 255K
 * Testcase Example:  '["RandomizedSet","insert","remove","insert","getRandom","remove","insert","getRandom"]\n' +
  '[[],[1],[2],[2],[],[1],[2],[]]'
 *
 * 实现RandomizedSet 类：
 *
 *
 *
 *
 * RandomizedSet() 初始化 RandomizedSet 对象
 * bool insert(int val) 当元素 val 不存在时，向集合中插入该项，并返回 true ；否则，返回 false 。
 * bool remove(int val) 当元素 val 存在时，从集合中移除该项，并返回 true ；否则，返回 false 。
 * int getRandom() 随机返回现有集合中的一项（测试用例保证调用此方法时集合中至少存在一个元素）。每个元素应该有 相同的概率 被返回。
 *
 *
 * 你必须实现类的所有函数，并满足每个函数的 平均 时间复杂度为 O(1) 。
 *
 *
 *
 * 示例：
 *
 *
 * 输入
 * ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove",
 * "insert", "getRandom"]
 * [[], [1], [2], [2], [], [1], [2], []]
 * 输出
 * [null, true, false, true, 2, true, false, 2]
 *
 * 解释
 * RandomizedSet randomizedSet = new RandomizedSet();
 * randomizedSet.insert(1); // 向集合中插入 1 。返回 true 表示 1 被成功地插入。
 * randomizedSet.remove(2); // 返回 false ，表示集合中不存在 2 。
 * randomizedSet.insert(2); // 向集合中插入 2 。返回 true 。集合现在包含 [1,2] 。
 * randomizedSet.getRandom(); // getRandom 应随机返回 1 或 2 。
 * randomizedSet.remove(1); // 从集合中移除 1 ，返回 true 。集合现在包含 [2] 。
 * randomizedSet.insert(2); // 2 已在集合中，所以返回 false 。
 * randomizedSet.getRandom(); // 由于 2 是集合中唯一的数字，getRandom 总是返回 2 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * -2^31 <= val <= 2^31 - 1
 * 最多调用 insert、remove 和 getRandom 函数 2 * 10^5 次
 * 在调用 getRandom 方法时，数据结构中 至少存在一个 元素。
 *
 *
 *
 *
 */

// @lc code=start
struct RandomizedSet {
    map: std::collections::HashMap<i32, usize>,
    vec: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
            vec: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            self.vec.push(val);
            self.map.insert(val, self.vec.len() - 1);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.map.remove(&val) {
            if self.map.is_empty() || i == self.vec.len() - 1 {
                self.vec.pop();
            } else {
                let last_index = self.vec.len() - 1;
                self.vec[i] = self.vec[last_index];
                self.vec.pop();
                self.map.insert(self.vec[i], i);
            }
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        self.vec[std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize
            % self.vec.len()]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
// @lc code=end
