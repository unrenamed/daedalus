mod app;
mod event;
mod terminal;

use argh::FromArgs;
use eyre::Result;
use std::time::Duration;

static MAZE_MIN_WIDTH: usize = 1;
static MAZE_MIN_HEIGHT: usize = 1;
static MAZE_MAX_WIDTH: usize = 45;
static MAZE_MAX_HEIGHT: usize = 45;

#[derive(Debug, FromArgs)]
#[argh(description = "Generate mazes")]
struct Cli {
    /// time in ms between two ticks. defaults to: 33ms (30 FPS)
    #[argh(option, short = 't', default = "33")]
    tick_rate: u64,
    /// grid width in characters. defaults to: 15
    #[argh(option, short = 'w', default = "15")]
    width: usize,
    /// grid height in characters. defaults to: 10
    #[argh(option, short = 'h', default = "10")]
    height: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = argh::from_env();
    validate_cli_options(&cli);

    let tick_rate = Duration::from_millis(cli.tick_rate);
    terminal::run(tick_rate, cli.width, cli.height).await?;

    Ok(())
}

fn validate_cli_options(cli: &Cli) {
    if cli.width < MAZE_MIN_WIDTH {
        panic!("Maze width must take at least {} cell.", MAZE_MIN_WIDTH);
    }

    if cli.height < MAZE_MIN_HEIGHT {
        panic!("Maze height must take at least {} cell.", MAZE_MIN_HEIGHT);
    }

    if cli.width > MAZE_MAX_WIDTH {
        panic!("Maze width can not be greater than {} cells.", MAZE_MAX_WIDTH);
    }

    if cli.height > MAZE_MAX_HEIGHT {
        panic!("Maze height can not be greater than {} cells.", MAZE_MAX_HEIGHT);
    }
}
