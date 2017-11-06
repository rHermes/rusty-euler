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

mod euler18;


mod euler67;

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

// TODO(rHermes): Easy optimize.
pub fn n9() -> u64 {
    euler9::run()
}

// TODO(rHermes): Easy optimize, better prime thing. Lot of memory, because
// we allocate 2000000 bools, which is 1 byte. This requires 2GB then.
pub fn n10() -> u64 {
    euler10::run()
}

pub fn n11() -> u64 {
    euler11::run()
}

pub fn n18() -> u64 {
    euler18::run()
}

pub fn n67() -> u64 {
    euler67::run()
}
