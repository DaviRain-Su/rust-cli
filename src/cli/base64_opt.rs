use super::verify_exists;
use clap::Parser;
use std::fmt;
use std::{fmt::Display, str::FromStr};

#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to Base64")]
    Base64Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a Base64 string")]
    Base64Decode(Base64DecodeOpts),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    /// Input file, use `-` for stdin
    #[arg(short, long, value_parser = verify_exists, default_value = "-")]
    pub input: String,
    /// Support format: standard and urlsafe
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    /// Input file, use `-` for stdin
    #[arg(short, long, value_parser = verify_exists, default_value = "-")]
    pub input: String,
    /// Support format: standard and urlsafe
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    Urlsafe,
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "Standard"),
            Base64Format::Urlsafe => write!(f, "URL-safe"),
        }
    }
}

impl From<Base64Format> for String {
    fn from(f: Base64Format) -> Self {
        match f {
            Base64Format::Standard => "Standard".to_string(),
            Base64Format::Urlsafe => "URL-safe".to_string(),
        }
    }
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::Urlsafe),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}
