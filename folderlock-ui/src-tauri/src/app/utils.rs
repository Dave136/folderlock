// use std::fs::{File, OpenOptions};
// use std::io::{Read, Write};
// use std::num::Wrapping;

// use bcrypt::{hash, verify, DEFAULT_COST};
// use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, SealingKey, UnboundKey, AES_256_GCM};
// use ring::error::Unspecified;

// pub fn hash_password(password: String) -> String {
//     hash(password, DEFAULT_COST).unwrap()
// }

// pub fn verify_password(password: String, hash: &str) -> bool {
//     verify(password, hash).unwrap()
// }

// struct NonceGen {
//     current: Wrapping<u128>,
//     start: u128,
// }

// impl NonceGen {
//     fn new(start: [u8; 12]) -> Self {
//         let mut array = [0; 16];
//         array[..12].copy_from_slice(&start);
//         let start = u128::from_le_bytes(array);
//         Self {
//             current: Wrapping(start),
//             start,
//         }
//     }
// }

// impl NonceSequence for NonceGen {
//     fn advance(&mut self) -> Result<Nonce, Unspecified> {
//         // ! Warning: doesn't check u128 overflow correctly
//         // Also, ring docs explicitly call this
//         // out as "reasonable (but probably not ideal)"
//         let n = self.current.0;
//         self.current += 1;
//         if self.current.0 == self.start {
//             return Err(Unspecified);
//         }
//         Ok(Nonce::assume_unique_for_key(
//             n.to_le_bytes()[..12].try_into().unwrap(),
//         ))
//     }
// }

// pub fn encrypt_file(key: &[u8], input: &str, output: &str, nonce: &[u8]) -> std::io::Result<()> {
//     let bytes: [u8; 16] = [
//         0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
//     ];
//     let nonce_seed = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];

//     // Read the input file
//     let mut input_file = File::open(input)?;
//     let mut input_data = Vec::new();
//     input_file.read_to_end(&mut input_data)?;

//     // // Set key and the nonce to cipher
//     // // let unbound_key = UnboundKey::new(&AES_256_GCM, key).unwrap();
//     // let nonce_key = Nonce::try_assume_unique_for_key(nonce).unwrap();

//     // Cipher the input data using AES-256 GCM
//     let mut cipher_text = vec![0u8; input_data.len() + AES_256_GCM.tag_len()];
//     let mut sealing_key = {
//         let unbound_key = UnboundKey::new(&AES_256_GCM, key).unwrap();
//         let nonce_gen = NonceGen::new(nonce_seed);
//         SealingKey::new(unbound_key, nonce_gen)
//     };

//     // println!("data readed: {:?}", input_data);

//     sealing_key
//         .seal_in_place_append_tag(Aad::empty(), &mut cipher_text)
//         .unwrap();

//     // Write the cipher text
//     let mut ouput_file = File::create("cipher.bin").unwrap();
//     // output_file.wri
//     ouput_file.write_all((&cipher_text)).unwrap();

//     Ok(())
// }

// fn decrypt_file(key: &[u8], input_path: &str, output_path: &str) -> std::io::Result<()> {
//     Ok(())
// }
