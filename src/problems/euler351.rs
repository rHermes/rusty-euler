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
// I've commented out all the unceccecary parts, because it raises a lot of
// errors with rusts compilator. but it can be uncommented if you want to see
// the run times.
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

/*
use std::collections::HashMap;

fn euler_phi(y: u64) -> u64 {
    let mut x = y;
    let mut res = x;

    if x % 2 == 0 {
        res = res - res / 2;
        while x % 2 == 0 {
            x /= 2;
        }
    }

    let mut p = 3;

    while p * p <= x {
        if x % p == 0 {
            res = res - res / p;
            while x % p == 0 {
                x /= p;
            }
        }

        p += 2;
    }

    if x > 1 {
        res = res - res / x;
    }

    res
}


fn run_slow() -> u64 {
    let mut sum = 0;
    let n = 100_000_000;

    for x in 0..(n + 1) {
        sum += euler_phi(x);
    }

    3 * n * (n + 1) - 6 * sum
}

// Solves the problem in around 10 seconds.
fn run_medium() -> u64 {
    let mut sum: u64 = 0;
    let n: u32 = 100_000_000;

    let mut tots: Vec<u32> = vec![0; 100_000_001];

    let mut i: u32 = 0;
    while i < (n + 1) as u32 {
        tots[i as usize] = i;
        i += 1;
    }

    sum = 1;

    let mut p = 2;

    while p < (n + 1) as u32 {
        if p == tots[p as usize] {
            let mut k = p;

            while k <= n {
                tots[k as usize] = tots[k as usize] - tots[k as usize] / p;
                k = k + p;
            }
        }

        sum = sum + tots[p as usize] as u64;
        p += 1;
    }

    3 * (n as u64) * ((n + 1) as u64) - 6 * sum
}

fn fast_1_rec_v(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(vl) = cache.get(&n) {
        return *vl;
    }

    let mut res = (n * (n + 1)) / 2;

    let nis = (n as f64).sqrt() as u64;

    for g in 2..nis + 1 {
        res = res - fast_1_rec_v(n / g, cache);
    }

    for z in 1..nis + 1 {
        if (n / z) != z {
            res = res - (n / z - n / (z + 1)) * fast_1_rec_v(z, cache);
        }
    }


    cache.insert(n, res);
    res
}

// Runs in about 0.624285 seconds.
fn run_fast_1_rec() -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();
    let n = 100_000_000;
    let sum = fast_1_rec_v(n, &mut cache);

    3 * n * (n + 1) - 6 * sum
}


fn fast_2_rec_v(n: u64, L: u64, cache: &mut HashMap<u64, u64>, sieve: &[u64]) -> u64 {
    if n <= L {
        return sieve[n as usize];
    } else if let Some(vl) = cache.get(&n) {
        return *vl;
    }

    let mut res = (n * (n + 1)) / 2;

    let nis = (n as f64).sqrt() as u64;

    for g in 2..nis + 1 {
        res = res - fast_2_rec_v(n / g, L, cache, sieve);
    }

    for z in 1..nis + 1 {
        if (n / z) != z {
            res = res - (n / z - n / (z + 1)) * fast_2_rec_v(z, L, cache, sieve);
        }
    }


    cache.insert(n, res);
    res
}

// Runs in about 0.0646 seconds.
fn run_fast_2_rec() -> u64 {
    let n = 100_000_000;
    let mut cache: HashMap<u64, u64> = HashMap::new();
    let L = (((n as f64) / (n as f64).ln().ln()).powf(2.0 / 3.0).floor()) as u64;

    // Sieve.
    let mut sieve: Vec<u64> = vec![0; (L + 1) as usize];

    let mut i: u64 = 0;
    while i < (L + 1) as u64 {
        sieve[i as usize] = i;
        i += 1;
    }


    let mut p = 2;
    while p < (L + 1) as u64 {
        if p == sieve[p as usize] {
            let mut k = p;

            while k <= L {
                sieve[k as usize] = sieve[k as usize] - sieve[k as usize] / p;
                k = k + p;
            }
        }

        sieve[p as usize] = sieve[p as usize] + sieve[(p - 1) as usize];
        p += 1;
    }


    let sum = fast_2_rec_v(n, L, &mut cache, sieve.as_slice());

    3 * n * (n + 1) - 6 * sum
}
*/
