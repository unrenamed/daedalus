mod app;
mod event;
mod terminal;

use argh::FromArgs;
use eyre::Result;
use std::time::Duration;

#[derive(Debug, FromArgs)]
#[argh(description = "...")]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, short = 't', default = "15")]
    tick_rate: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    terminal::run(tick_rate).await?;
    Ok(())
}
