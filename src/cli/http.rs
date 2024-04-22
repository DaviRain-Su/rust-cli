use super::verify_path_exists;
use crate::process::process_http_serve;
use crate::CmdExecuter;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecuter)]
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

impl CmdExecuter for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{:?}", self);
        println!("http server is running on http://0.0.0.0:{}", self.port);
        process_http_serve(self.path, self.port).await
    }
}
