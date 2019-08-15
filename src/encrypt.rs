use crate::keygen as kg;
use crate::millerrabin as mrp;
use num::bigint::BigInt;
use num::bigint::BigUint;
use num::bigint::ToBigInt;

pub fn encrypt(s: String, k: [u64; 3]) -> Vec<BigInt>
{
	let bs = blocks(s);
	let mut ciphertext = Vec::new();
	for b in bs {
		ciphertext.extend(bEncrypt(b, k));
	}
	ciphertext
}

pub fn blocks(s: String) -> Vec<u64> {
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
pub fn bEncrypt(b: u64, key: [u64; 3]) -> Vec<BigInt>
{
	let newNum = kg::lessThan(key[0]);
	let c1 = mrp::modular_exponentiation(&key[2], &newNum, &key[0]);
	let c2 = mrp::modular_exponentiation(&key[1], &newNum, &key[0]) * BigInt::from(b);
	let c3 = mrp::modular_exponentiation(&c2, &BigInt::from(1u32), &BigInt::from(key[0]));
	
	vec![c1, c3]
}