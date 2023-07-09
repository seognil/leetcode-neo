// Created by seognil LC at 2023/07/10 00:37
// leetgo: 1.3.3
// https://leetcode.cn/problems/two-sum/

// * [ '36 ms', '95%', '14.2 MB', '60%' ]

// @lc code=begin

class Solution {
  func twoSum(_ nums: [Int], _ target: Int) -> [Int] {
    var map = [Int: Int]()
    for (i, num) in nums.enumerated() {
      let complement = target - num
      if let j = map[complement] {
        return [j, i]
      }
      map[num] = i
    }
    return []
  }
}

// @lc code=end
