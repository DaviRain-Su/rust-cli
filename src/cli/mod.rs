use clap::Parser;

mod base64_opt;
mod csv_opt;
mod genpass_opt;
mod text;

pub use base64_opt::{Base64Format, Base64SubCommand};
pub use csv_opt::{CsvOpts, OutputFormat};
pub use genpass_opt::GenPassOpts;
pub use text::{TextSignFormat, TextSubCommand};

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
