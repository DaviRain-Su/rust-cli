use clap::Parser;

pub mod base64_opt;
pub mod csv_opt;
pub mod genpass_opt;
pub mod text;

use base64_opt::Base64SubCommand;
use csv_opt::CsvOpts;
use genpass_opt::GenPassOpts;
use text::TextSubCommand;

#[derive(Parser, Debug)]
#[command(name = "csv", version, about, author, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode or decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text processing")]
    Text(TextSubCommand),
}

fn verify_exists(filename: &str) -> Result<String, String> {
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_exists() {
        assert_eq!(verify_exists("*"), Err("File does not exist".into()));
        assert_eq!(verify_exists("-"), Ok("-".into()));
        assert_eq!(
            verify_exists("not_exist.txt"),
            Err("File does not exist".into())
        );
        assert_eq!(verify_exists("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
