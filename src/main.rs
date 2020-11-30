use serde::Serialize;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::result::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "as-keyer",
    about = "A CLI tool to generate app-store.json file for using Fastlane AppStore Connect API
"
)]
struct Opt {
    /// Path to .p8 key file
    #[structopt(long)]
    input: PathBuf,

    /// Key ID for the p8 file
    #[structopt(long)]
    key_id: String,

    /// Issuer ID value
    #[structopt(long)]
    issuer_id: String,

    /// Output file location
    #[structopt(long, default_value = ".")]
    output: PathBuf,
}

#[derive(Serialize)]
struct AppStoreJSON {
    key_id: String,
    issuer_id: String,
    key: String,
    duration: isize,
    in_house: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let file_content = File::open(opt.input).and_then(|mut file| {
        let mut buffer = String::new();
        if let Err(err) = file.read_to_string(&mut buffer) {
            return Err(err);
        }
        Ok(buffer)
    })?;

    let json = AppStoreJSON {
        key_id: opt.key_id,
        issuer_id: opt.issuer_id,
        key: file_content,
        duration: 1200,
        in_house: false,
    };
    let encoded = serde_json::to_vec(&json)?;

    let mut output = opt.output;
    output.push("api-key.json");
    let mut result = File::create(output)?;
    let _ = result.write(&encoded)?;
    Ok(())
}
