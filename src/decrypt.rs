// Copyright © 2019 Nicholas Hanemann
// [This program is licensed under the GNU General Public License 3.0]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//This file contains the logic for decryption.
//There are no tests because they would rely on every other function in the program -
//essentially, testing them would require running the whole program as a user

use crate::millerrabin as mrp;
use num::bigint::BigInt;

pub fn decrypt(ct: Vec<u64>, key: [u64; 3]) -> String {
    let mut plaintext = String::new();
    for x in 0..ct.len() / 2 {
        let (_a, chars) = b_decrypt(ct[2 * x], ct[2 * x + 1], key).to_bytes_be();
        let chars = match std::str::from_utf8(&chars) {
            Err(_) => return "~~~debug~~~".to_string(),
            Ok(n) => n,
        };
        plaintext.push_str(&chars);
    }
    plaintext
}

pub fn b_decrypt(first: u64, second: u64, key: [u64; 3]) -> BigInt {
    let (prime, lt) = (key[0], key[1]);

    let left = mrp::modular_exponentiation(&first, &(prime - 1 - lt), &prime);
    let right = mrp::modular_exponentiation(&second, &1, &prime);
    mrp::modular_exponentiation(&(left * right), &BigInt::from(1u32), &BigInt::from(prime))
}
