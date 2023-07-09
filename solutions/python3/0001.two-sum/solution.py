# Created by seognil LC at 2023/07/09 22:12
# leetgo: 1.3.3
# https://leetcode.cn/problems/two-sum/

from typing import *
from leetgo_py import *

# * [ '40 ms', '87%', '17.3 MB', '27%' ]

# @lc code=begin


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        idx_map = {}
        for i, num in enumerate(nums):
            complement = target - num
            if complement in idx_map:
                return [idx_map[complement], i]
            idx_map[num] = i


# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    target: int = deserialize("int", read_line())
    ans = Solution().twoSum(nums, target)

    print("\noutput:", serialize(ans))
