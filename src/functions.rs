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
use crate::utils::hexstr_to_4u64;
use crate::utils::read_circuit_inputs;
use crate::utils::read_start_input;
use crate::utils::compile_circom;

type G1 = provider::bn256_grumpkin::bn256::Point;
type G2 = provider::bn256_grumpkin::grumpkin::Point;

pub fn setup(circom_path: PathBuf, verbose: bool) {
    // start setup and timer
    let start = Instant::now();
    if verbose {println!("setup start");}

    // compile circom
    compile_circom(circom_path.clone(), verbose);
    if verbose {println!("circom file {:?} compiled", circom_path.clone());}

    // load r1cs
    let mut r1cs_path = circom_path.clone();
    r1cs_path.set_extension("r1cs");
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(r1cs_path.clone()));
    if verbose {println!("r1cs loaded from file {:?}", r1cs_path);}

    // create public parameters
    let pp = create_public_params::<G1, G2>(r1cs.clone());
    if verbose {println!("public parameters created");}

    // create pk and vk
    let (pk, vk) = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::setup(&pp).unwrap();
    if verbose {println!("prover and verifier key created");}

    // write pp to file
    let mut cbor_path = circom_path.clone();
    cbor_path.set_extension("pp");
    obj_to_cbor(cbor_path.clone(), pp);
    if verbose {println!("public parameters written to file {:?}", cbor_path);}

    // write pk to file
    let mut pk_path = circom_path.clone();
    pk_path.set_extension("pk");
    obj_to_json(pk_path.clone(), pk);
    if verbose {println!("prover key written to file {:?}", pk_path);}

    // write vk to file
    let mut vk_path = circom_path.clone();
    vk_path.set_extension("vk");
    obj_to_json(vk_path.clone(), vk);
    if verbose {println!("verifier key written to file {:?}", vk_path);}

    // print elapsed time
    if verbose {println!("setup done in {:?}", start.elapsed());}
}

pub fn prove(pp_path: PathBuf, pk_path: PathBuf, input_path: PathBuf, start_path: PathBuf, verbose: bool) {
    // start prove and timer
    let start = Instant::now();
    if verbose {println!("prove start");}

    // load r1cs
    let mut r1cs_path = pp_path.clone();
    r1cs_path.set_extension("r1cs");
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(r1cs_path.clone()));
    if verbose {println!("r1cs loaded from file {:?}", r1cs_path);}

    // witness generator file name
    let mut witness_generator_file = r1cs_path.clone();
    let circuit_path2 = r1cs_path.clone();
    let r1cs_name = circuit_path2.file_name().unwrap().to_str().unwrap();
    witness_generator_file.pop();
    witness_generator_file.push(r1cs_name.replace(".r1cs", "_js"));
    witness_generator_file.push(r1cs_name.replace(".r1cs", ".wasm"));
    if verbose {println!("witness generator filename is {:?}", witness_generator_file.clone());}

    // read public parameters from file
    let pp: PublicParams<G1, G2, C1<G1>, C2<G2>> = cbor_to_obj(pp_path.clone());
    if verbose {println!("public parameters read from file {:?}", pp_path);}

    // read prover key from file
    let pk: ProverKey<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>> = json_to_obj(pk_path.clone());
    if verbose {println!("prover key read from file {:?}", pk_path);}

    // read private circuit inputs
    let private_inputs = read_circuit_inputs(input_path.clone());
    if verbose {println!("private inputs read from file {:?}", input_path);}

    // read start public input
    let start_public_input_vector = read_start_input(start_path.clone());
    let mut start_public_input = Vec::new();
    for a in start_public_input_vector {
        start_public_input.push(F::<G1>::from_raw(hexstr_to_4u64(a)));
    }
    if verbose {println!("start public input read from file {:?}", start_path);}

    // recursive snark
    let recursive_snark = create_recursive_circuit(FileLocation::PathBuf(witness_generator_file), r1cs, private_inputs, start_public_input.clone(), &pp).unwrap();
    if verbose {println!("recursive snark done");}

    // prove
    let proof = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::prove(&pp, &pk, &recursive_snark).unwrap();
    if verbose {println!("proof done");}

    // write proof to file
    let mut proof_path = pp_path.clone();
    proof_path.set_extension("proof");
    obj_to_json(proof_path.clone(), proof);
    if verbose {println!("proof written to file {:?}", proof_path);}

    // print elapsed time
    if verbose {println!("prove done in {:?}", start.elapsed());}
}

pub fn verify(proof_path: PathBuf, vk_path: PathBuf, start_path: PathBuf, iteration_count: usize, verbose: bool) {
    // start verify and timer
    let start = Instant::now();
    if verbose {println!("verify start");}

    // read proof from file
    let proof: CompressedSNARK<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>> = json_to_obj(proof_path.clone());
    if verbose {println!("proof read from file {:?}", proof_path);}

    // read verifier key from file
    let vk: VerifierKey<G1, G2, C1<G1>, C2<G2>, S<G1>, S<G2>> = json_to_obj(vk_path.clone());
    if verbose {println!("verifier key read from file {:?}", vk_path);}

    // read start public input
    let start_public_input_vector = read_start_input(start_path.clone());
    let mut start_public_input = Vec::new();
    for a in start_public_input_vector {
        start_public_input.push(F::<G1>::from_raw(hexstr_to_4u64(a)));
    }
    if verbose {println!("start public input read from file {:?}", start_path);}

    // verify proof
    let z0_secondary = vec![F::<G2>::from(0)];
    let result = proof.verify(&vk, iteration_count, start_public_input.clone(), z0_secondary).unwrap();
    if verbose {println!("proof verified, {:?}", result.0);}

    // print elapsed time
    if verbose {println!("verify done in {:?}", start.elapsed());}
}
