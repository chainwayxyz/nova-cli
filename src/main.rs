use std::path::PathBuf;
use clap::{Parser, Subcommand};

use nova_cli::functions::{setup, prove, verify};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

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
            if cli.verbose {println!("setup for {:?}", file_circom.with_extension(""));}
            setup(file_circom.to_path_buf(), cli.verbose);
        },
        Commands::Prove {file_pp, file_pk, file_input, file_start} => {
            if cli.verbose {println!("prove for {:?}", file_pp.with_extension(""));} 
            prove(file_pp.to_path_buf(), file_pk.to_path_buf(), file_input.to_path_buf(), file_start.to_path_buf(), cli.verbose);
        },
        Commands::Verify {file_proof, file_vk, file_start, iteration_count} => {
            if cli.verbose {println!("verify: {:?}", file_proof.with_extension(""));}
            verify(file_proof.to_path_buf(), file_vk.to_path_buf(), file_start.to_path_buf(), iteration_count.to_owned(), cli.verbose);
        }
    }
}
