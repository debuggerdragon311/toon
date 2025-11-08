use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use toon::{decode_toon_to_json, encode_json_to_toon, DecodeOptions, EncodeOptions};

#[derive(Parser)]
#[command(name = "toon")]
#[command(about = "A compact, lossless JSON encoding format", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode JSON to TOON format
    Encode {
        /// Input JSON file (stdin if not provided)
        input: Option<PathBuf>,

        /// Output file (stdout if not provided)
        #[arg(short, long)]
        out: Option<PathBuf>,

        /// Use tabular layout for uniform arrays of objects
        #[arg(long)]
        tabular_arrays: bool,

        /// Use compact binary format
        #[arg(long)]
        compact: bool,

        /// Indentation in spaces (default: 2)
        #[arg(long)]
        indent: Option<u8>,

        /// Fail on validation errors
        #[arg(long)]
        strict: bool,
    },

    /// Decode TOON to JSON format
    Decode {
        /// Input TOON file (stdin if not provided)
        input: Option<PathBuf>,

        /// Output file (stdout if not provided)
        #[arg(short, long)]
        out: Option<PathBuf>,

        /// Fail on validation errors
        #[arg(long)]
        strict: bool,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Encode {
            input,
            out,
            tabular_arrays,
            compact,
            indent,
            strict,
        } => {
            let input_data = read_input(input.as_deref())?;
            let json: serde_json::Value = serde_json::from_slice(&input_data)
                .context("Failed to parse input JSON")?;

            let options = EncodeOptions {
                tabular_arrays,
                compact,
                indent,
                strict,
            };

            let toon_data = encode_json_to_toon(&json, &options)
                .context("Failed to encode JSON to TOON")?;

            write_output(out.as_deref(), &toon_data)?;
            Ok(())
        }

        Commands::Decode { input, out, strict } => {
            let input_data = read_input(input.as_deref())?;

            let options = DecodeOptions {
                compact: false, // Auto-detect
                strict,
            };

            let json = decode_toon_to_json(&input_data, &options)
                .context("Failed to decode TOON to JSON")?;

            let json_str = serde_json::to_string_pretty(&json)
                .context("Failed to serialize JSON")?;

            write_output(out.as_deref(), json_str.as_bytes())?;
            Ok(())
        }
    }
}

fn read_input(path: Option<&std::path::Path>) -> Result<Vec<u8>> {
    match path {
        Some(p) => fs::read(p).with_context(|| format!("Failed to read file: {:?}", p)),
        None => {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            Ok(buf)
        }
    }
}

fn write_output(path: Option<&std::path::Path>, data: &[u8]) -> Result<()> {
    match path {
        Some(p) => fs::write(p, data).with_context(|| format!("Failed to write file: {:?}", p)),
        None => {
            io::stdout()
                .write_all(data)
                .context("Failed to write to stdout")?;
            Ok(())
        }
    }
}
