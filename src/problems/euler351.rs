// Euler problem 351
//
// Answer: 11762187201804552
//
//
// I calculated it with matematica first.
//
// Then when I got access to the euler paper, I started improving my solution
// based on that. The solution currently in use is just a 1-to-1 translation
// of the final psudocode.
//
fn fast_totient_sum_2(n: u64) -> u64 {
    let l = (((n as f64) / (n as f64).ln().ln()).powf(2.0 / 3.0).floor()) as u64;

    let mut big_v: Vec<u64> = vec![0; ((n / l) + 1) as usize];

    // Sieve.
    let mut sieve: Vec<u64> = vec![0; (l + 1) as usize];

    // TODO(rHermes): If I can find a way to do this faster or to ingretrate it.
    // into the second loop it would be big.
    let mut i: u64 = 0;
    while i < (l + 1) as u64 {
        sieve[i as usize] = i;
        i += 1;
    }


    let mut p = 2;
    while p < (l + 1) as u64 {
        if p == sieve[p as usize] {
            let mut k = p;

            while k <= l {
                sieve[k as usize] = sieve[k as usize] - sieve[k as usize] / p;
                k = k + p;
            }
        }

        sieve[p as usize] = sieve[p as usize] + sieve[(p - 1) as usize];
        p += 1;
    }

    let mut x = n / l;

    while x > 0 {
        let k = n / x;
        let kis = (k as f64).sqrt() as u64;
        let mut res = (k * (k + 1)) / 2;

        for g in 2..kis + 1 {
            res -= if k / g <= l {
                sieve[(k / g) as usize]
            } else {
                big_v[(x * g) as usize]
            }
        }

        for z in 1..kis + 1 {
            if (k / z) != z {
                res = res - (k / z - k / (z + 1)) * sieve[z as usize];
            }
        }

        big_v[x as usize] = res;

        x -= 1;
    }


    big_v[1]
}

fn run_fast_2_itr() -> u64 {
    const N: u64 = 100_000_000;
    let sum = fast_totient_sum_2(N);

    3 * N * (N + 1) - 6 * sum
}

pub fn run() -> u64 {
    run_fast_2_itr()
}
