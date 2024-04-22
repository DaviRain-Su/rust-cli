use clap::Parser;

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    #[arg(long, default_value = "16")]
    pub length: usize,

    #[arg(short, long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(short, long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
