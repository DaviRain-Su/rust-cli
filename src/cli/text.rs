use clap::Parser;
use std::fmt;
use std::path::PathBuf;
use std::{fmt::Display, str::FromStr};

use super::verify_exists;

#[derive(Parser, Debug)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a sign message")]
    Verify(TextVerifyOpts),
    #[command(name = "generator", about = "Generate a new key")]
    GenKey(TextGenKeyOpts),
}

#[derive(Parser, Debug)]
pub struct TextSignOpts {
    /// Input file, use `-` for stdin
    #[arg(short, long, value_parser = verify_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_exists)]
    pub key: String,
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct TextVerifyOpts {
    /// Input file, use `-` for stdin
    #[arg(short, long, value_parser = verify_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_exists)]
    pub key: String,
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Parser, Debug)]
pub struct TextGenKeyOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path_exists)]
    pub output: PathBuf,
}

fn verify_path_exists(path: &str) -> Result<PathBuf, &'static str> {
    let path = PathBuf::from(path);
    if path.exists() && path.is_dir() {
        Ok(path)
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[derive(Parser, Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "Blake3"),
            TextSignFormat::Ed25519 => write!(f, "Ed25519"),
        }
    }
}

impl From<TextSignFormat> for String {
    fn from(f: TextSignFormat) -> Self {
        match f {
            TextSignFormat::Blake3 => "Blake3".to_string(),
            TextSignFormat::Ed25519 => "Ed25519".to_string(),
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}
