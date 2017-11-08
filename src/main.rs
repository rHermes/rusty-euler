//#![feature(iterator_step_by)]
//
//
//Use this when I want to profile memory.
// #![feature(global_allocator, allocator_api, heap_api)]
//
// use std::heap::{Alloc, System, Layout, AllocErr};
// #[global_allocator]
// static GLOBAL: System = System;

extern crate num;
extern crate time;
use time::PreciseTime;

mod problems;
mod primes;

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
    eval_prob(problems::n9, 9, 31875000);
    eval_prob(problems::n10, 10, 142913828922);
    eval_prob(problems::n11, 11, 70600674);
    eval_prob(problems::n12, 12, 76576500);

    eval_prob(problems::n18, 18, 1074);

    eval_prob(problems::n67, 67, 7273);


    eval_prob(problems::n351, 351, 11762187201804552);
    let total_end = PreciseTime::now();

    println!("Total runtime: {}", total_start.to(total_end));
}
