use std::{error::Error, io, time::Duration};

use crossterm::{cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};

fn main() -> Result <(), Box<dyn Error>> {
    //terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Game Loop
    'gameloop: loop {
        //input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                  KeyCode::Esc | KeyCode::Char('q') => {
                    break 'gameloop;
                  }
                  _ => {}
                }
            }
        }

    }

    //cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())


}
