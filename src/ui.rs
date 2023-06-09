use std::{io, thread, time::Duration};

use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
  backend::{Backend, CrosstermBackend},
  layout::{Constraint, Direction, Layout},
  widgets::{Block, Borders, Widget},
  Frame, Terminal,
};

fn setup() -> Result<(), io::Error> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  terminal.draw(|f| {
    ui(f);
    //        let size = f.size();
    //        let block = Block::default()
    //            .title("kalorie")
    //            .borders(Borders::ALL);
    //        f.render_widget(block, size);
  })?;

  // hold tui open for 5 seconds
  //    thread::sleep(Duration::from_millis(5000));

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

fn ui<B: Backend>(f: &mut Frame<B>) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
      [
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
      ]
      .as_ref(),
    )
    .split(f.size());
  let block = Block::default().title("kalorie").borders(Borders::ALL);
  f.render_widget(block, chunks[0]);
  let block = Block::default()
    .title("kalorie 2: electric boogaloo")
    .borders(Borders::ALL);
  f.render_widget(block, chunks[1]);
}
