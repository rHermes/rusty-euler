// Euler problem 20
pub fn run() -> u64 {
    use num::bigint::BigUint;
    use num::One;

    let mut a = BigUint::one();

    for i in 1..101 as u16 {
        a = a * i;
    }

    a.to_radix_be(10).iter().map(|x| *x as u64).sum::<u64>()
}
