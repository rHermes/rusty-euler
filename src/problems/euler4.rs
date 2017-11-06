// Euler problem 4

// Here we use the fact t

fn is_palin(n: u64) -> bool {
    let mut pn = 0;
    let mut nc = n;

    while nc > 0 {
        pn *= 10;
        pn += nc % 10;
        nc /= 10;
    }

    pn == n
}


pub fn run() -> u64 {
    let mut ans = 0;
    let mut a = 999;

    while a >= 100 {
        let mut db = 11;
        let mut b = 990;

        if a % 11 == 0 {
            db = 1;
            b = 999;
        }

        while b >= a {
            let c = a * b;

            if c <= ans {
                break;
            }

            if is_palin(c) {
                ans = c;
            }
            b = b - db
        }
        a -= 1;
    }


    ans
}
