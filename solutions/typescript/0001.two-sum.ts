// Created by seognil LC at 2023/07/09 21:48
// leetgo: 1.3.3
// https://leetcode.cn/problems/two-sum/

// * [ '64 ms', '93%', '44.2 MB', '64%' ]

// @lc code=begin

const twoSum = (nums: number[], target: number): number[] => {
  const idxMap: Record<number, number> = {};
  for (let i = 0; i < nums.length; i++) {
    const complement = target - nums[i];
    if (complement in idxMap) {
      return [idxMap[complement], i];
    }
    idxMap[nums[i]] = i;
  }
  return [];
};

// @lc code=end
