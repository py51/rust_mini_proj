struct Solution {}

fn main() {
    let s:String = "Happy life !".to_string();
    let result = Solution::replace_space(s);
    println!("{:?}", result);
}


impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut len: usize = s.len();
        let mut s = s.chars().collect::<Vec<char>>();
        let mut count = 0;
        for i in &s {
            if i.is_ascii_whitespace() {
                count += 1;
            }
        }
        let mut new_len = len + count * 2;
        s.resize(new_len, ' ');
        while len < new_len {
            len -= 1;
            new_len -= 1;
            if s[len].is_ascii_whitespace() {
                s[new_len] = '0';
                s[new_len - 1] = '2';
                s[new_len - 2] = '%';
                new_len -= 2;
            }
            else { s[new_len] = s[len] }
        }
        s.iter().collect::<String>()
    }
}