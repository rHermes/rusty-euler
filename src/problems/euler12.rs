// Euler problem 12


pub fn run() -> u64 {
    0
}
// This is a solution for sum of divisors.
/*
pub fn run() -> u64 {
    use std::collections::HashMap;

    let mut sigma = HashMap::new();

    sigma.insert(0, 0);
    sigma.insert(1, 1);
    sigma.insert(2, 2);
    sigma.insert(3, 2);

    let mut i = 4;

    loop {

        // We simply summize the previous ones.
        let mut ans = 0;

        let mut n: i64 = 1;
        let mut penta: i64 = 1;
        let mut cycle: u8 = 0;

        while (i > penta) {
            if cycle < 2 {
                ans += sigma.get(&(i - penta)).unwrap();
            } else {
                ans -= sigma.get(&(i - penta)).unwrap();
            }

            cycle = if cycle == 3 { 0 } else { cycle + 1 };
            n = if n < 0 { 1 - n } else { -n };
            penta = (3 * n * n - n) / 2;
        }

        sigma.insert(i, ans);
        println!("Calculating peta for {} is {}", i, ans);

        i += 1;

        if (i > 50) {
            break;
        }

    }

    i as u64
}
*/
