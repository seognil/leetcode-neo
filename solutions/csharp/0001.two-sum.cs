// Created by seognil LC at 2023/07/10 00:37
// leetgo: 1.3.3
// https://leetcode.cn/problems/two-sum/

// * [ '128 ms', '97%', '43.7 MB', '41%' ]

// @lc code=begin

public class Solution {
    public int[] TwoSum(int[] nums, int target) {
        Dictionary<int, int> map = new Dictionary<int, int>();
        for (int i = 0; i < nums.Length; i++) {
            int complement = target - nums[i];
            if (map.ContainsKey(complement)) {
                return new int[] { map[complement], i };
            }
            map[nums[i]] = i;
        }
        throw new ArgumentException("No two sum solution");
    }
}

// @lc code=end
