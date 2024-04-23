use crate::process::{process_jwt_sign, process_jwt_verify};
use crate::CmdExecuter;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecuter)]
pub enum JwtSubCommand {
    #[command(about = "")]
    Sign(JwtSignOpts),
    #[command(about = "")]
    Verify(JwtVerifyOpts),
}

#[derive(Parser, Debug)]
pub struct JwtSignOpts {
    /// Subject
    #[arg(long)]
    pub sub: String,
    /// Audience
    #[arg(long)]
    pub aud: String,
    /// Expiry, format "<number>d" for days
    #[arg(long)]
    pub exp: String,
}

impl CmdExecuter for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = process_jwt_sign(self.sub, self.aud, self.exp)?;
        println!("{}", sig);
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct JwtVerifyOpts {
    /// Input file, use `-` for stdin
    #[arg(short, long)]
    pub token: String,
    /// Audience
    #[arg(long, default_value = "me")]
    pub aud: String,
}

impl CmdExecuter for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verifyed = process_jwt_verify(self.token, self.aud)?;
        println!("{}", verifyed);
        Ok(())
    }
}
