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
    let server_key_file = "/home/kunho/Practicum/keys/ser_server_key.bin";
    let client_key_file = "/home/kunho/Practicum/keys/ser_client_key.bin";

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
    let readcsv = "/home/kunho/Practicum/iomt/convert/wustl-onlyint-FHE.csv";
    let writecsv = "/home/kunho/Practicum/iomt/convert/wustl-onlyint-FHE-avg.csv";

    println!("Operating on encrypted CSV...");
    let start = Instant::now();
    if let Err(e) = operate_csv(readcsv, writecsv, loaded_client_key){
        eprintln!("{}", e);
    }
    let encrypt_time = start.elapsed();
    println!("SUCCESFULLY OPERATED ON CSV ({:?})\n", encrypt_time);

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

    // let mut temp = ReadRow {
    //     SpO2: [],
    //     Pulse_Rate: [],
    //     SYS: [],
    //     DIA: [],
    //     Heart_rate: [],
    //     Resp_Rate: [],
    //     ST: [],
    //     Label: [];
    // }

    //get rows, encrypt, then write to new file

    let mut spo2=Vec::new();
    let mut pr=Vec::new();
    let mut sys=Vec::new();
    let mut dia=Vec::new();
    let mut hr=Vec::new();
    let mut rr=Vec::new();
    let mut st=Vec::new();
    let mut lbl=Vec::new();
    //let mut begin = true;

    for result in reader.deserialize(){
        let record: ReadRow = result?;

        spo2.push(decode_deserialize(record.SpO2));
        pr.push(decode_deserialize(record.Pulse_Rate));
        sys.push(decode_deserialize(record.SYS));
        dia.push(decode_deserialize(record.DIA));
        hr.push(decode_deserialize(record.Heart_rate));
        rr.push(decode_deserialize(record.Resp_Rate));
        st.push(decode_deserialize(record.ST));
        lbl.push(decode_deserialize(record.Label));
        //begin = false;
    }

    let spo2_sum: FheUint16 = sum(spo2, ck.clone());
    let pr_sum: FheUint16 = sum(pr, ck.clone());
    let sys_sum: FheUint16 = sum(sys, ck.clone());
    let dia_sum: FheUint16 = sum(dia, ck.clone());
    let hr_sum: FheUint16 = sum(hr, ck.clone());
    let rr_sum: FheUint16 = sum(rr,ck.clone());
    let st_sum: FheUint16 = sum(st, ck.clone());
    let lbl_sum: FheUint16 = sum(lbl, ck.clone());

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
    temp.SpO2 = serialize_encode(spo2_sum);
    //Pulse
    temp.Pulse_Rate = serialize_encode(pr_sum);
    //SYS
    temp.SYS = serialize_encode(sys_sum);
    //DIA
    temp.DIA = serialize_encode(dia_sum);
    //HR
    temp.Heart_rate = serialize_encode(hr_sum);
    //RR
    temp.Resp_Rate = serialize_encode(rr_sum);
    //ST
    temp.ST = serialize_encode(st_sum);
    //Label
    temp.Label = serialize_encode(lbl_sum);

    //write temp to new csv file
    writer.serialize(temp)?;
    //let writetime = start.elapsed();
    //println!("Wrote a row! ({:?})", writetime);

    writer.flush()?;

    Ok(())
}

//bincode serialize, then b64 encode
fn serialize_encode(input: FheUint16) -> String {
    let ct_ser = bincode::serialize(&input).unwrap();
    let ct_ser_b64 = general_purpose::STANDARD.encode(&ct_ser);
    return ct_ser_b64;
}

//b64 decode , then bincode deserialize
fn decode_deserialize(input: String) -> FheUint16 {
    let ct_dec = general_purpose::STANDARD.decode(&input).unwrap();
    let ct_des: FheUint16 = bincode::deserialize(&ct_dec).unwrap();
    return ct_des;
}

//sum ciphertexts
fn sum(input: Vec<FheUint16>, ck: ClientKey) -> FheUint16 {

    //initialize sum as first value in vector
    let mut sum = input[0].clone();

    for i in 1..input.len(){
        //get next value
        let ct = &input[i];
        // let ct_dec: u16 = ct.decrypt(&ck.clone());
        // println!("DECRYPTED VALUE = {:?}", ct_dec);

        let temp_sum = sum.clone() + ct.clone();

        //update sum
        sum = temp_sum;
        // let sum_dec: u16 = sum.decrypt(&ck.clone());
        // println!("NEW SUM = {:?}", sum_dec);
    }

    return sum.clone();
}
