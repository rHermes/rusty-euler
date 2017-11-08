// Euler problem 10

pub fn run() -> u64 {
    let mut is_prime = vec![true; 2_000_000];
    is_prime[0] = false;
    is_prime[1] = false;


    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;


    // Remove Even.
    let mut i = 2;
    let mut unprime = i * i;
    while unprime < l {
        is_prime[unprime] = false;
        unprime += i;
    }

    i = 3;

    let mut ans = 2;
    while i < sqrt + 1 {
        if is_prime[i] {
            ans += i;
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += 2 * i;
            }
        }
        i += 2;
    }
    // Now we just finish.
    while i < l {
        if is_prime[i] {
            ans += i;
        }
        i += 2;
    }

    ans as u64
}
