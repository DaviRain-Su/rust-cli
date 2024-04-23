use super::{verify_exists, verify_path_exists};
use crate::process::{process_text_decrypt, process_text_encrypt};
use crate::CmdExecuter;
use chacha20poly1305::{
    aead::{KeyInit, OsRng},
    ChaCha20Poly1305,
};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecuter)]
pub enum Chacha20Poly1305SubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Verify a sign message")]
    Decrypt(TextDecryptOpts),
    #[command(name = "generator", about = "Generate a new key")]
    GenKey(TextChacCha20GenKeyOpts),
}

#[derive(Parser, Debug)]
pub struct TextEncryptOpts {
    /// Input file, use `-` for stdin
    #[arg(short, long, value_parser = verify_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_exists)]
    pub key: String,
}

impl CmdExecuter for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = process_text_encrypt(&self.input, &self.key)?;
        println!("{}", sig);
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct TextDecryptOpts {
    /// Input file, use `-` for stdin
    #[arg(short, long, value_parser = verify_exists, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_exists)]
    pub key: String,
}

impl CmdExecuter for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode_result = process_text_decrypt(&self.input, &self.key)?;
        println!("{}", decode_result);
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct TextChacCha20GenKeyOpts {
    #[arg(short, long, value_parser = verify_path_exists)]
    pub output: PathBuf,
}

impl CmdExecuter for TextChacCha20GenKeyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let name = self.output.join("chacha20poly1305.txt");
        tokio::fs::write(name, &key).await?;
        Ok(())
    }
}
