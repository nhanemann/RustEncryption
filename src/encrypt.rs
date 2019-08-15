// Copyright © 2019 Nicholas Hanemann
// [This program is licensed under the GNU General Public License 3.0]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//This file contains the logic for encryption.
//It also contains a test. The functions without tests are not tested
//because they rely on every other function in the program -
//essentially, testing them would require running the whole program as a user

use crate::keygen as kg;
use crate::millerrabin as mrp;
use num::bigint::BigInt;

pub fn encrypt(s: String, k: [u64; 3]) -> Vec<BigInt> {
    let bs = blocks(s);
    let mut ciphertext = Vec::new();
    for b in bs {
        ciphertext.extend(b_encrypt(b, k));
    }
    ciphertext
}

pub fn blocks(s: String) -> Vec<u64> {
    //This converts essentially converts string->7 length hex->decimals
    //It can be manually checked with an ascii table and a hex->dec converter
    let bytes = s.into_bytes();

    let mut arr: [u8; 8] = [0; 8];
    let mut ret: Vec<u64> = Vec::new();
    for x in 0..bytes.len() {
        arr[(x % 7) + 1] = bytes[x];
        if x % 7 == 6 || x == bytes.len() - 1 {
            ret.push(u64::from_be_bytes(arr));
            arr = [0; 8];
        }
    }
    ret
}

//a b c = b^c % a
pub fn b_encrypt(b: u64, key: [u64; 3]) -> Vec<BigInt> {
    let new_num = kg::less_than(key[0]);
    let c1 = mrp::modular_exponentiation(&key[2], &new_num, &key[0]);
    let c2 = mrp::modular_exponentiation(&key[1], &new_num, &key[0]) * BigInt::from(b);
    let c3 = mrp::modular_exponentiation(&c2, &BigInt::from(1u32), &BigInt::from(key[0]));

    vec![c1, c3]
}

//--------TESTS BELOW HERE------------------------------
#[test]
fn test_blocks() {
    assert_eq!(vec![29_384_913_927_995_392u64], blocks("hello".to_string()));
    assert_eq!(
        vec![14_109_355_933_655_915u64, 9_126_382_154_511_464u64],
        blocks("2 block length".to_string())
    );
}
