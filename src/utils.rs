use std::path::PathBuf;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use std::collections::HashMap;
use std::process::Command;
use std::process::Stdio;
use serde::de::DeserializeOwned;
use serde_cbor;
use serde_json::Value;
use serde_json;


pub fn json_to_obj<T: DeserializeOwned>(file_path: PathBuf) -> T {
    let file = File::open(file_path).expect("error");
    let reader = BufReader::new(file);
    let a: T = serde_json::from_reader(reader).expect("error");
    return a;
}

pub fn obj_to_json<T: serde::Serialize>(file_path: PathBuf, obj: T) {
    let file = File::create(file_path).expect("error");
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &obj).expect("write cbor error");
}

pub fn cbor_to_obj<T: DeserializeOwned>(file_path: PathBuf) -> T {
    let file = File::open(file_path).expect("error");
    let reader = BufReader::new(file);
    let a: T = serde_cbor::from_reader(reader).expect("error");
    return a;
}

pub fn obj_to_cbor<T: serde::Serialize>(file_path: PathBuf, obj: T) {
    let file = File::create(file_path).expect("error");
    let writer = BufWriter::new(file);
    serde_cbor::to_writer(writer, &obj).expect("write cbor error");
}

pub fn read_circuit_inputs(input_path: PathBuf) -> Vec<HashMap<String, Value>> {
    let input: Value = json_to_obj(input_path);
    let circuit_inputs = input.as_array().unwrap();
    let mut private_inputs = Vec::new();
    for circuit_input in circuit_inputs {
        let mut private_input = HashMap::new();
        let circuit = circuit_input.as_object().unwrap().clone();
        for (k, v) in circuit {
            private_input.insert(k, v);
        }
        private_inputs.push(private_input);
    }
    return private_inputs;
}

pub fn read_start_input(start_path: PathBuf) -> String {
    let start_input: Value = json_to_obj(start_path);
    let a = start_input.as_object().unwrap().get("step_in").unwrap().as_str().unwrap().to_string();
    return a;
}

pub fn compile_circom(circom_path: PathBuf, verbose: bool) {
    if verbose {println!("compiling circom file {:?}", circom_path);}
    let mut a = Command::new("circom");
    let parent = circom_path.parent().expect("no parent");
    if !verbose {
        a.stdout(Stdio::null());
    }
    a.arg(circom_path.clone()).arg("--wasm").arg("--r1cs").arg("-o").arg(parent);
    a.status().expect("process failed");
}

pub fn hexstr_to_u64(hex_string: &str) -> [u64; 4] {
    if hex_string.len() != 64 {
        panic!("Invalid hex string length. Expected 64 characters.");
    }
    let mut parts = [0u64; 4];
    for i in 0..4 {
        let start = i * 16;
        let end = start + 16;
        let slice = &hex_string[start..end];
        let num = u64::from_str_radix(slice, 16).expect("Invalid hex string.");
        parts[3 - i] = num;
    }
    return parts;
}
