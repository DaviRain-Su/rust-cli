use super::verify_path_exists;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "Serve a file over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Parser, Debug)]
pub struct HttpServeOpts {
    #[arg(long, value_parser = verify_path_exists, default_value = ".")]
    pub path: PathBuf,
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}
