// Created by seognil LC at 2023/07/10 01:32
// leetgo: 1.3.3
// https://leetcode.cn/problems/two-sum/

import java.util.Map;
import java.util.HashMap;

// * [ '1 ms', '98%', '42.5 MB', '51%' ]

// @lc code=begin

class Solution {
    public int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> map = new HashMap<>();
        for (int i = 0; i < nums.length; i++) {
            int complement = target - nums[i];
            if (map.containsKey(complement)) {
                return new int[] { map.get(complement), i };
            }
            map.put(nums[i], i);
        }
        return new int[0];
    }
}

// @lc code=end
