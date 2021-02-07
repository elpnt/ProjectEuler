use num::integer::lcm;

fn main() {
    let mut ans = 1u64;
    for i in 1..=20 {
        ans = lcm(ans, i);
    }
    println!("{}", ans);
}
