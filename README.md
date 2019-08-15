# RustEncryption
Nicholas Hanemann
CS410P: Rust Programming
Summer 2019, Portland State University

Public key encryption/decryption program in Rust

This project is using the GNU General Public License 3.0,
which can be found in the LICENSE.txt file in this repository, or online from the Free Software Foundation.

In this project, I have implemented a public key encryption algorithm, which was initially provided by Dr. Sarah Mocas in her Cryptography course at Portland State.

The encryption is currently implmented in 7 character blocks (ie: 56bit).

The functionality that the it has includes the following:
  - Generate a private key consisting of 3 numbers
  - Generate a corresponding public key also consisting of 3 numbers
  - Given a key and a plaintext, encrypt it into ciphertext
  - Given the proper key and a ciphertext, decrypt it into plaintext
  
These actions will be done via command line, using the following arguments:
	keygen
	encrypt [file_to_encrypt]
	decrypt
Key files, encrypted files, and decrypted files are placed in the active directory.

The first user would call on the keygen to generate a priKey.txt and pubKey.txt file.
Then, they would send the pubKey.txt to other users.
Other users can use the public key to encrypt messages that only the original user can decrypt.
