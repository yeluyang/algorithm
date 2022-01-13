/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] 子集
 *
 * https://leetcode-cn.com/problems/subsets/description/
 *
 * algorithms
 * Medium (80.20%)
 * Likes:    1440
 * Dislikes: 0
 * Total Accepted:    351.3K
 * Total Submissions: 437.8K
 * Testcase Example:  '[1,2,3]'
 *
 * 给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。
 *
 * 解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,3]
 * 输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0]
 * 输出：[[],[0]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * -10
 * nums 中的所有元素 互不相同
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut result = Vec::new();
        let mut buf = Vec::new();
        backtracking(nums.as_mut_slice(), &mut result, &mut buf);
        result
    }
}
fn backtracking(nums: &mut [i32], result: &mut Vec<Vec<i32>>, buf: &mut Vec<i32>) {
    if nums.is_empty() {
        result.push(buf.clone());
    } else {
        backtracking(&mut nums[1..], result, buf);
        buf.push(nums[0]);
        backtracking(&mut nums[1..], result, buf);
        buf.pop();
    }
}
// @lc code=end
