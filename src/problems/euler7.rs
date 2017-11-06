// Euler Problem 7


/// So if Pi(x) > x/ln(x) for x >= 17, where Pi(x) is the number of primes below or equal
/// to x, then we can find the upper bound on our prime by doing:
///
/// WE can find the upper bound by: n*(ln(n)*ln(ln(n)))
///
/// now that we have the upper bound, we can use sieve of eratosthenes.
/// This is not pretty and it is not fast, but we do what we can.
pub fn run() -> u64 {
    const N: u64 = 10001;
    const NF: f64 = N as f64;
    let lim: u64 = (NF * (NF.ln() + NF.ln().ln())).ceil() as u64;


    let mut mrk = vec![false; (lim + 1) as usize];

    let mut num_ans = 1;
    let mut i: u64 = 3;
    while i < lim {
        if mrk[i as usize] {
            i += 2;
            continue;
        }
        num_ans += 1;

        if num_ans == N {
            break;
        }

        for j in 2..lim {
            if (i * j) >= lim {
                break;
            }

            mrk[(i * j) as usize] = true;
        }

        i += 2;
    }

    i as u64
}
