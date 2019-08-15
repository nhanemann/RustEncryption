use crate::millerrabin as mrp;
use num::bigint::BigInt;
use rand::Rng;

pub fn r57bit() -> u64 {
    //2^56 = 72057594037927936
    //2^57 = 144115188075855872
    let mut rng = rand::thread_rng();
    let x: u64 = rng.gen_range(72057594037927936u64, 144115188075855872u64);
    x
}

pub fn big_prime() -> u64 {
    let mut done = false;
    let mut val: u64 = 0;
    while !done {
        val = r57bit();
        done = mrp::is_prime(&val);
    }
    val
}

pub fn less_than(n: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let x: u64 = rng.gen_range(0, n);
    x
}

pub fn keys() -> (u64, u64, BigInt) {
    let p = big_prime();
    let l = less_than(p);
    let pm = mrp::modular_exponentiation(&2, &l, &p);
    (p, l, pm)
}
