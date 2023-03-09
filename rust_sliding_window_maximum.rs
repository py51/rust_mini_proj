struct Solution {}
use std::collections::VecDeque;

fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let result = Solution::max_sliding_window(nums, k);
    println!("{:?}", result);
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut queue = VecDeque::with_capacity(k as usize);
        for (i, &v) in nums.iter().enumerate() {
            if i - queue.front().unwrap_or(&0) == k as usize {
                queue.pop_front();
            }
            while let Some(&index) = queue.back() {
                if nums[index] >= v {
                    break;
                }
                queue.pop_back();
            }
            queue.push_back(i);
            if i >= k as usize - 1 {
                res.push(nums[queue[0]]);
            }
        }
        res
    }
}