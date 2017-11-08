// Euler problem 15

pub fn run() -> u64 {
    let mut ans: f64 = 1.0;
    for i in 1..21 {
        ans *= (20 + i) as f64 / (i as f64);
    }

    ans.round() as u64
}
