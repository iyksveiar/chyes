use std::{io};

use chyes::Board;
use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use tui::{
  backend::CrosstermBackend,
  layout::{Margin, Rect},
  widgets::{Block, Borders},
  Terminal
};

fn main() -> Result<(), io::Error> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  loop {
    terminal.draw(|f| {
      let board = Board::default();
      let size = f.size();
      let block = Block::default().title("Chyes").borders(Borders::ALL);
      f.render_widget(block, size);
      f.render_widget(
        board,
        size.inner(&Margin {
          horizontal: 10,
          vertical:   10
        })
      );
    })?;

    match event::read()? {
      Event::Key(event) => match event.code {
        KeyCode::Char('q') => break,
        _ => {}
      },
      Event::Mouse(_) => (),
      Event::Resize(width, height) => {
        {
          terminal.resize(Rect::new(0, 0, width, height))?;
        };
      }
    }
  }

  // restore terminal
  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  Ok(())
}
