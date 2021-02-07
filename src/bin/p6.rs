fn main() {
    let mut sum0: u64 = 0;
    let mut sum1: u64 = 0;
    for i in 1..=100 {
        sum0 += i * i;
        sum1 += i;
    }
    sum1 *= sum1;
    println!("sum of squares:  {}", sum0);
    println!("square of sum: {}", sum1);
    println!("{}", sum1 - sum0);
}
