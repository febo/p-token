use std::path::Path;

use clap::Parser;
use console::style;
use mollusk_svm::{fuzz::check::FixtureCheck, Mollusk};
use mollusk_svm_fuzz_fixture::Fixture;
use solana_pubkey::Pubkey;
use solana_sdk::bpf_loader_upgradeable;

/// The `p-token` program ID.
const TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array(token_interface::program::ID);

/// Simple CLI to execute a fixture against the `p-token` program.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filename of the fixture to run.
    #[arg()]
    file: Option<String>,

    /// Directory containing the fixtures.
    #[clap(short, long)]
    directory: Option<String>,
}

fn main() {
    let args = Args::parse();

    if std::env::var("SBF_OUT_DIR").is_err() {
        println!(
            "{} SBF_OUT_DIR is not set. Please set it to the output directory of the BPF build.",
            style("[ 🔴 ERROR ]").red(),
        );
        std::process::exit(1);
    }

    let fixtures = if let Some(file) = args.file {
        let path = Path::new(&file);
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        vec![(filename, file)]
    } else if let Some(directory) = args.directory {
        let dir = std::fs::read_dir(directory).unwrap();
        dir.filter_map(|entry| {
            let path = entry.unwrap().path();
            let filename = String::from(path.file_name().unwrap().to_str().unwrap());
            if path.is_file() && path.extension().is_some_and(|ext| ext == "fix") {
                Some((filename, path.to_str().unwrap().to_string()))
            } else {
                None
            }
        })
        .collect()
    } else {
        println!(
            "{} Missing 'file' or 'directory' argument.",
            style("[ 🔴 ERROR ]").red(),
        );
        std::process::exit(1);
    };

    let mut mollusk = Mollusk::default();
    mollusk.add_program(
        &TOKEN_PROGRAM_ID,
        "token_program",
        &bpf_loader_upgradeable::ID,
    );

    let width = format!("{}", fixtures.len()).len();

    fixtures
        .iter()
        .enumerate()
        .for_each(|(index, (path, filename))| {
            print!(
                "[ {:width$} / {:width$} ]: {:?}",
                index + 1,
                fixtures.len(),
                path
            );

            let fixture = Fixture::load_from_blob_file(filename);
            mollusk.process_and_partially_validate_fixture(
                &fixture,
                &[
                    FixtureCheck::ProgramResult,
                    FixtureCheck::ReturnData,
                    FixtureCheck::all_resulting_accounts(),
                ],
            );

            println!("\t✔️");
        });
}
