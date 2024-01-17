/*
 * @lc app=leetcode.cn id=127 lang=rust
 *
 * [127] 单词接龙
 *
 * https://leetcode.cn/problems/word-ladder/description/
 *
 * algorithms
 * Hard (48.36%)
 * Likes:    1334
 * Dislikes: 0
 * Total Accepted:    201.7K
 * Total Submissions: 416.5K
 * Testcase Example:  '"hit"\n"cog"\n["hot","dot","dog","lot","log","cog"]'
 *
 * 字典 wordList 中从单词 beginWord 和 endWord 的 转换序列 是一个按下述规格形成的序列 beginWord -> s1 ->
 * s2 -> ... -> sk：
 *
 *
 * 每一对相邻的单词只差一个字母。
 * 对于 1 <= i <= k 时，每个 si 都在 wordList 中。注意， beginWord 不需要在 wordList 中。
 * sk == endWord
 *
 *
 * 给你两个单词 beginWord 和 endWord 和一个字典 wordList ，返回 从 beginWord 到 endWord 的 最短转换序列
 * 中的 单词数目 。如果不存在这样的转换序列，返回 0 。
 *
 *
 * 示例 1：
 *
 *
 * 输入：beginWord = "hit", endWord = "cog", wordList =
 * ["hot","dot","dog","lot","log","cog"]
 * 输出：5
 * 解释：一个最短转换序列是 "hit" -> "hot" -> "dot" -> "dog" -> "cog", 返回它的长度 5。
 *
 *
 * 示例 2：
 *
 *
 * 输入：beginWord = "hit", endWord = "cog", wordList =
 * ["hot","dot","dog","lot","log"]
 * 输出：0
 * 解释：endWord "cog" 不在字典中，所以无法进行转换。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= beginWord.length <= 10
 * endWord.length == beginWord.length
 * 1 <= wordList.length <= 5000
 * wordList[i].length == beginWord.length
 * beginWord、endWord 和 wordList[i] 由小写英文字母组成
 * beginWord != endWord
 * wordList 中的所有字符串 互不相同
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut graph = Graph::with_capacity(word_list.len() + 1);
        graph.insert(begin_word);
        for w in word_list {
            graph.insert(w);
        }
        let end_i = graph.indicate(&end_word);
        if end_i.is_none() {
            return 0;
        };
        let end_i = end_i.unwrap();
        let start_i = 0usize;
        unimplemented!()
    }
}

struct Node {
    start_accessed: bool,
    end_accessed: bool,
    next: Vec<usize>,
}

struct Graph {
    indicates: std::collections::HashMap<String, usize>,
    nodes: Vec<Node>,
}

impl Graph {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            indicates: std::collections::HashMap::with_capacity(capacity),
            nodes: Vec::with_capacity(capacity),
        }
    }

    fn indicate(&self, word: &str) -> Option<usize> {
        self.indicates.get(word).map(|v| *v)
    }

    fn insert(&mut self, word: String) {
        let mut buff = word.chars().collect::<Vec<char>>();

        let i = self.nodes.len();
        self.indicates.insert(word, i);
        self.nodes.push(Node {
            start_accessed: false,
            end_accessed: false,
            next: Vec::new(),
        });

        for j in 0..buff.len() {
            let c = buff[j];
            buff[j] = '*';
            let key = buff.iter().collect::<String>();
            buff[j] = c;

            if let Some(k) = self.indicates.get(&key) {
                self.nodes[i].next.push(*k);
                self.nodes[*k].next.push(i);
            };
        }
    }
}
// @lc code=end
