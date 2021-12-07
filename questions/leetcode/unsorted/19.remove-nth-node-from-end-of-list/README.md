[19] 删除链表的倒数第 N 个结点  

https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/description/

Tags:   algorithms   linked-list   two-pointers 

Langs:  c   cpp   csharp   elixir   erlang   golang   java   javascript   kotlin   php   python   python3   racket   ruby   rust   scala   swift   typescript 

* algorithms
* Medium (43.18%)
* Likes:    1689
* Dislikes: -
* Total Accepted:    581.6K
* Total Submissions: 1.3M
* Testcase Example:  '[1,2,3,4,5]\n2'
* Source Code:       tmp/19.remove-nth-node-from-end-of-list.rs

<p>给你一个链表，删除链表的倒数第&nbsp;<code>n</code><em>&nbsp;</em>个结点，并且返回链表的头结点。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;" />
<pre>
<strong>输入：</strong>head = [1,2,3,4,5], n = 2
<strong>输出：</strong>[1,2,3,5]
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>head = [1], n = 1
<strong>输出：</strong>[]
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>head = [1,2], n = 1
<strong>输出：</strong>[1]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li>链表中结点的数目为 <code>sz</code></li>
	<li><code>1 &lt;= sz &lt;= 30</code></li>
	<li><code>0 &lt;= Node.val &lt;= 100</code></li>
	<li><code>1 &lt;= n &lt;= sz</code></li>
</ul>

<p>&nbsp;</p>

<p><strong>进阶：</strong>你能尝试使用一趟扫描实现吗？</p>

