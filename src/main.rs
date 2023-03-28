mod tree;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{Print, StyledContent},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    Result,
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

fn main() -> Result<()> {
    // Setup
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    execute!(stdout(), Hide)?;
    // Setup Complete

    ui_loop()?;
    thread::sleep(Duration::from_secs(1));

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
    let mut t = tree::Tree::new(scr.x_max as i16, scr.y_max as i16); // We should really stop using
                                                                     // floats now.

    while !t.is_dead() {
        t.grow();
        for (x, y, s) in t.observe() {
            scr.draw_str(x as i16, y as i16, s);
            thread::sleep(Duration::from_millis(20));
            stdout().flush()?;
        }
    }
    thread::sleep(Duration::from_secs(1));

    Ok(())
}
