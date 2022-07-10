mod app;
mod event;
mod terminal;

use argh::FromArgs;
use eyre::Result;
use std::time::Duration;

#[derive(Debug, FromArgs)]
#[argh(description = "...")]
struct Cli {
    /// time in ms between two ticks. defaults to: 33ms = 30 FPS
    #[argh(option, short = 't', default = "33")]
    tick_rate: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    terminal::run(tick_rate).await?;
    Ok(())
}
