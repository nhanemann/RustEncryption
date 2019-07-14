# RustEncryption
Nicholas Hanemann
CS410P: Rust Programming
Summer 2019, Portland State University

Public key encryption/decryption program in Rust

[Project is currently mostly incomplete]

In this project, I will implement a public key encryption algorithm, which was initially provided by Dr. Sarah Mocas in her Cryptography course at Portland State.

The current plan is to implement the encryption in 7 character blocks (ie: 56bit).

The functionality that the final product will have will include the following:
  - Generate a private key consisting of 3 numbers
  - Generate a corresponding public key also consisting of 3 numbers
  - Given a key and a plaintext, encrypt it into ciphertext
  - Given the proper key and a ciphertext, decrypt it into plaintext
  
When fully implemented, these actions will be done via command line.
The first user would call on the keygen to generate a prikey.txt and pubkey.txt file.
Then, they would send the pubkey.txt to other users.
Other users can use the public key to encrypt messages that only the original user can decrypt.
