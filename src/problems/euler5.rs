// Euler problem 5
//
//
use num::integer::lcm;

pub fn run() -> u64 {
    let mut ans = 1;
    for i in 2..21 {
        ans = lcm(ans, i);
    }
    ans
}
