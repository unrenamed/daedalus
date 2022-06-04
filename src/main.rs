mod algos;
mod app;
mod grid;
mod terminal;
mod ui;
mod utils;
mod widgets;

use crate::terminal::run;

use argh::FromArgs;
use std::{error::Error, time::Duration};

#[derive(Debug, FromArgs)]
#[argh(description = "...")]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, short = 't', default = "10")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate, cli.enhanced_graphics)?;
    Ok(())
}
