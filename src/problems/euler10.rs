// Euler problem 10

pub fn run() -> u64 {
    const N: u64 = 2000000;

    let mut mrk = [false; N as usize];

    let mut ans: u64 = 2;
    let mut i: u64 = 3;
    while i < N {
        if mrk[i as usize] {
            i += 2;
            continue;
        }
        ans += i;

        for j in 2..N {
            if (i * j) >= N {
                break;
            }

            mrk[(i * j) as usize] = true;
        }

        i += 2;
    }

    ans
}
