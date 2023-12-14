mod event;
mod utils;
mod ui;
mod app;

fn main() -> Result<(), ()> {

    let mut terminal = utils::start()?;
    let mut app = app::App::new();

    loop {
        terminal.draw(|f| {
            ui::ui(&app, f);
        }).map_err(|_| ())?;

        event::update(&mut app);

        if app.quit {
            break;
        }
    }
    utils::shutdown()?;
    Ok(())
}
