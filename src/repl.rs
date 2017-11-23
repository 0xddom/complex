use complex::Complex;
use std::cmp::PartialEq;
use std::io::{Write,stdout};
use std::str::SplitWhitespace;

pub enum State {
    Number,
    Operation
}

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
    pub fn new(state: State, complex: Complex) -> AppState {
        AppState { current_state: state, number: complex }
    }
    
    pub fn initial_state() -> AppState {
        AppState::new_with_state(State::Number)
    }

    pub fn new_with_state(state: State) -> AppState {
        AppState::new(state, Complex::new(0.0,0.0))
    }

    pub fn state(self) -> State {
        self.current_state
    }

    pub fn number(self) -> Complex {
        self.number
    }
}

fn parse_command(head: &str, tail: SplitWhitespace) -> Result<Command, String> {
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
        s => {
            Err(format!("Unknown command: {}", s))
        }
    }
}

pub fn read_command() -> Result<Command, String> {
    print!(">>> ");
    {stdout().flush();}
    let input_str: String = read!("{}\n");
    let mut iter = input_str.split_whitespace();
    match iter.next() {
        None => Err("Please enter a command".into()),
        Some(s) => parse_command(s, iter)
    }
}

fn print_help() {
    println!("help - Show all available operations and commands");
    println!("exit - Quits the program");
    println!("clear - Calculator is restarted");
    println!("addition - Performs addition to the current number");
    println!("subtraction - Performs subtraction to the current number");
    println!("multiplication - Performs multiplication to the current number");
    println!("real - Returns the real part of the complex number");
    println!("imaginary - Returns the imaginary part of the complex number");
    println!("power - Calculate the power. Usage: power <arg>");
    println!("root - Calculate the root. Usage: root <arg>");
    {stdout().flush();}
}

fn common_cmds(current_num: Complex, cmd: Command, err_msg: String, state: State) -> Result<AppState, (AppState, String)> {
    match cmd {
        Command::Clear => Ok(AppState::initial_state()),
        Command::Help => {
            print_help();
            Ok(AppState::new(State::Number, current_num))
        },
        Command::Real => {
            println!("{}", current_num.real());
            Ok(AppState::new(State::Operation, current_num))
        },
        Command::Imaginary => {
            println!("{}", current_num.imaginary());
            Ok(AppState::new(State::Operation, current_num))
        },
        _ => Err((AppState::new(state, current_num), (err_msg)))
    }
}

fn eval_cmd_in_number_state(current_num: Complex, cmd: Command) -> Result<AppState, (AppState, String)> {
    match cmd {
        Command::Number(num) => Ok(AppState::new(State::Operation, num)),
        c => common_cmds(current_num, c, "Expecting a number".into(), State::Number)
    }
}

fn eval_cmd_in_command_state(current_num: Complex, cmd: Command) -> Result<AppState, (AppState, String)> {
    match cmd {
        c => common_cmds(current_num, c, "Expecting a command".into(), State::Operation)
    }
}

pub fn eval_cmd(app_state: AppState, cmd: Command) -> Result<AppState, (AppState, String)> {
    match app_state {
        AppState {
            current_state: State::Number,
            number: current_num
        } => eval_cmd_in_number_state(current_num, cmd),
        AppState {
            current_state: State::Operation,
            number: current_num
        } => eval_cmd_in_command_state(current_num, cmd)
    }
}
