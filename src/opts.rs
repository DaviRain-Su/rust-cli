use clap::Parser;

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
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file_exists(path: &str) -> Result<String, String> {
    if std::path::Path::new(path).exists() {
        Ok(path.into())
    } else {
        Err("File does not exist".into())
    }
}
