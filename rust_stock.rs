struct Solution {}

fn main(){
    let prices = vec![7,1,5,3,6,4];
    let result = Solution::max_profit(prices);
    println!("{:?}", result);
}



impl Solution {
    fn max(a: i32, b: i32) -> i32 {
        if a > b { a } else { b }
    }
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 1..prices.len() {
            result += Self::max(prices[i] - prices[i - 1], 0);
        }
        result
    }
}
