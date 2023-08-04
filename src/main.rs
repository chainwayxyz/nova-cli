use std::path::PathBuf;
use clap::{Parser, Subcommand};

use nova_cli::functions::{setup, prove, verify};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Setup {
        file_circom: PathBuf
    },
    Prove {
        file_pp: PathBuf,
        file_pk: PathBuf,
        file_input: PathBuf,
        file_start: PathBuf
    },
    Verify {
        file_proof: PathBuf,
        file_vk: PathBuf,
        file_start: PathBuf,
        iteration_count: usize
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Setup {file_circom} => {
            println!("setup for {:?}", file_circom);
            setup(file_circom.to_path_buf());
        },
        Commands::Prove {file_pp, file_pk, file_input, file_start} => {
            println!("prove: {:?}", file_pp);
            prove(file_pp.to_path_buf(), file_pk.to_path_buf(), file_input.to_path_buf(), file_start.to_path_buf());
        },
        Commands::Verify {file_proof, file_vk, file_start, iteration_count} => {
            println!("verify: {:?}", file_proof);
            verify(file_proof.to_path_buf(), file_vk.to_path_buf(), file_start.to_path_buf(), iteration_count.to_owned());
        }
    }
}
