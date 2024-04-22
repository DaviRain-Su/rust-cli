use super::verify_file_exists;
use clap::Parser;
use std::fmt;
use std::{fmt::Display, str::FromStr};

#[derive(Parser, Debug)]
pub struct CsvOpts {
    /// csv input file
    #[arg(short, long, value_parser = verify_file_exists)]
    pub input: String,
    /// csv output file
    #[arg(short, long)]
    pub output: Option<String>,
    /// output format: json, yaml
    #[arg(long, value_parser = parse_format)]
    pub format: OutputFormat,
    /// delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    /// header
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

fn parse_format(s: &str) -> Result<OutputFormat, anyhow::Error> {
    s.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(f: OutputFormat) -> Self {
        match f {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}

impl TryFrom<&str> for OutputFormat {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}
