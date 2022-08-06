mod app;
mod event;
mod terminal;

use argh::FromArgs;
use eyre::Result;
use std::time::Duration;

#[derive(Debug, FromArgs)]
#[argh(description = "Generate mazes")]
struct Cli {
    /// time in ms between two ticks. defaults to: 33ms (30 FPS)
    #[argh(option, short = 't', default = "33")]
    tick_rate: u64,
    /// grid width in characters. defaults to: 15
    #[argh(option, short = 'w', default = "15")] // todo: define max width
    width: usize,
    /// grid height in characters. defaults to: 10
    #[argh(option, short = 'h', default = "10")] // todo: define max height
    height: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    terminal::run(tick_rate, cli.width, cli.height).await?;
    Ok(())
}
