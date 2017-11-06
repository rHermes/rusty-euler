// Euler problem 9
pub fn run() -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 2;
    let n = 300;

    while a < n {
        b = a + 1;
        while b < n {
            c = b + 1;

            if a + b + c > n {
                break;
            }

            while c < n {
                if (a + b + c == n) && (a * a + b * b == c * c) {
                    return a * b * c;
                }

                c += 1;
            }

            b += 1;
        }

        a += 1;
    }

    a * b * c
}
