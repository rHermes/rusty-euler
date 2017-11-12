// Euler problem 16


pub fn run() -> u64 {
    use num::bigint::BigUint;
    use num::One;

    let mut b = BigUint::one() + BigUint::one();

    for _ in 0..999 {
        b = b * (2 as u16);
    }

    b.to_radix_be(10).iter().map(|x| *x as u64).sum::<u64>()
}
