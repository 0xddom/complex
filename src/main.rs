#![feature(use_extern_macros)]

#[macro_use]
extern crate text_io;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod complex;
mod repl;

use repl::{read_command, Command, AppState, eval_cmd};

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
