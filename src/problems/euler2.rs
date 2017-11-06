// Euler problem 2
pub fn run() -> u64 {
    let mut x1 = 1;
    let mut x2 = 2;
    let mut ans = 0;

    while x2 <= 4000000 {
        if x2 % 2 == 0 {
            ans += x2
        }

        let x3 = x2;
        x2 = x2 + x1;
        x1 = x3;
    }

    ans
}
