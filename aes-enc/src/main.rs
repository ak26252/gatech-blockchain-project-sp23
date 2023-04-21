use std::time::{Instant};
use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use rand::RngCore;
use std::fs;
use std::fs::{File, read_to_string};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Read the contents of the input file into a byte vector
    let start = Instant::now();

    println!("AES ENCRYPTION");

    let mut input_file = File::open("/home/akim359/testfiles/csv/basex1000.csv").unwrap();
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let key_start = Instant::now();
    println!("Generating random 128 bit keys...");
    // Generate a random 128-bit key for AES encryption
    let key = GenericArray::from([0u8; 16]);
    let iv = GenericArray::from([0u8; 16]);
    let key_end = key_start.elapsed();
    println!("Keys generated! {:?}", key_end);

    let enc_start = Instant::now();
    println!("Encrypting csv...");
    // Create a new AES-128 cipher instance with the generated key
    let cipher = Aes128::new(&key);

    let mut encrypted_data = Vec::new();
    for chunk in input_data.chunks_exact(16){
        let block = GenericArray::clone_from_slice(chunk);
        let mut encrypted_block = block.clone();
        cipher.encrypt_block(&mut encrypted_block);
        encrypted_data.append(&mut encrypted_block.to_vec());
    }

    // Write the encrypted data and the key/IV to the output file
    let mut file = File::create("/home/akim359/testfiles/csv-enc/basex1000-aes.csv").unwrap();
    file.write_all(&key)?;
    file.write_all(&iv)?;
    file.write_all(&encrypted_data)?;

    let enc_end = enc_start.elapsed();
    println!("Encryption done! {:?}", enc_end);

    let finish = start.elapsed();
    println!("TOTAL TIME = {:?}", finish);

    Ok(())
}

