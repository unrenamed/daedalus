use log::{debug, warn};

use crate::event::Key;

use self::{
    actions::{Action, Actions},
    state::AppState,
};

mod actions;
mod algos;
mod grid;
mod state;
mod utils;
mod widgets;

pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App<'a> {
    title: &'a str,
    /// Contextual actions
    pub actions: Actions,
    /// State
    pub state: AppState<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        let actions = vec![
            Action::Quit,
            Action::GotoNextTab,
            Action::GotoPrevTab,
            Action::SelectNextAlgo,
            Action::SelectPrevAlgo,
            Action::RunMazeGeneration,
        ]
        .into();
        let state = AppState::new();
        Self {
            title,
            actions,
            state,
        }
    }

    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);

            match action {
                Action::GotoNextTab => self.state.goto_next_tab(),
                Action::GotoPrevTab => self.state.goto_prev_tab(),
                Action::SelectNextAlgo => self.state.select_next_algo(),
                Action::SelectPrevAlgo => self.state.select_prev_algo(),
                Action::RunMazeGeneration => self.state.start_maze_generation(),
                Action::Quit => return AppReturn::Exit,
            };

            AppReturn::Continue
        } else {
            warn!("No action accociated to {}", key);
            AppReturn::Continue
        }
    }

    pub fn update_on_tick(&mut self) -> AppReturn {
        self.state.on_tick();
        AppReturn::Continue
    }
}
