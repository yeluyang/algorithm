[49] 字母异位词分组  

https://leetcode-cn.com/problems/group-anagrams/description/

Tags:   algorithms   amazon   bloomberg   facebook   uber   yelp   hash-table   string 

Langs:  c   cpp   csharp   elixir   erlang   golang   java   javascript   kotlin   php   python   python3   racket   ruby   rust   scala   swift   typescript 

* algorithms
* Medium (66.69%)
* Likes:    929
* Dislikes: -
* Total Accepted:    253.7K
* Total Submissions: 380.5K
* Testcase Example:  '["eat","tea","tan","ate","nat","bat"]'
* Source Code:       49.group-anagrams.rs

<p>给你一个字符串数组，请你将 <strong>字母异位词</strong> 组合在一起。可以按任意顺序返回结果列表。</p>

<p><strong>字母异位词</strong> 是由重新排列源单词的字母得到的一个新单词，所有源单词中的字母通常恰好只用一次。</p>

<p>&nbsp;</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> strs = <code>["eat", "tea", "tan", "ate", "nat", "bat"]</code>
<strong>输出: </strong>[["bat"],["nat","tan"],["ate","eat","tea"]]</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> strs = <code>[""]</code>
<strong>输出: </strong>[[""]]
</pre>

<p><strong>示例 3:</strong></p>

<pre>
<strong>输入:</strong> strs = <code>["a"]</code>
<strong>输出: </strong>[["a"]]</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= strs.length &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= strs[i].length &lt;= 100</code></li>
	<li><code>strs[i]</code>&nbsp;仅包含小写字母</li>
</ul>

