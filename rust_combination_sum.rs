struct Solution {}

fn main() {
    let k = 3;
    let n = 7;
    let result = Solution::combination_sum3(k, n);
    println!("{:?}", result);
}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut path = vec![];
        Self::backtrace(&mut result, &mut path, n, k, 0, 1);
        result
    }
    pub fn backtrace(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        target_sum: i32,
        k: i32,
        sum: i32,
        start_index: i32,
    ) {
        if sum > target_sum {
            return;
        }
        let len = path.len() as i32;
        if len == k {
            if sum == target_sum {
                result.push(path.to_vec());
            }
            return;
        }
        for i in start_index..=9 - (k - len) + 1 {
            path.push(i);
            Self::backtrace(result, path, target_sum, k, sum + i, i + 1);
            path.pop();
        }
    }
}

