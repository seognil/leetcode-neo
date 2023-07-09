// Created by seognil LC at 2023/07/10 00:37
// leetgo: 1.3.3
// https://leetcode.cn/problems/two-sum/

// * [ '192 ms', '94%', '37.2 MB', '57%' ]

// @lc code=begin

class Solution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        val map = HashMap<Int, Int>()
        for (i in nums.indices) {
            val complement = target - nums[i]
            if (map.containsKey(complement)) {
                return intArrayOf(map[complement]!!, i)
            }
            map[nums[i]] = i
        }
        throw IllegalArgumentException("No two sum solution")
    }
}

// @lc code=end
