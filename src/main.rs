#![feature(use_extern_macros)]

#[macro_use]
extern crate text_io;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod complex;
mod repl;
mod state;
mod command;
mod actions;

use repl::{eval_cmd};
use state::AppState;
use command::{Command, read_command};
use std::io::prelude::*;
use std::fs::{File};
use std::path::Path;

fn log_number(state: AppState, mut file: &File) -> AppState {
    match state {
        AppState { number: Some(num), pending_op: op, log: true } => {
            let _ = file.write_all(format!("{}\n", num).as_bytes());
            AppState::new(Some(num), op, false)
        },
        state => state
    }
}

fn main_loop(state: AppState, mut file: &File) {
    let ns = log_number(state, &mut file);
    match read_command() {
        Err(msg) => {
            println!("{}", msg);
            main_loop(ns, &mut file)
        }
        Ok(Command::Exit) => return,
        Ok(cmd) => match eval_cmd(ns, cmd) {
            Ok(s) => main_loop(s, &mut file),
            Err((s, msg)) => {
                println!("{}", msg);
                main_loop(s, &mut file)
            }
        }
    }
}

fn main() {
    let state = AppState::default();
    let path = Path::new("results.txt");
    let mut results_file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => panic!("File error: {}", e)
    };
    main_loop(state, &mut results_file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main__064() {
        main();
    }
}
