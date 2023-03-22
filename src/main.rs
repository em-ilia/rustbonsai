mod tree;
use std::{
    io,
    time::{Duration, Instant}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

fn main() -> () {
    println!("Branch creation debugging!");
    let mut t = tree::Tree::default();
    for i in 1..=40 {
        println!("{:?}", t.observe());
        t.grow();
    }
}