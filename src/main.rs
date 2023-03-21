use std::{
    io,
    time::{Duration, Instant}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    layout::{Layout, Rect},
    widgets::{
        Block, BorderType, Borders,
        canvas::{Canvas, Line, Rectangle}
    },
    Frame, Terminal};

struct Trunk {
    x_range: f64,
    y_range: f64,
    child: Tree
}
enum Tree {
    Branch(Branch),
    Leaf(Leaf)
}
struct Branch {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    child: Box<Tree>

}
struct Leaf {
    x: f64,
    y: f64
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>) {
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .x_bounds([-10.0,10.0])
        .y_bounds([0.0,20.0])
        .paint(|ctx| {
            ctx.print(0.0,5.0,"shrimp");
        });
    frame.render_widget(canvas, frame.size());
}