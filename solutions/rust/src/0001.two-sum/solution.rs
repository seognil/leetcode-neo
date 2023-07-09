// Created by seognil LC at 2023/07/09 21:48
// leetgo: 1.3.3
// https://leetcode.cn/problems/two-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// * [ '0 ms', '100%', '2.6 MB', '6%' ]

// @lc code=begin

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::two_sum(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
