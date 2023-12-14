use std::{env, fs::{File, self}};
mod event;
mod utils;
mod ui;
mod app;

fn main() -> Result<(), ()> {

    let args = env::args().collect::<Vec<String>>();
    let mut app: app::App;
    if args.len() == 1 {
        app = app::App::new();
    } else {
        let filename = &args[1];
        let mut file = match fs::File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                fs::File::create(filename).expect("What the fuck!")
            }
        };
        app = app::App::from_file(&mut file);
    }

    let mut terminal = utils::start()?;

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
