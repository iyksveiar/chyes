/* TUI library */
extern crate termion;
// use termion::raw::IntoRawMode;
use std::io::{Write, Read, Error};

/* All functions are taken from ncurses library */
pub fn clearscr(stdout: &mut std::io::Stdout) -> Result<(), Error> {
    write!(stdout, "{}", termion::clear::All)
}

pub fn mv(y: u16, x: u16, stdout: &mut std::io::Stdout) -> Result<(), Error> {
    write!(stdout, "{}", termion::cursor::Goto(x, y))
}

pub fn printw(s: &str, stdout: &mut std::io::Stdout) -> Result<(), Error> {
    write!(stdout, "{}", s)
}

pub fn mvprintw(y: u16, x: u16, s: &str, stdout: &mut std::io::Stdout) -> Result<(), Error> {
    mv(y, x, stdout)?;
    printw(s, stdout)
}
