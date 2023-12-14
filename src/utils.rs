use std::{process::exit, io::{Stdout, stdout}};

use ratatui::{Terminal, backend::CrosstermBackend};
use crossterm::{ExecutableCommand, terminal::{EnterAlternateScreen, enable_raw_mode, disable_raw_mode, LeaveAlternateScreen}};

pub fn errndie(msg: &str, st_code: i32) -> () {
    eprintln!("{msg}");
    exit(st_code);
}

pub fn start() -> Result<Terminal<CrosstermBackend<Stdout>>, ()> {
    stdout().execute(EnterAlternateScreen).map_err(|_| {
        errndie("Could not access alternate buffer!", 1);
    })?;
    enable_raw_mode().map_err(|_| {
        errndie("Could not enable raw mode!", 1);
    })?;

    let terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout())).map_err(|_| {
        errndie("Could not get access to terminal!", 1);
    })?;

    Ok(terminal)
}

pub fn shutdown() -> Result<(), ()> {
    disable_raw_mode().map_err(|_| {
        errndie("Could not disable raw mode!", 1);
    })?;
    stdout().execute(LeaveAlternateScreen).map_err(|_| {
        errndie("Could not leave alternate buffer", 1);
    })?;
    Ok(())
}
