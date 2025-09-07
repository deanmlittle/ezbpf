use anyhow::Result;
use clap::Parser;
use ezbpf_core::program::Program;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filename of IDL file
    #[arg(short, long)]
    filename: String,
    /// Print asm instead of json
    #[arg(short, long)]
    asm: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut file = File::open(args.filename)?;
    let mut b = vec![];
    file.read_to_end(&mut b)?;
    let program = Program::from_bytes(b.as_ref())?;

    if args.asm {
        println!(
            "{}",
            program
                .section_header_entries
                .iter()
                .map(|h| h.ixs.clone())
                .filter(|ixs| !ixs.is_empty())
                .map(|ixs| ixs
                    .iter()
                    .map(|i| i.to_asm().unwrap())
                    .collect::<Vec<String>>()
                    .join("\n"))
                .collect::<Vec<String>>()
                .join("\n")
        );
    } else {
        println!("{}", serde_json::to_string_pretty(&program)?);
    }

    Ok(())
}
