/*
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * [207] 课程表
 *
 * https://leetcode.cn/problems/course-schedule/description/
 *
 * algorithms
 * Medium (53.86%)
 * Likes:    1861
 * Dislikes: 0
 * Total Accepted:    364.1K
 * Total Submissions: 675.8K
 * Testcase Example:  '2\n[[1,0]]'
 *
 * 你这个学期必须选修 numCourses 门课程，记为 0 到 numCourses - 1 。
 *
 * 在选修某些课程之前需要一些先修课程。 先修课程按数组 prerequisites 给出，其中 prerequisites[i] = [ai, bi]
 * ，表示如果要学习课程 ai 则 必须 先学习课程  bi 。
 *
 *
 * 例如，先修课程对 [0, 1] 表示：想要学习课程 0 ，你需要先完成课程 1 。
 *
 *
 * 请你判断是否可能完成所有课程的学习？如果可以，返回 true ；否则，返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：numCourses = 2, prerequisites = [[1,0]]
 * 输出：true
 * 解释：总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。
 *
 * 示例 2：
 *
 *
 * 输入：numCourses = 2, prerequisites = [[1,0],[0,1]]
 * 输出：false
 * 解释：总共有 2 门课程。学习课程 1 之前，你需要先完成​课程 0 ；并且学习课程 0 之前，你还应先完成课程 1 。这是不可能的。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= numCourses <= 2000
 * 0 <= prerequisites.length <= 5000
 * prerequisites[i].length == 2
 * 0 <= ai, bi < numCourses
 * prerequisites[i] 中的所有课程对 互不相同
 *
 *
 */

// @lc code=start
#[derive(Clone)]
enum Status {
    Searching,
    Done,
}
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: Vec<(Option<Status>, Vec<i32>)> = vec![(None, vec![]); num_courses as usize];
        for p in prerequisites {
            graph[p[1] as usize].1.push(p[0]);
        }
        for i in 0..num_courses {
            match graph[i as usize].0 {
                None => {
                    if !Self::dfs(&mut graph, i) {
                        return false;
                    }
                }
                Some(Status::Searching) => {
                    return false;
                }
                Some(Status::Done) => continue,
            };
        }
        true
    }
    fn dfs(graph: &mut Vec<(Option<Status>, Vec<i32>)>, cur: i32) -> bool {
        match graph[cur as usize].0 {
            None => {
                graph[cur as usize].0.replace(Status::Searching);
            }
            Some(Status::Searching) => {
                return false;
            }
            Some(Status::Done) => {
                return true;
            }
        };
        for i in 0..graph[cur as usize].1.len() {
            if !Self::dfs(graph, graph[cur as usize].1[i]) {
                return false;
            };
        }
        graph[cur as usize].0.replace(Status::Done);
        true
    }
}
// @lc code=end
