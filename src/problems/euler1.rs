// Euler problem 1

fn sum_un(bar: u64, base: u64) -> u64 {
    let mut agg = 0;
    let mut n = 0;
    loop {
        n += base;
        if n >= bar {
            break agg;
        }

        agg += n;
    }
}

pub fn run() -> u64 {
    const LIM: u64 = 1000;
    sum_un(LIM, 3) + sum_un(LIM, 5) - sum_un(LIM, 15)
}
