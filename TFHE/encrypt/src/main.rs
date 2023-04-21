use std::time::{Instant};
use std::fs::File;
use std::io::{Write, Read};
use std::str;
use std::string;
use std::error::Error;
use csv;
use serde::{Deserialize, Serialize};
use tfhe::shortint::prelude::*;
use concrete::{ConfigBuilder, generate_keys, set_server_key, FheUint16, ServerKey, ClientKey};
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
    SpO2: u16,
    Pulse_Rate: u16,
    SYS: u16,
    DIA: u16,
    Heart_rate: u16,
    Resp_Rate: u16,
    ST: u16,
    Label: u16
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

    //1 if generate new keys
    //2 if use old keys

    let mut choice = String::new();
    println!("CHOOSE AN OPTION:\n(1) Generate new keys\n(2) Use old keys\n");
    let b1 = std::io::stdin().read_line(&mut choice).unwrap();

    let server_key_file = "/home/kunho/gatech-cybersec-practicum-2023/demo/ser_server_key.bin";
    let client_key_file = "/home/kunho/gatech-cybersec-practicum-2023/demo/ser_client_key.bin";

    let mut config = ConfigBuilder::all_disabled().enable_default_uint16().build();

    let total_start = Instant::now();
    
    if choice == "1\n" {
        let start = Instant::now();

        println!("\nGenerating keys...");

        let ( client_key, server_key) = generate_keys(config);

        let encoded_server_key = bincode::serialize(&server_key).unwrap();
        let encoded_client_key = bincode::serialize(&client_key).unwrap();

        // We write the keys to files:
        let mut file = File::create(server_key_file)
            .expect("failed to create server key file");
        file.write_all(encoded_server_key.as_slice()).expect("failed to write key to file");
        let mut file = File::create(client_key_file)
            .expect("failed to create client key file");
        file.write_all(encoded_client_key.as_slice()).expect("failed to write key to file");

        let writekeys = start.elapsed();

        println!("SUCCESSFULLY WROTE KEYS TO FILE ({:?})", writekeys);
    }
    
    // We retrieve the keys:
    let start = Instant::now();
    println!("\nRetrieving keys...");
    let mut file = File::open(server_key_file)
        .expect("failed to open server key file");
    let mut encoded_server_key: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_server_key).expect("failed to read the key");

    let mut file = File::open(client_key_file)
        .expect("failed to open client key file");
    let mut encoded_client_key: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_client_key).expect("failed to read the key");

    let retrievekeys = start.elapsed();

    println!("SUCCESSFULLY RETRIEVED KEYS ({:?})\n", retrievekeys);

    // We deserialize the keys:
    let start = Instant::now();
    println!("Deserializing keys...");

    let loaded_server_key: ServerKey = bincode::deserialize(&encoded_server_key[..])
        .expect("failed to deserialize");
    let loaded_client_key: ClientKey = bincode::deserialize(&encoded_client_key[..])
        .expect("failed to deserialize");

    set_server_key(loaded_server_key);

    let serializekeys = start.elapsed();

    println!("Key length = {:?}", encoded_server_key.len());
    println!("SUCCESSFULLY DESERIALIZED KEYS ({:?})\n", serializekeys);


    //csv stuff
    let readcsv = "/home/kunho/gatech-cybersec-practicum-2023/demo/wustl-onlyint.csv";
    let writecsv = "/home/kunho/gatech-cybersec-practicum-2023/demo/wustl-onlyint-FHE.csv";

    println!("Encrypting CSV...");
    let start = Instant::now();
    if let Err(e) = encrypt_csv(readcsv, writecsv, loaded_client_key){
        eprintln!("{}", e);
    }
    let encrypt_time = start.elapsed();
    println!("SUCCESFULLY ENCRYPTED CSV ({:?})\n", encrypt_time);

    let total_time = total_start.elapsed();
    println!("---------------------------------------------\nTOTAL TIME TO RUN = {:?}\n", total_time);
    Ok(())   
}

//FHE encrypt a csv file (<100 rows is ideal)
fn encrypt_csv(readpath: &str, writepath: &str, ck: ClientKey) -> Result<(), Box<dyn Error>>{
    //setup reader and writer for csv
    let mut reader = csv::Reader::from_path(readpath)?;
    let mut writer = csv::Writer::from_path(writepath)?;

    //create new csv file to write to
    let mut newfile = File::create(writepath).expect("failed to create server key file");

    //get headers
    //println!("\nENCRYPTING CSV AND WRITING TO NEW ONE");
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
fn encrypt_serialize_encode(input: u16, key: ClientKey) -> String {
    let ct = FheUint16::encrypt(input, &key);
    let ct_ser = bincode::serialize(&ct).unwrap();
    let ct_ser_b64 = general_purpose::STANDARD.encode(&ct_ser);
    return ct_ser_b64;
}

