use std::cmp::max;

// 10000 ~ 999999
fn is_palindrome(n: u64) -> bool {
    let s: Vec<char> = n.to_string().chars().collect();
    let l = s.len();
    s[0] == s[l - 1] && s[1] == s[l - 2] && s[2] == s[l - 3]
}

fn main() {
    let mut ans = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            if is_palindrome(i * j) {
                ans = max(ans, i * j);
                println!("{} x {} = {}", i, j, i * j);
            }
        }
    }
    println!("{}", ans);
}
