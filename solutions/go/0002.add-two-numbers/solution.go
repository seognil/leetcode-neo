// Created by seognil LC at 2023/07/09 22:53
// leetgo: 1.3.3
// https://leetcode.cn/problems/add-two-numbers/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// * [ '8 ms', '81%', '4.3 MB', '89%' ]

// @lc code=begin

func addTwoNumbers(l1 *ListNode, l2 *ListNode) (ans *ListNode) {

	carry := 0
	dummyHead := &ListNode{}
	currentNode := dummyHead

	for l1 != nil || l2 != nil || carry != 0 {
		val1, val2 := 0, 0
		if l1 != nil {
			val1, l1 = l1.Val, l1.Next
		}
		if l2 != nil {
			val2, l2 = l2.Val, l2.Next
		}

		sum := val1 + val2 + carry
		carry, sum = sum/10, sum%10

		currentNode.Next = &ListNode{Val: sum}
		currentNode = currentNode.Next
	}

	return dummyHead.Next
}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	l1 := Deserialize[*ListNode](ReadLine(stdin))
	l2 := Deserialize[*ListNode](ReadLine(stdin))
	ans := addTwoNumbers(l1, l2)

	fmt.Println("\noutput:", Serialize(ans))
}
