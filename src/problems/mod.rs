mod euler1;
mod euler2;
mod euler3;
mod euler4;
mod euler5;
mod euler6;
mod euler7;
mod euler8;
mod euler9;
mod euler10;
mod euler11;
mod euler12;


mod euler14;
mod euler15;
mod euler16;

mod euler18;
mod euler19;
mod euler20;


mod euler67;


mod euler351;


pub fn n1() -> u64 {
    euler1::run()
}

pub fn n2() -> u64 {
    euler2::run()
}

pub fn n3() -> u64 {
    euler3::run()
}

pub fn n4() -> u64 {
    euler4::run()
}

pub fn n5() -> u64 {
    euler5::run()
}

pub fn n6() -> u64 {
    euler6::run()
}

// TODO(rHermes): Easy optimize, just better prime.
pub fn n7() -> u64 {
    euler7::run()
}

pub fn n8() -> u64 {
    euler8::run()
}

pub fn n9() -> u64 {
    euler9::run()
}


// TODO(rHermes): Maybe optimize, better prime.
pub fn n10() -> u64 {
    euler10::run()
}


pub fn n11() -> u64 {
    euler11::run()
}

// TODO(rHermes): Easy optimize, prime factor or sieve.
pub fn n12() -> u64 {
    euler12::run()
}

// Easy pickings.
pub fn n14() -> u64 {
    euler14::run()
}

pub fn n15() -> u64 {
    euler15::run()
}

// easy optimize, I don't know how to use the big num library yet.
pub fn n16() -> u64 {
    euler16::run()
}

pub fn n18() -> u64 {
    euler18::run()
}

// TODO(rHermes): Easy optimization, ger rid of chrono library.
pub fn n19() -> u64 {
    euler19::run()
}

// TODO(rHermes): Possible optimization, get rid of big int library.
pub fn n20() -> u64 {
    euler20::run()
}

pub fn n67() -> u64 {
    euler67::run()
}

pub fn n351() -> u64 {
    euler351::run()
}
