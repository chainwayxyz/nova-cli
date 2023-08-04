use nova_scotia::circom::reader::load_r1cs;
use nova_scotia::create_public_params;
use nova_scotia::create_recursive_circuit;
use nova_scotia::FileLocation;
use nova_scotia::S;
use nova_scotia::F;
use nova_scotia::C1;
use nova_scotia::C2;

use nova_snark::CompressedSNARK;
use nova_snark::ProverKey;
use nova_snark::VerifierKey;
use nova_snark::PublicParams;
use nova_snark::provider;

use std::path::PathBuf;
use std::time::Instant;

use crate::utils::json_to_obj;
use crate::utils::obj_to_json;
use crate::utils::cbor_to_obj;
use crate::utils::obj_to_cbor;
use crate::utils::hexstr_to_u64;
use crate::utils::read_circuit_inputs;
use crate::utils::read_start_input;
use crate::utils::compile_circom;

type G1 = provider::bn256_grumpkin::bn256::Point;
type G2 = provider::bn256_grumpkin::grumpkin::Point;

pub fn setup(circom_path: PathBuf) {
    // start setup and timer
    let start = Instant::now();
    println!("setup start");

    // compile circom
    compile_circom(circom_path.clone());

    // load r1cs
    let mut r1cs_path = circom_path.clone();
    r1cs_path.set_extension("r1cs");
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(r1cs_path));

    // create public parameters
    let pp = create_public_params::<G1, G2>(r1cs.clone());

    // create pk and vk
    let (pk, vk) = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::setup(&pp).unwrap();

    // write pp to file
    let mut cbor_path = circom_path.clone();
    cbor_path.set_extension("pp");
    obj_to_cbor(cbor_path, pp);

    // write pk to file
    let mut pk_path = circom_path.clone();
    pk_path.set_extension("pk");
    obj_to_json(pk_path, pk);

    // write vk to file
    let mut vk_path = circom_path.clone();
    vk_path.set_extension("vk");
    obj_to_json(vk_path, vk);

    // print elapsed time
    println!("setup done in {:?}", start.elapsed());
}

pub fn prove(pp_path: PathBuf, pk_path: PathBuf, input_path: PathBuf, start_path: PathBuf) {
    // start prove and timer
    let start = Instant::now();
    println!("prove start");

    // load r1cs
    let mut r1cs_path = pp_path.clone();
    r1cs_path.set_extension("r1cs");
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(r1cs_path.clone()));

    // witness generator file name
    let mut witness_generator_file = r1cs_path.clone();
    let circuit_path2 = r1cs_path.clone();
    let r1cs_name = circuit_path2.file_name().unwrap().to_str().unwrap();
    witness_generator_file.pop();
    witness_generator_file.push(r1cs_name.replace(".r1cs", "_js"));
    witness_generator_file.push(r1cs_name.replace(".r1cs", ".wasm"));

    // read public parameters from file
    let pp: PublicParams<G1, G2, C1<G1>, C2<G2>> = cbor_to_obj(pp_path.clone());

    // read prover key from file
    let pk: ProverKey<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>> = json_to_obj(pk_path.clone());

    // read private circuit inputs
    let private_inputs = read_circuit_inputs(input_path);

    // read start public input
    let start_public_input_string = read_start_input(start_path);
    let start_public_input = vec![F::<G1>::from_raw(hexstr_to_u64(start_public_input_string.as_str()))];

    // recursive snark
    let recursive_snark = create_recursive_circuit(FileLocation::PathBuf(witness_generator_file), r1cs, private_inputs, start_public_input.clone(), &pp).unwrap();

    // prove
    let proof = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::prove(&pp, &pk, &recursive_snark).unwrap();

    // write proof to file
    let mut proof_path = pp_path.clone();
    proof_path.set_extension("proof");
    obj_to_json(proof_path, proof);

    // print elapsed time
    println!("prove done in {:?}", start.elapsed());
}

pub fn verify(proof_path: PathBuf, vk_path: PathBuf, start_path: PathBuf, iteration_count: usize) {
    // start verify and timer
    let start = Instant::now();
    println!("verify start");

    // read proof from file
    let proof: CompressedSNARK<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>> = json_to_obj(proof_path.clone());

    // read verifier key from file
    let vk: VerifierKey<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>> = json_to_obj(vk_path);

    // read start public input
    let start_public_input_string = read_start_input(start_path);
    let start_public_input = vec![F::<G1>::from_raw(hexstr_to_u64(start_public_input_string.as_str()))];

    // verify proof
    let z0_secondary = vec![F::<G2>::from(0)];
    let result = proof.verify(&vk, iteration_count, start_public_input.clone(), z0_secondary).unwrap();
    println!("-> {:?}", result.0[0]);

    // print elapsed time
    println!("verify done in {:?}", start.elapsed());
}
