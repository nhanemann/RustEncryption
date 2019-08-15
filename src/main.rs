// Copyright © 2019 Nicholas Hanemann
// [This program is licensed under the GNU General Public License 3.0]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///This file is the main launching point for work, and also includes file IO functions
mod decrypt;
mod encrypt;

mod keygen;
mod millerrabin;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const USER_GUIDE: &str =
    "Use one of the following as an argument:\nkeygen\nencrypt [file to encrypt]\ndecrypt";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        println!("Wrong number of arguments provided.");
        println!("{}", USER_GUIDE);
        return;
    }

    match args[1].as_str() {
        "keygen" => key_build(),
        "encrypt" => {
            if args.len() == 3 {
                cipher_write(args[2].as_str())
            } else {
                println!("Need text file to encrypt.");
                println!("{}", USER_GUIDE);
            }
        }
        "decrypt" => plain_write(),
        _ => println!("{}", USER_GUIDE),
    }
}

fn key_build() {
    key_write();
    while bad_key() {
        key_write();
    }
    println!("Public key written.");
    println!("Private key written.");
}

fn key_write() {
    let key = keygen::keys();
    let pub_path = Path::new("pubKey.txt");
    let pri_path = Path::new("priKey.txt");

    let mut file = match File::create(&pub_path) {
        Err(_) => panic!("Couldn't create public key file"),
        Ok(file) => file,
    };

    if file
        .write_fmt(format_args!("{} {} {}", key.0, key.2, 2))
        .is_err()
    {
        panic!("Couldn't write public key file")
    }

    file = match File::create(&pri_path) {
        Err(_) => panic!("Couldn't create private key file"),
        Ok(file) => file,
    };

    if file
        .write_fmt(format_args!("{} {} {}", key.0, key.1, 2))
        .is_err()
    {
        panic!("Couldn't write private key file")
    }
}

fn key_read(p: &str) -> [u64; 3] {
    let path = Path::new(p);
    let mut file = match File::open(&path) {
        Err(_) => panic!("Couldn't access {}", p),
        Ok(file) => file,
    };

    let mut key_string = String::new();
    if file.read_to_string(&mut key_string).is_err() {}

    let ret = text_int_compile(key_string);
    if ret.len() != 3 {
        panic!("{} has too many values.", p);
    }
    [ret[0], ret[1], ret[2]]
}

fn text_int_compile(k: String) -> Vec<u64> {
    let split = k.trim().split(' ').collect::<Vec<&str>>();
    let mut ret = Vec::new();
    for s in split {
        let i: u64;
        match s.parse::<u64>() {
            Ok(n) => i = n,
            Err(_e) => panic!("Source file improperly formatted."),
        }
        if i != 0 {
            ret.push(i)
        }
    }
    ret
}

fn plain_write() {
    let r = decrypt::decrypt(cipher_read("ciphertext.txt"), key_read("priKey.txt"));
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

fn plain_read(p: &str) -> String {
    let path = Path::new(p);
    let mut file = match File::open(&path) {
        Err(_) => panic!("Couldn't access {}", p),
        Ok(file) => file,
    };
    let mut plaintext = String::new();
    if file.read_to_string(&mut plaintext).is_err() {}
    plaintext
}

fn cipher_write(orig: &str) {
    let r = encrypt::encrypt(plain_read(orig), key_read("pubKey.txt"));
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

fn cipher_read(p: &str) -> Vec<u64> {
    let path = Path::new(p);
    let mut file = match File::open(&path) {
        Err(_) => panic!("Couldn't access {}", p),
        Ok(file) => file,
    };

    let mut ciphertext = String::new();
    if file.read_to_string(&mut ciphertext).is_err() {}

    let ret = text_int_compile(ciphertext);
    if ret.len() % 2 != 0 {
        panic!("{} has the wrong number of values.", p);
    }
    ret
}

fn bad_key() -> bool {
    let r = encrypt::encrypt(
        "This is a testing string to fix mrb".to_string(),
        key_read("pubKey.txt"),
    );
    let mut ciphertext = String::new();
    for x in r {
        ciphertext.push_str(&(x.to_string() + " "));
    }
    let ct = text_int_compile(ciphertext);
    let y = decrypt::decrypt(ct, key_read("priKey.txt"));
    y == "~~~debug~~~"
}
