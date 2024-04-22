use clap::Parser;

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    /// Length of the password
    #[arg(long, default_value = "16")]
    pub length: usize,
    /// Uppercase letters
    #[arg(short, long, default_value_t = true)]
    pub uppercase: bool,
    /// Lowercase letters
    #[arg(short, long, default_value_t = true)]
    pub lowercase: bool,
    /// Numbers
    #[arg(long, default_value_t = true)]
    pub number: bool,
    /// Symbols
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
