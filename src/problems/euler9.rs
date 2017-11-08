// Euler problem 9

/// So here we can either bruteforce, or we can use the fact that one
/// can generate pythagorean triples with formulas:
///
///
/// This is taken straight from the psudocode of the thread solution.
///
/// https://en.wikipedia.org/wiki/Pythagorean_triple

use num::integer::gcd;

pub fn run() -> u64 {
    let s2 = 1000 / 2;
    let mlimit = ((s2 as f64).sqrt().ceil() - 1.0) as u64;


    for m in 2..mlimit {
        if s2 % m == 0 {
            let mut sm = s2 / m;
            while sm % 2 == 0 {
                sm /= 2;
            }

            let mut k = m + 1 + (m % 2);

            while k < 2 * m && k <= sm {

                if sm % k == 0 && gcd(k, m) == 1 {
                    let d = s2 / (k * m);
                    let n = k - m;
                    let a = d * (m * m - n * n);
                    let b = 2 * d * m * n;
                    let c = d * (m * m + n * n);
                    return a * b * c;
                }

                k += 2;
            }
        }
    }
    0
}
