#![feature(use_extern_macros)]
#![feature(try_trait)]

#[macro_use]
extern crate text_io;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod complex;
mod repl;
mod state;
mod command;

use repl::{eval_cmd};
use state::AppState;
use command::{Command, read_command};

fn main_loop(state: AppState) {
    match read_command() {
        Err(msg) => {
            println!("{}", msg);
            main_loop(state)
        }
        Ok(Command::Exit) => return,
        Ok(cmd) => {
            match eval_cmd(state, cmd) {
                Ok(s) => main_loop(s),
                Err((s, msg)) => {
                    println!("{}", msg);
                    println!("ERR");
                    main_loop(s)
                }
            }
        }
    }
}

fn main() {
    let state = AppState::initial_state();
    main_loop(state);
}
