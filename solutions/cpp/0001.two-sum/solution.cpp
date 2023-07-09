// Created by seognil LC at 2023/07/10 01:32
// leetgo: 1.3.3
// https://leetcode.cn/problems/two-sum/

#include "LC_IO.h"
#include <bits/stdc++.h>
using namespace std;

// * [ '4 ms', '99%', '10.7 MB', '21%' ]

// @lc code=begin

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {
    unordered_map<int, int> map;
    for (int i = 0; i < nums.size(); i++) {
      int complement = target - nums[i];
      if (map.count(complement)) {
        return {map[complement], i};
      }
      map[nums[i]] = i;
    }
    return {};
  }
};

// @lc code=end

int main() {
  ios_base::sync_with_stdio(false);
  stringstream out_stream;

  vector<int> nums;
  LeetCodeIO::scan(cin, nums);
  int target;
  LeetCodeIO::scan(cin, target);

  Solution *obj = new Solution();
  auto res = obj->twoSum(nums, target);
  LeetCodeIO::print(out_stream, res);
  cout << "\noutput: " << out_stream.rdbuf() << endl;

  delete obj;
  return 0;
}
