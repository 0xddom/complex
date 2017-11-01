use complex::Complex;
use std::cmp::PartialEq;
use std::io::{Write,stdout};
use std::str::SplitWhitespace;

#[derive(Copy)]
pub enum State {
    Number,
    Operation
}

#[derive(PartialEq,Eq)]
pub enum Command {
    Help,
    Exit,
    Clear,
    Addition,
    Subtraction,
    Multiplication,
    Real,
    Imaginary,
    Power(i32),
    Root(i32),
    Number(Complex),
}

pub struct AppState {
    current_state: State,
    number: Complex
}

impl AppState {
    pub fn initial_state() -> AppState {
        AppState::new_with_state(State::Number)
    }

    pub fn new_with_state(state: State) -> AppState {
        AppState { current_state: state, number: Complex::new(0.0,0.0) }
    }

    pub fn state(self) -> State {
        self.current_state
    }

    pub fn number(self) -> Complex {
        self.number
    }
}

impl Copy for AppState {
}

fn parse_command(head: &str, rest: SplitWhitespace) -> Result<Command, String> {
    match head {
        "help" => Ok(Command::Help),
        "clear" => Ok(Command::Clear),
        "exit" => Ok(Command::Exit),
        "addition" => Ok(Command::Addition),
        "subtraction" => Ok(Command::Subtraction),
        "multiplication" => Ok(Command::Multiplication),
        "real" => Ok(Command::Real),
        "imaginary" => Ok(Command::Imaginary),
        "power" => {
            Err("TODO".into())
        },
        "root" => {
            Err("TODO".into())
        },
        _ => {
            Err("Unknown command".into())
        }
    }
}

pub fn read_command() -> Result<Command, String> {
    print!(">>> ");
    stdout().flush();
    let input_str: String = read!("{}\n");
    let mut iter = input_str.split_whitespace();
    match iter.next() {
        None => Err("Please enter a command".into()),
        Some(s) => parse_command(s, iter)
    }
}

pub fn eval_cmd(app_state: AppState, cmd: Command) -> Result<AppState,String> {
    match app_state.state() {
        State::Number => {
            Ok(app_state)
        },
        State::Operation => {
            Ok(app_state)
        }
    }
}
