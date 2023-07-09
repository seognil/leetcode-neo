// Created by seognil LC at 2023/07/09 21:48
// leetgo: 1.3.3
// https://leetcode.cn/problems/two-sum/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// * [ '4 ms', '95%', '4.1 MB', '27%' ]

// @lc code=begin

func twoSum(nums []int, target int) (ans []int) {

	mapper := make(map[int]int)

	for i, num := range nums {
		complement := target - num
		if index, ok := mapper[complement]; ok {
			return []int{index, i}
		}
		mapper[num] = i
	}

	return nil

}

// @lc code=end

func main() {
	stdin := bufio.NewReader(os.Stdin)
	nums := Deserialize[[]int](ReadLine(stdin))
	target := Deserialize[int](ReadLine(stdin))
	ans := twoSum(nums, target)

	fmt.Println("\noutput:", Serialize(ans))
}
