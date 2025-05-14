use anyhow::Result;
use clap::Parser;
use ezbpf_core::errors::EZBpfError;
use ezbpf_core::program::Program;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filename of IDL file
    #[arg(short, long)]
    filename: String,

    /// Output destination
    #[arg(short, long)]
    output_file: Option<String>,

    /// Display assembly
    #[arg(short, long, default_value_t = false)]
    asm: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut file = File::open(args.filename)?;
    let mut b = vec![];
    file.read_to_end(&mut b)?;

    let program = Program::from_bytes(b.as_ref())?;
    let output: String;

    match args.asm {
        true => {
            output = program
                .section_header_entries
                .iter()
                .map(|h| h.ixs.clone())
                .filter(|ixs| !ixs.is_empty())
                .map(|ixs| {
                    ixs.iter()
                        .map(|i| i.to_asm().unwrap())
                        .collect::<Vec<String>>()
                        .join("\n")
                })
                .collect::<Vec<String>>()
                .join("\n");
            println!("{}", output);
        }
        false => {
            output = serde_json::to_string_pretty(&program)?;
            println!("{}", output);
        }
    }

    match args.output_file {
        Some(path) => {
            let mut file = File::create(path).expect("failed to create file");
            file.write_all(output.as_bytes())
                .expect("failed to write to file");
        }
        None => {}
    }
    
    Ok(())
}
