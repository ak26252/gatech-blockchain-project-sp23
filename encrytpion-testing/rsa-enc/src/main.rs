use std::fs::{File, read_to_string};
use std::io::{Write, BufReader, BufWriter, BufRead, Read};
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use std::time::{Instant};

const CHUNK_SIZE: usize = 128;

fn main() {
    // Generate a new RSA keypair with a key size of 2048 bits
    println!("RSA ENCRYPTION\n");
    let start = Instant::now();

    println!("Key generation...");
    let keygen = Instant::now();

    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    
    let keygen_end = keygen.elapsed();
    println!("Keys generated! {:?}", keygen_end);
    
    // println!("Private key:\n{}", private_key_pem);
    // println!("Public key:\n{}", public_key_pem);

    let encrypt_start = Instant::now();
    println!("Encrypting...");

    let input_file = File::open("/home/akim359/testfiles/csv/basex1000.csv").unwrap();
    let input_data = BufReader::new(input_file).bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();

    // Encrypt the input data using the RSA public key
    let mut encrypted_data = Vec::new();
    for chunk in input_data.chunks(CHUNK_SIZE) {
        let encrypted_chunk = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, chunk).unwrap();
        //let encrypted_chunk = public_key.encrypt(chunk, PaddingScheme::PKCS1v15).unwrap();
        encrypted_data.extend(encrypted_chunk);
    }

    let mut output_file = File::create("/home/akim359/testfiles/csv-enc/basex1000-rsa.csv").unwrap();
    output_file.write_all(&encrypted_data).unwrap();

    let encrypt_end = encrypt_start.elapsed();
    println!("CSV file encrypted and saved! {:?}", encrypt_end);

    let time = start.elapsed();

    println!("RSA total encryption time = {:?}", time);
}
