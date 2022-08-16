use crate::{
    app::App,
    app::{ui, AppReturn},
    event::{Event, Events},
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use log::LevelFilter;
use std::{io, sync::Arc, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub async fn run(tick_rate: Duration, width: usize, height: usize) -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app
    let app = Arc::new(tokio::sync::Mutex::new(App::new("Maze Generator", width, height)));
    let app_ui = Arc::clone(&app);

    // configure logger
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    // run app
    start_ui(&mut terminal, &app_ui, tick_rate).await?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}

async fn start_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &Arc<tokio::sync::Mutex<App<'_>>>,
    tick_rate: Duration,
) -> Result<()> {
    let mut events = Events::new(tick_rate);

    loop {
        let mut app = app.lock().await;

        terminal.draw(|f| ui::draw(f, &mut app))?;

        let result = match events.next().await {
            // process that event
            Event::Input(key) => app.do_action(key),
            // handle no user input
            Event::Tick => app.update_on_tick(),
        };

        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    Ok(())
}
