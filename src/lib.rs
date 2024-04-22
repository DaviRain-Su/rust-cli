use enum_dispatch::enum_dispatch;

mod cli;
mod process;
mod utils;

pub use cli::*;
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecuter {
    async fn execute(self) -> anyhow::Result<()>;
}
