use clap::Parser;
use std::io::{self, Read, Write};
use std::fs::File;

use base64;

#[derive(Parser)]

#[command(version, about = "Base64 Encoder, Version: 0.0.1 alpha")]
struct Args {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let arguments = Args::parse();
    match (arguments.input, arguments.output) 
    {
        (Some(in_path), Some(out_path)) => {
            match read_and_encode_file(&in_path, &out_path) 
            {
                Ok(_) => println!("Encoded to base64."),
                Err(err) => eprintln!("Error: {}", err)
            }
        }

        (None, None) => 
        {
            let mut in_data = String::new();
            match io::stdin().read_line(&mut in_data) 
            {
                Ok(_) => 
                {
                    let encoded_data = base64::encode_function(in_data.trim().as_bytes());
                    println!("{}", encoded_data);
                }
                Err(err) => eprintln!("Error reading from stdin: {}", err),
            }
        }
        _ => eprintln!("Specify input file and output file."),
    }
}

fn read_and_encode_file(in_path: &str, out_path: &str) -> io::Result<()> {

    let mut input_file = File::open(in_path)?;
    let mut in_data = Vec::new();

    input_file.read_to_end(&mut in_data)?;
    let encoded_data = base64::encode_function(&in_data);

    let mut output_file = File::create(out_path)?;
    output_file.write_all(encoded_data.as_bytes())?;
    Ok(())
}