// Euler problem 6


// Sum of numbers up to n: f(n) = n*(n+1)/2
// Sum of squares up to n: g(n) = n*(n+1)*(2n+1)/6
//
// h(x) = f(n) ^2 - g(n) = 1/12 * n * (n+1) * (3*n + 2) * (n-1)
pub fn run() -> u64 {
    const N: u64 = 100;
    (N * (N + 1) * (3 * N + 2) * (N - 1)) / 12
}
