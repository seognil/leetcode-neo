# Created by seognil LC at 2023/07/09 22:53
# leetgo: 1.3.3
# https://leetcode.cn/problems/add-two-numbers/

from typing import *
from leetgo_py import *


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# * [ '56 ms', '86%', '15.9 MB', '68%' ]

# @lc code=begin


# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:
        carry = 0
        dummyHead = currentNode = ListNode()

        while l1 or l2 or carry:
            carry += (l1.val if l1 else 0) + (l2.val if l2 else 0)
            currentNode.next = ListNode(carry % 10)
            carry //= 10

            # * a bit slower
            # carry, sum = divmod(
            #     (l1.val if l1 else 0) + (l2.val if l2 else 0) + carry, 10
            # )
            # currentNode.next = ListNode(sum)

            currentNode = currentNode.next

            if l1:
                l1 = l1.next
            if l2:
                l2 = l2.next

        return dummyHead.next


# @lc code=end

if __name__ == "__main__":
    l1: ListNode = deserialize("ListNode", read_line())
    l2: ListNode = deserialize("ListNode", read_line())
    ans = Solution().addTwoNumbers(l1, l2)

    print("\noutput:", serialize(ans))
