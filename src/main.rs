#![feature(use_extern_macros)]

#[macro_use] extern crate text_io;

mod complex;
mod repl;

use complex::Complex;
use repl::{read_command,Command, AppState,eval_cmd};

fn main() {
    let mut state = AppState::initial_state();
    loop {
        match read_command() {
            Err(msg) => println!("{}", msg),
            Ok(Command::Exit) => break,
            Ok(cmd) => {
                match eval_cmd(state, cmd) {
                    Ok(s) => state = s,
                    Err(msg) => panic!("{}", msg)
                }
            }
        }
    }
}

