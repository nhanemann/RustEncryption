use crate::millerrabin as mrp;
use num::bigint::BigInt;
use num::bigint::BigUint;
use num::bigint::ToBigInt;

pub fn decrypt(ct: Vec<u64>, key: [u64; 3]) -> String
{
	let mut plaintext = String::new();
	for x in 0..ct.len()/2 {
		let (a, chars) = bDecrypt(ct[2*x], ct[2*x + 1], key).to_bytes_be();
		let chars = match std::str::from_utf8(&chars) {
			Err(_) => return "~~~debug~~~".to_string(),
			Ok(n) => n,
		};
		plaintext.push_str(&chars);
	}
	plaintext
}

pub fn bDecrypt(first: u64, second: u64, key: [u64; 3]) -> BigInt
{
	let (prime, lt, gen) = (key[0], key[1] , key[2]);
	
	let left = mrp::modular_exponentiation(&first, &(prime - 1 - lt), &prime);
	let right = mrp::modular_exponentiation(&second, &1, &prime);
	mrp::modular_exponentiation(&(left * right), &BigInt::from(1u32), &BigInt::from(prime))
}