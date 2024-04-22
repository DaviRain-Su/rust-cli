use crate::process::process_genpass;
use crate::CmdExecuter;
use clap::Parser;
use zxcvbn::zxcvbn;

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

impl CmdExecuter for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        // Make sure the password have at least one of each type

        let passwords_string = String::from_utf8(password)?;
        println!("{}", passwords_string);

        let estimate = zxcvbn(&passwords_string, &[]).unwrap();
        eprintln!("password strength {}", estimate.score());
        Ok(())
    }
}
