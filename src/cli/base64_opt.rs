use super::verify_exists;
use crate::process::{process_base64_decode, process_base64_encode};
use crate::CmdExecuter;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::fmt;
use std::{fmt::Display, str::FromStr};

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecuter)]
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

impl CmdExecuter for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encode = process_base64_encode(&self.input, self.format)?;
        println!("{}", encode);
        Ok(())
    }
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

impl CmdExecuter for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode = process_base64_decode(&self.input, self.format)?;
        // TODO: decode data might not string,
        // so we need to handle this case(but for this example, we assume it is)
        println!("{}", String::from_utf8(decode)?);
        Ok(())
    }
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
