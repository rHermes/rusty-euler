//#![feature(iterator_step_by)]

extern crate num;
extern crate time;
use time::PreciseTime;

mod problems;

fn eval_prob(f: fn() -> u64, n: u64, exp: u64) -> bool {
    let start = PreciseTime::now();
    let ans = f();
    let end = PreciseTime::now();
    let re = ans == exp;
    if re {
        println!("[PASS] [{}] Problem {}: {}", start.to(end), n, ans)
    } else {
        println!(
            "[FAIL] [{}] Problem {}: Expected {} got {}",
            start.to(end),
            n,
            exp,
            ans
        )
    }
    re
}

fn main() {
    let total_start = PreciseTime::now();
    eval_prob(problems::n1, 1, 233168);
    eval_prob(problems::n2, 2, 4613732);
    eval_prob(problems::n3, 3, 6857);
    eval_prob(problems::n4, 4, 906609);
    eval_prob(problems::n5, 5, 232792560);
    eval_prob(problems::n6, 6, 25164150);
    eval_prob(problems::n7, 7, 104743);
    eval_prob(problems::n8, 8, 23514624000);
    eval_prob(problems::n9, 9, 780000);
    let total_end = PreciseTime::now();

    println!("Total runtime: {}", total_start.to(total_end));
}
