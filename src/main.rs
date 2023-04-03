mod tree;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Print, StyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    Result,
};
use std::{
    io::{stdout, Write},
    thread::{self, sleep},
    time::Duration,
};

fn main() -> Result<()> {
    // Setup
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    execute!(stdout(), Hide)?;
    // Setup Complete

    ui_loop()?;

    // Clean up
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, Show)?;

    Ok(())
}

struct Screen {
    pub x_max: u16,
    pub y_max: u16,
}
impl Screen {
    pub fn new() -> Self {
        let (x, y) = size().unwrap();
        Screen { x_max: x, y_max: y }
    }
    pub fn draw_str(&self, x: i16, y: i16, s: StyledContent<&str>) {
        let x_adj = x + self.x_max as i16 / 2;
        let y_adj = self.y_max as i16 - y;
        if x_adj as u16 > self.x_max {
            return ();
        };
        if y_adj as u16 > self.y_max {
            return ();
        };

        queue!(stdout(), MoveTo(x_adj as u16, y_adj as u16), Print(s));
    }
}

fn ui_loop() -> Result<()> {
    let scr = Screen::new();
    let mut options = (false, );
    print_pot(&scr)?;
    stdout().flush()?;

    tree_loop(&scr, &mut options)?;

    loop {
        if crossterm::event::poll(Duration::from_millis(50))? {
            // Wait for close
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('d') => options.0 ^= true,
                    _ => (),
                }
            }
        }
    }
}

fn tree_loop(scr: &Screen, options: &mut (bool, ) ) -> Result<()> {
    let mut t = tree::Tree::new(scr.x_max as i16, scr.y_max as i16);
    let mut override_counter: u16 = 0;

    while override_counter < 150 && !t.is_dead() {
        override_counter += 1;
        t.grow();
        for (x, y, s) in t.observe() {
            scr.draw_str(x as i16, y as i16, s);
            if crossterm::event::poll(Duration::from_millis(10))? {
                // Allow user termination
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('d') => options.0 ^= true,
                        _ => (),
                    }
                }
            }
            stdout().flush()?;
        }
        if options.0 {
            scr.draw_str(
                0,
                (scr.y_max as i16) - 9,
                (override_counter.to_string().as_str()).red(),
            );
            scr.draw_str(
                0,
                (scr.y_max as i16) - 8,
                (t.age.to_string().as_str()).red(),
            );
        }
    }
    scr.draw_str(0, (scr.y_max as i16) - 10, "shrimp".green());
    stdout().flush()?;
    return Ok(());
}

fn print_pot(scr: &Screen) -> Result<()> {
    const TRUNK2: &str = "./~~~\\.";
    const GRASS2: &str = "~~~~~~~~~~~~~~~~~~~~~~~~~~~";
    const LAYER2: &str = " \\                           / ";
    const LAYER1: &str = "  \\_________________________/ ";
    const LAYER0: &str = "  (=)                     (=)";
    let x = -(LAYER0.len() as i16) / 2;

    scr.draw_str(x, 1, LAYER0.white());
    scr.draw_str(x, 2, LAYER1.white());
    scr.draw_str(x, 3, LAYER2.white());
    scr.draw_str(x + 2, 3, GRASS2.green());
    scr.draw_str(x + 12, 3, TRUNK2.yellow());

    scr.draw_str(0, (scr.y_max as i16) - 10, "shrimp".red());

    Ok(())
}
