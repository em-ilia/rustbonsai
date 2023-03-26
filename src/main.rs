mod tree;
use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant},
    thread
};
use crossterm::{
    execute,
    event,
    queue,
    Result,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, size},
    cursor::{Hide, Show, MoveTo},
    style::Print
};

fn _main() -> () {
    println!("Branch creation debugging!");
    let mut t = tree::Tree::default();
    for _ in 1..=40 {
        println!("{:?}", t.observe());
        t.grow();
    }
}

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
    x_max: u16,
    y_max: u16,
    x0: u16,
    y0: u16
}
impl Screen {
    pub fn new() -> Self {
        let (x,y) = size().unwrap();
        Screen { x_max: x, y_max: y, x0: 60, y0: 20 }
    }
    pub fn draw_str(&self, x: i16, y: i16, s: &str) {
        let x_adj = x + self.x_max as i16/2 ;
        let y_adj = y + self.y_max as i16/2 ;
        if x_adj as u16 > self.x_max {return ()};
        if y_adj as u16 > self.y_max {return ()};

        queue!(stdout(), MoveTo(x as u16, y as u16), Print(s));
    }
}


fn ui_loop() -> Result<()> {
    let mut t = tree::Tree::default();
    let scr = Screen::new();

    for _ in 1..=30 {
        t.grow();
        for (x,y,s) in t.observe() {
            scr.draw_str(x as i16, y as i16, s);
        }
        stdout().flush();
    }
    thread::sleep(Duration::from_secs(1));

    Ok(())
}