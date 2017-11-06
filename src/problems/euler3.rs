// Euler problem 3

pub fn run() -> u64 {
    let mut ans: u64 = 600851475143;

    let mut i = 3;

    while i < ans / 2 {
        while ans % i == 0 {
            ans /= i;
        }

        i += 2;
    }

    ans
}
