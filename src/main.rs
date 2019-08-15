// Copyright © 2019 Nicholas Hanemann
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///This file is the main launching point for work, and also includes file IO functions

mod keygen;
mod encrypt;
mod decrypt;
mod millerrabin;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use num::bigint::BigInt;
use std::env;

const USER_GUIDE: &str = "Use one of the following as an argument:\nkeygen\nencrypt [file to encrypt]\ndecrypt";

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 && args.len() != 3 {
		println!("Wrong number of arguments provided.");
		println!("{}", USER_GUIDE);
		return;
	}
	
	match args[1].as_str() {
		"keygen" => keyBuild(),
		"encrypt" => if args.len() == 3 {
				cipherWrite(args[2].as_str())
			} else {
				println!("Need text file to encrypt.");
				println!("{}", USER_GUIDE);
			},
		"decrypt" => plainWrite(),
		_ => println!("{}", USER_GUIDE),
	}
}

fn keyBuild() {
	keyWrite();
	while badKey() {
		keyWrite();
	}
	println!("Public key written.");
	println!("Private key written.");
}

fn keyWrite() {
	let mut key = keygen::keys();
	let pubPath = Path::new("pubKey.txt");
	let priPath = Path::new("priKey.txt");
	
	let mut file = match File::create(&pubPath) {
		Err(_) => panic!("Couldn't create public key file"),
		Ok(file) => file,
	};
	
	match file.write_fmt(format_args!("{} {} {}", key.0, key.2, 2)) {
		Err(_) => panic!("Couldn't write public key file"),
		Ok(_) => (),
	}
	
	file = match File::create(&priPath) {
		Err(_) => panic!("Couldn't create private key file"),
		Ok(file) => file,
	};
	
	match file.write_fmt(format_args!("{} {} {}", key.0, key.1, 2)) {
		Err(_) => panic!("Couldn't write private key file"),
		Ok(_) => (),
	}
	//pub = prime + gen + mod
	//pri = prime + gen + l
}

fn keyRead(p: &str) -> [u64; 3] {
	let path = Path::new(p);
	let mut file = match File::open(&path) {
		Err(_) => panic!("Couldn't access {}", p),
		Ok(file) => file,
	};
	
	let mut keyString = String::new();
	file.read_to_string(&mut keyString);
	
	let ret = textIntCompile(keyString);
	if ret.len() != 3 {
		panic!("{} has too many values.", p);
	}
	[ret[0], ret[1], ret[2]]
}

fn textIntCompile(k: String) -> Vec<u64>
{
	let mut split = k.trim().split(" ").collect::<Vec<&str>>();
	let mut ret = Vec::new();
	for s in split {
		let mut i: u64 = 0;
		match s.parse::<u64>() {
			Ok(n) => i=n,
			Err(e) => panic!("Source file improperly formatted."),
		}
		if i != 0 {
			ret.push(i)
		}
	}
	ret
} 

fn plainWrite() {
	let r = decrypt::decrypt(cipherRead("ciphertext.txt"), keyRead("priKey.txt"));
	println!("Decrypted plaintext:\n{}", r);
	
	let path = Path::new("plaintext.txt");
	
	let mut file = match File::create(&path) {
		Err(_) => panic!("Couldn't create plaintext file"),
		Ok(file) => file,
	};
	
	match file.write_all(r.as_bytes()) {
		Err(_) => panic!("Couldn't write plaintext file"),
		Ok(_) => println!("Plaintext written."),
	}
}

fn plainRead(p: &str) -> String {
	let path = Path::new(p);
	let mut file = match File::open(&path) {
		Err(_) => panic!("Couldn't access {}", p),
		Ok(file) => file,
	};
	let mut plaintext = String::new();
	file.read_to_string(&mut plaintext);
	plaintext
}

fn cipherWrite(orig: &str) {
	let r = encrypt::encrypt(plainRead(orig), keyRead("pubKey.txt"));
	let path = Path::new("ciphertext.txt");
	let mut ciphertext = String::new();

	for x in r {
		ciphertext.push_str(&(x.to_string() + " "));
	}
	
	let mut file = match File::create(&path) {
		Err(_) => panic!("Couldn't create ciphertext file"),
		Ok(file) => file,
	};
	
	match file.write_all(ciphertext.as_bytes()) {
		Err(_) => panic!("Couldn't write ciphertext file"),
		Ok(_) => println!("Ciphertext written."),
	}
	
}

fn cipherRead(p: &str) -> Vec<u64> {
	let path = Path::new(p);
	let mut file = match File::open(&path) {
		Err(_) => panic!("Couldn't access {}", p),
		Ok(file) => file,
	};
	
	let mut ciphertext = String::new();
	file.read_to_string(&mut ciphertext);
	
	let ret = textIntCompile(ciphertext);
	if ret.len() % 2 != 0 {
		panic!("{} has the wrong number of values.", p);
	}
	ret
}

fn badKey() -> bool
{
	let r = encrypt::encrypt("This is a testing string to fix mrb".to_string(), keyRead("pubKey.txt"));
	let mut ciphertext = String::new();
	for x in r {
		ciphertext.push_str(&(x.to_string() + " "));
	}
	let ct = textIntCompile(ciphertext);
	let y = decrypt::decrypt(ct, keyRead("priKey.txt"));
	y == "~~~debug~~~".to_string()
}