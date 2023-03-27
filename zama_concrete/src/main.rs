use std::time::{Instant};
use std::fs::File;
use std::io::{Write, Read};
use tfhe::shortint::prelude::*;
use bincode;
use hex::ToHex;
use base64::{engine::general_purpose, Engine as _};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    // //GENERATE KEYS
    let start = Instant::now();
    // let config = ConfigBuilder::all_disabled().enable_default_uint8().build();

    // let (client_key, server_key) = generate_keys(config);

    // let key_gen_time = start.elapsed();
    // println!("Keys generated: {:?}", key_gen_time);

    //FETCH KEYS
    // We generate a set of client/server keys, using the default parameters:
    //let (client_key, server_key) = gen_keys();

// We serialize the keys to bytes:
    //let encoded_server_key: Vec<u8> = bincode::serialize(&server_key).unwrap();
    //let encoded_client_key: Vec<u8> = bincode::serialize(&client_key).unwrap();
    //let encoded_server_key = serde_json::to_string(&server_key).unwrap();
    //let encoded_client_key = serde_json::to_string(&client_key).unwrap();

    //println!("server key: {:?}", encoded_server_key.as_slice());
    //println!("server key as bytes: {:?}", encoded_server_key.as_bytes());

    let server_key_file = "/home/kunho/Practicum/keys/ser_server_key.bin";
    let client_key_file = "/home/kunho/Practicum/keys/ser_client_key.bin";

// We write the keys to files:
    //let mut file = File::create(server_key_file)
        //.expect("failed to create server key file");
    //file.write_all(encoded_server_key.as_slice()).expect("failed to write key to file");
    //file.write(encoded_server_key.as_bytes()).expect("failed to write key to file");
    //let mut file = File::create(client_key_file)
        //.expect("failed to create client key file");
    //file.write_all(encoded_client_key.as_slice()).expect("failed to write key to file");
    //file.write(encoded_client_key.as_bytes()).expect("failed to write key to file");

    let mut file = File::open(server_key_file)
        .expect("failed to open server key file");
    let mut encoded_server_key: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_server_key).expect("failed to read the key");

    let mut file = File::open(client_key_file)
        .expect("failed to open client key file");
    let mut encoded_client_key: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_client_key).expect("failed to read the key");

    //We deserialize the keys:
    let loaded_server_key: ServerKey = bincode::deserialize(&encoded_server_key[..])
        .expect("failed to deserialize");
    let loaded_client_key: ClientKey = bincode::deserialize(&encoded_client_key[..])
        .expect("failed to deserialize");

    //SET KEYS
    //let start = Instant::now();
    //set_server_key(loaded_server_key);


    let key_set_time = start.elapsed();
    println!("Keys set: {:?}", key_set_time);

    let(client_key, server_key) = gen_keys(Parameters::default());

    //println!("Client key: {:?}", client_key);
    //println!("Server key: {:?}", server_key);

    let clear_a = 55;
    let clear_b = 15;

    let modulus = client_key.parameters.message_modulus.0;

    //ENCRYPTION
    let start = Instant::now();
    //let a = loaded_client_key.encrypt(clear_a);
    let a = client_key.encrypt_with_message_modulus(clear_a, MessageModulus(255));
    let b = client_key.encrypt(clear_b);
    let encryption = start.elapsed();
    println!("Encryption done: {:?}", encryption);

    let c = server_key.unchecked_add(&a, &b);

    let a_str: Vec<u8> = bincode::serialize(&a).unwrap();
    let a_hex_enc = a_str.encode_hex::<String>();
    let a_b64_enc = general_purpose::STANDARD.encode(&a_str);
    
    //println!("Ciphertext = {:?}", a_str);
    //println!("Ciphertext hex enc = {:?}", a_hex_enc);
    //println!("Ciphertext hex enc = {:?}", a_b64_enc);
    //print_type_of(&a_str);
    print_type_of(&a_hex_enc);
    print_type_of(&a_b64_enc);
    println!("Ciphertext hex length = {:?}", a_hex_enc.len());
    println!("Ciphertext b64 length = {:?}", a_b64_enc.len());

    //deserialize and decode
    let a_dec = general_purpose::STANDARD.decode(&a_b64_enc).unwrap();
    //print_type_of(&a_dec);
    let a_des: Vec<u8> = bincode::deserialize(&a_dec).unwrap();

    print_type_of(&a_str);
    print_type_of(&a_dec);
    assert_eq!(a_str, a_dec);
    //assert_eq!(1,2);

    //let a_decrypted = loaded_client_key.decrypt(&a_des);
    //let a_decrypted = loaded_client_key.decrypt(&a);
    let a_decrypted = client_key.decrypt(&a);
    println!("Plaintext should be = {:?}", clear_a % modulus as u64);
    println!("Plaintext = {:?}", a_decrypted);

    //OPERATION ON CIPHERTEXT
    // let start = Instant::now();
    // let result = a + b;

    // let enc_add = start.elapsed();
    // println!("Encryption addition done: {:?}", enc_add);
    // //println!("FHE result: {}", result);

    // //DECRYPTION
    // let start = Instant::now();
    // let decrypted_result: u8 = result.decrypt(&client_key);

    // let decryption = start.elapsed();
    // println!("Decryption done: {:?}", decryption);
    // println!("Decrypted result: {}", decrypted_result);

    //OPERATION ON CLEARTEXT
    //let clear_result = clear_a + clear_b;

    //assert_eq!(decrypted_result, clear_result);
}