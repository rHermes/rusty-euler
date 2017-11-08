// Euler problem 14
//
//


fn collatz(n: u64, cache: &mut [u16; 500_000]) -> u16 {
    let nu = n as usize;
    if nu < 500_000 && cache[nu] > 0 {
        return cache[nu];
    }


    let res = 1 +
        if n % 2 == 0 {
            collatz(n / 2, cache)
        } else {
            1 + collatz((3 * n + 1) / 2, cache)
        };

    if nu < 500_000 {
        cache[nu] = res;
    }

    res
}

pub fn run() -> u64 {
    let mut cache: [u16; 500_000] = [0; 500_000];
    cache[0] = 0;
    cache[1] = 1;


    let mut ans = 1;
    let mut big = 0;

    for i in 1..1_000_000 {
        let kv = collatz(i, &mut cache);

        if kv > big {
            ans = i;
            big = kv;
        }
    }
    ans as u64
}
