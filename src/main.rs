// Copyright © 2019 Nicholas Hanemann
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///This file will contain little code itself.
///It will largely be the launching point for all functionality.

mod keygen;
mod encrypt;
mod decrypt;

fn main() {
    println!("Despite being main, this file currently does nothing!\n(at least of value)");
	
	keygen::new_keys();
	encrypt::encrypt();
	decrypt::decrypt()
}
