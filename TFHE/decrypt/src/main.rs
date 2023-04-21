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
//use concrete::integers::*;
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
    SpO2: String,
    Pulse_Rate: String,
    SYS: String,
    DIA: String,
    Heart_rate: String,
    Resp_Rate: String,
    ST: String,
    Label: String
}

//struct for writing rows
#[derive(Debug, Serialize)]
struct WriteRow {
    SpO2: u16,
    Pulse_Rate: u16,
    SYS: u16,
    DIA: u16,
    Heart_rate: u16,
    Resp_Rate: u16,
    ST: u16,
    Label: u16
}



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let server_key_file = "/home/kunho/gatech-cybersec-practicum-2023/demo/ser_server_key.bin";
    let client_key_file = "/home/kunho/gatech-cybersec-practicum-2023/demo/ser_client_key.bin";

    let mut config = ConfigBuilder::all_disabled().enable_default_uint16().build();

    let total_start = Instant::now();

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
    let readcsv = "/home/kunho/gatech-cybersec-practicum-2023/demo/wustl-onlyint-FHE-avg.csv";
    let writecsv = "/home/kunho/gatech-cybersec-practicum-2023/demo/wustl-onlyint-FHE-decrypt.csv";

    println!("Decrypting CSV...");
    let start = Instant::now();
    if let Err(e) = operate_csv(readcsv, writecsv, loaded_client_key){
        eprintln!("{}", e);
    }
    let encrypt_time = start.elapsed();
    println!("SUCCESFULLY Decrypted CSV ({:?})\n", encrypt_time);

    let total_time = total_start.elapsed();
    println!("---------------------------------------------\nTOTAL TIME TO RUN = {:?}\n", total_time);
    Ok(())   
}

//FHE encrypt a csv file (<100 rows is ideal)
fn operate_csv(readpath: &str, writepath: &str, ck: ClientKey) -> Result<(), Box<dyn Error>>{
    //setup reader and writer for csv
    let mut reader = csv::Reader::from_path(readpath)?;
    let mut writer = csv::Writer::from_path(writepath)?;

    //create new csv file to write to
    let mut newfile = File::create(writepath).expect("failed to create csv file");

    //get headers
    //println!("\nENCRYPTING CSV AND WRITING TO NEW ONE");
    let headers = reader.headers()?;
    println!("CSV HEADERS = {:?}", headers);

    //get rows, encrypt, then write to new file

   //temp struct for writing rows
    let mut temp = WriteRow {
        SpO2: 1,
        Pulse_Rate: 1,
        SYS: 1,
        DIA: 1,
        Heart_rate: 1,
        Resp_Rate: 1,
        ST: 1,
        Label: 1
    };

    for result in reader.deserialize(){
        let record: ReadRow = result?;

        //print_type_of(&decode_deserialize(record.SpO2));

        //Sp02
        temp.SpO2 = decode_deserialize(record.SpO2).decrypt(&ck.clone());
        //Pulse
        temp.Pulse_Rate = decode_deserialize(record.Pulse_Rate).decrypt(&ck.clone());
        //SYS
        temp.SYS = decode_deserialize(record.SYS).decrypt(&ck.clone());
        //DIA
        temp.DIA = decode_deserialize(record.DIA).decrypt(&ck.clone());
        //HR
        temp.Heart_rate = decode_deserialize(record.Heart_rate).decrypt(&ck.clone());
        //RR
        temp.Resp_Rate = decode_deserialize(record.Resp_Rate).decrypt(&ck.clone());
        //ST
        temp.ST = decode_deserialize(record.ST).decrypt(&ck.clone());
        //Label
        temp.Label = decode_deserialize(record.Label).decrypt(&ck.clone());
    }

    println!("---------------------------------------------\nCalculated averages: ");
    println!("Sp02 = {:?}", temp.SpO2/50);
    println!("Pulse_Rate = {:?}", temp.Pulse_Rate/50);
    println!("SYS = {:?}", temp.SYS/50);
    println!("DIA = {:?}", temp.DIA/50);
    println!("Heart_Rate = {:?}", temp.Heart_rate/50);
    println!("Resp_Rate = {:?}", temp.Resp_Rate/50);
    println!("ST = {:?}", temp.ST/50);
    println!("Label = {:?}", temp.Label/50);

    //write temp to new csv file
    writer.serialize(temp)?;
    //let writetime = start.elapsed();
    //println!("Wrote a row! ({:?})", writetime);

    writer.flush()?;

    Ok(())
}

//b64 decode , then bincode deserialize
fn decode_deserialize(input: String) -> FheUint16 {
    let ct_dec = general_purpose::STANDARD.decode(&input).unwrap();
    let ct_des: FheUint16 = bincode::deserialize(&ct_dec).unwrap();
    return ct_des;
}
