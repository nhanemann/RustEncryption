// Copyright © 2019 Nicholas Hanemann
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//This file contains the logic and some necessary math for generating keys.
//It also contains the relevant tests.

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

//--------TESTS BELOW HERE------------------------------
#[test]
fn test_57_bit() {
	for _x in 0..10 {
		let t_val = r57bit();
		assert_eq!(true, t_val > 72057594037927936u64);
		assert_eq!(true, t_val < 144115188075855872u64);
	}
}

#[test]
fn test_big_prime() {
	for _x in 0..10 {
		assert_eq!(true, mrp::is_prime(&big_prime()));
	}
}

#[test]
fn test_less_than() {
	assert_eq!(true, less_than(1000) < 1000);
	assert_eq!(true, less_than(10) < 10);
	assert_eq!(true, less_than(144115188075855872u64) < 144115188075855872u64);
}

#[test]
fn test_keygen() {
	let (p, lt, pm) = keys();
	assert_eq!(true, lt < p);
	assert_eq!(pm, mrp::modular_exponentiation(&2, &lt, &p));
}