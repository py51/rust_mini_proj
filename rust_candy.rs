struct Solution {}

fn main() {
    let ratings = vec![1, 0, 2];
    let result = Solution::candy(ratings);
    println!("{:?}", result);
}
impl Solution {
pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1i32; ratings.len()];
    for i in 1..ratings.len() {
        if ratings[i - 1] < ratings[i] {
            candies[i] = candies[i - 1] + 1;
        }
    }

    for i in (0..ratings.len()-1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
    }
    candies.iter().sum()
}
}