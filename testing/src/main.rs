use std::time::{Instant};
use std::fs::File;
use std::io::{Write, Read};
use std::str;
use std::string;
use std::error::Error;
use csv;
use serde::{Deserialize, Serialize};
use tfhe::shortint::prelude::*;
use concrete::{ConfigBuilder, generate_keys, set_server_key, FheUint8, ServerKey, ClientKey};
use std::io::Cursor;
use concrete::prelude::*;
use bincode;
use hex::ToHex;
use base64::{engine::general_purpose, Engine as _};
use compressed_string::ComprString;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

//struct for reading rows
#[derive(Debug, Deserialize)]
struct ReadRow {
    SpO2: u8,
    Pulse_Rate: u8,
    SYS: u8,
    DIA: u8,
    Heart_rate: u8,
    Resp_Rate: u8,
    ST: u8,
    Label: u8
}

//struct for writing rows
#[derive(Debug, Serialize)]
struct WriteRow {
    SpO2: String,
    Pulse_Rate: String,
    SYS: String,
    DIA: String,
    Heart_rate: String,
    Resp_Rate: String,
    ST: String,
    Label: String
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let total_start = Instant::now();
    let start = Instant::now();

    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();

    let ( client_key, server_key) = generate_keys(config);
    let (tfhe_ck, tfhe_sk) = gen_keys(Parameters::default());

    let encoded_server_key = bincode::serialize(&server_key).unwrap();
    let encoded_client_key = bincode::serialize(&client_key).unwrap();
    let tfhe_ck_enc = bincode::serialize(&tfhe_ck).unwrap();
    let tfhe_sk_enc = bincode::serialize(&tfhe_sk).unwrap();

    println!("Size of concrete serialized server key = {:?}", encoded_server_key.len());
    println!("Size of concrete serialized client key = {:?}", encoded_client_key.len());
    println!("Size of tfhe serialized server key = {:?}", tfhe_sk_enc.len());
    println!("Size of tfhe serialized client key = {:?}", tfhe_ck_enc.len());

    //set_server_key(server_key);

    print!("Type of enc server key = ");
    print_type_of(&encoded_server_key);
    print!("Type of enc client key = ");
    print_type_of(&encoded_client_key);

    let server_key_file = "/home/kunho/Practicum/keys/ser_server_key.bin";
    let client_key_file = "/home/kunho/Practicum/keys/ser_client_key.bin";

    // We write the keys to files:
    let mut file = File::create(server_key_file)
        .expect("failed to create server key file");
    file.write_all(encoded_server_key.as_slice()).expect("failed to write key to file");
    let mut file = File::create(client_key_file)
        .expect("failed to create client key file");
    file.write_all(encoded_client_key.as_slice()).expect("failed to write key to file");

    let writekeys = start.elapsed();

    println!("\nSUCCESSFULLY WROTE KEYS TO FILE ({:?})", writekeys);

    // We retrieve the keys:
    let start = Instant::now();
    let mut file = File::open(server_key_file)
        .expect("failed to open server key file");
    let mut encoded_server_key: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_server_key).expect("failed to read the key");

    let mut file = File::open(client_key_file)
        .expect("failed to open client key file");
    let mut encoded_client_key: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_client_key).expect("failed to read the key");

    let retrievekeys = start.elapsed();

    println!("SUCCESSFULLY RETRIEVED KEYS ({:?})", retrievekeys);

    // We deserialize the keys:
    let start = Instant::now();

    let loaded_server_key: ServerKey = bincode::deserialize(&encoded_server_key[..])
        .expect("failed to deserialize");
    let loaded_client_key: ClientKey = bincode::deserialize(&encoded_client_key[..])
        .expect("failed to deserialize");

    set_server_key(loaded_server_key);

    let serializekeys = start.elapsed();

    println!("Key length = {:?}", encoded_server_key.len());
    println!("SUCCESSFULLY DESERIALIZED KEYS ({:?})\n", serializekeys);

    // let start = Instant::now();
    
    // let msg1 = 133;
    // let msg2 = 12;

    // let value_1 = FheUint8::encrypt(msg1, &loaded_client_key);
    // let value_2 = FheUint8::encrypt(msg2, &loaded_client_key);

    // let value_3 = value_1+value_2;

    // let encrypttime = start.elapsed();

    // println!("SUCCESSFULLY ENCRYPTED VALUES ({:?})", encrypttime);

    // let start = Instant::now();

    // let value_3_str = bincode::serialize(&value_3).unwrap();
    // //let value_3_hex_enc = value_3_str.encode_hex::<String>();
    // let value_3_b64_enc = general_purpose::STANDARD.encode(&value_3_str);
    // //let value_3_b64_comp = ComprString::new(&value_3_b64_enc);
    // let s = String::from_utf8_lossy(&value_3_str);

    // let sertime = start.elapsed();
    // println!("SUCCESSFULLY SERIALIZED AND B64 ENCODED CIPHERTEXT ({:?})\n", sertime);
    
    // print!("Type of ciphertext = ");
    // print_type_of(&value_3_str);
    // print!("Type of ciphertext b64 enc = ");
    // print_type_of(&value_3_b64_enc);
    // print!("Type of ciphertext b64 enc = ");
    // print_type_of(&s);
    // // print!("Type of ciphertext b64 comp = ");
    // // print_type_of(&value_3_b64_comp);
    // println!("Ciphertext length = {:?}", value_3_str.len());
    // println!("Ciphertext b64 length = {:?}", value_3_b64_enc.len());
    // println!("Ciphertext length = {:?}\n", s.len());
    // //println!("Ciphertext b64 comp length = {:?}", value_3_b64_comp.compressed_len());

    // let compstring_file = "/home/kunho/Practicum/keys/compstring";

    // // We write the keys to files:
    // let mut file = File::create(compstring_file)
    //     .expect("failed to create comp string file");
    // file.write_all(value_3_b64_enc.as_bytes()).expect("failed to write key to file");

    // //deserialize and decode
    // let start = Instant::now();

    // let value_3_dec = general_purpose::STANDARD.decode(&value_3_b64_enc).unwrap();
    // let value_3_des: FheUint8 = bincode::deserialize(&value_3_dec).unwrap();

    // let destime = start.elapsed();
    // println!("SUCCESSFULLY DESERIALIZED AND DECODED CIPHERTEXT ({:?})", destime);

    // let start = Instant::now();

    // let value_3_pure_decrypted: u8 = value_3.decrypt(&loaded_client_key);
    // let value_3_b64_decrypted: u8 = value_3_des.decrypt(&loaded_client_key);
    // let real_decrypted = msg1+msg2;

    // let dectime = start.elapsed();
    // println!("SUCCESSFULLY DECRYPTED ({:?})\n", dectime);

    // println!("Pure decrypted = {:?}", value_3_pure_decrypted);
    // println!("Serialized/b64 decrypted = {:?}", value_3_b64_decrypted);
    // println!("Expected output (msg1+msg2) = {:?}", real_decrypted);

    // Prepare to send data to the server
    // The ClientKey is _not_ sent
    // let mut serialized_data = Vec::new();
    // bincode::serialize_into(&mut serialized_data, &server_key)?;
    // bincode::serialize_into(&mut serialized_data, &value_1)?;
    // bincode::serialize_into(&mut serialized_data, &value_2)?;

    // // Simulate sending serialized data to a server and getting
    // // back the serialized result
    // let serialized_result = server_function(&serialized_data)?;
    // let result: FheUint8 = bincode::deserialize(&serialized_result)?;

    // let output: u8 = result.decrypt(&client_key);
    // assert_eq!(output, msg1 + msg2);

    //csv stuff
    let readcsv = "/home/kunho/Practicum/iomt/convert/wustl-onlyint.csv";
    let writecsv = "/home/kunho/Practicum/iomt/convert/wustl-onlyint-FHE.csv";
    if let Err(e) = encrypt_csv(readcsv, writecsv, loaded_client_key){
        eprintln!("{}", e);
    }

    let total_time = total_start.elapsed();
    println!("TOTAL TIME TO RUN = {:?}", total_time);
    Ok(())   
}

// fn server_function(serialized_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//     let mut serialized_data = Cursor::new(serialized_data);
//     let server_key: ServerKey = bincode::deserialize_from(&mut serialized_data)?;
//     let ct_1: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
//     let ct_2: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;

//     set_server_key(server_key);

//     let result = ct_1 + ct_2;

//     let serialized_result = bincode::serialize(&result)?;

//     Ok(serialized_result)
// }

//FHE encrypt a csv file (<100 rows is ideal)
fn encrypt_csv(readpath: &str, writepath: &str, ck: ClientKey) -> Result<(), Box<dyn Error>>{
    //setup reader and writer for csv
    let mut reader = csv::Reader::from_path(readpath)?;
    let mut writer = csv::Writer::from_path(writepath)?;

    //create new csv file to write to
    let mut newfile = File::create(writepath).expect("failed to create server key file");

    //get headers
    println!("\nENCRYPTING CSV AND WRITING TO NEW ONE");
    let headers = reader.headers()?;
    println!("CSV HEADERS = {:?}", headers);

    //get rows, encrypt, then write to new file
    for result in reader.deserialize(){
        let start = Instant::now();
        let record: ReadRow = result?;
        //print!("Record type = ");
        //print_type_of(&record);
        //println!("{:?}", record.ST);

        //temp struct for writing rows
        let mut temp = WriteRow {
            SpO2: "".to_string(),
            Pulse_Rate: "".to_string(),
            SYS: "".to_string(),
            DIA: "".to_string(),
            Heart_rate: "".to_string(),
            Resp_Rate: "".to_string(),
            ST: "".to_string(),
            Label: "".to_string()
        };

        //Sp02
        temp.SpO2 = encrypt_serialize_encode(record.SpO2, ck.clone());
        //Pulse
        temp.Pulse_Rate = encrypt_serialize_encode(record.Pulse_Rate, ck.clone());
        //SYS
        temp.SYS = encrypt_serialize_encode(record.SYS, ck.clone());
        //DIA
        temp.DIA = encrypt_serialize_encode(record.DIA, ck.clone());
        //HR
        temp.Heart_rate = encrypt_serialize_encode(record.Heart_rate, ck.clone());
        //RR
        temp.Resp_Rate = encrypt_serialize_encode(record.Resp_Rate, ck.clone());
        //ST
        temp.ST = encrypt_serialize_encode(record.ST, ck.clone());
        //Label
        temp.Label = encrypt_serialize_encode(record.Label, ck.clone());

        //write temp to new csv file
        writer.serialize(temp)?;
        let writetime = start.elapsed();
        //println!("Wrote a row! ({:?})", writetime);
    }

    writer.flush()?;

    Ok(())
}

//encrypt, then bincode serialize, then b64 encode
fn encrypt_serialize_encode(input: u8, key: ClientKey) -> String {
    let ct = FheUint8::encrypt(input, &key);
    let ct_ser = bincode::serialize(&ct).unwrap();
    let ct_ser_b64 = general_purpose::STANDARD.encode(&ct_ser);
    return ct_ser_b64;
}

