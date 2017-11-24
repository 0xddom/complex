use complex::Complex;
use std::io::{Write, stdout};
use state::{AppState,State};
use command::{Command};

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
    {
        let _ = stdout().flush();
    }
}

fn common_cmds(current_num: Complex,
               cmd: Command,
               err_msg: String,
               state: State,
               op: Option<Command>)
               -> Result<AppState, (AppState, String)> {
    match cmd {
        Command::Clear => Ok(AppState::initial_state()),
        Command::Help => {
            print_help();
            Ok(AppState::new(state, current_num, op))
        }
        Command::Real => {
            println!("{}", current_num.real());
            Ok(AppState::new(state, current_num, op))
        }
        Command::Imaginary => {
            println!("{}", current_num.imaginary());
            Ok(AppState::new(state, current_num, op))
        }
        _ => Err((AppState::new(state, current_num, op), (err_msg))),
    }
}

fn eval_cmd_in_number_state(current_num: Complex,
                            cmd: Command,
                            op: Option<Command>)
                            -> Result<AppState, (AppState, String)> {
    match cmd {
        Command::Number(num) => match op {
            Some(Command::Addition) => {
                let new_num = current_num + num;
                println!("{}", new_num);
                Ok(AppState::new(State::Operation, new_num, None))
            },
            Some(Command::Subtraction) =>  {
                let new_num = current_num - num;
                println!("{}", new_num);
                Ok(AppState::new(State::Operation, new_num, None))
            },
            Some(Command::Multiplication) =>  {
                let new_num = current_num * num;
                println!("{}", new_num);
                Ok(AppState::new(State::Operation, new_num, None))
            },
            _ => Ok(AppState::new(State::Operation, num, None))
        },
        c => common_cmds(current_num, c, "Expecting a number".into(), State::Number, op),
    }
}

fn eval_cmd_in_command_state(current_num: Complex,
                             cmd: Command,
                             op: Option<Command>)
                             -> Result<AppState, (AppState, String)> {
    match cmd {
        Command::Addition => Ok(AppState::new(State::Number, current_num, Some(Command::Addition))),
        Command::Subtraction => Ok(AppState::new(State::Number, current_num, Some(Command::Subtraction))),
        Command::Multiplication => Ok(AppState::new(State::Number, current_num, Some(Command::Multiplication))),
        c => {
            common_cmds(current_num,
                        c,
                        "Expecting a command".into(),
                        State::Operation, op)
        }
    }
}

pub fn eval_cmd(app_state: AppState, cmd: Command) -> Result<AppState, (AppState, String)> {
    match app_state {
        AppState { current_state: State::Number, number: current_num, pending_op: op } => {
            eval_cmd_in_number_state(current_num, cmd, op)
        }
        AppState { current_state: State::Operation, number: current_num, pending_op: op } => {
            eval_cmd_in_command_state(current_num, cmd, op)
        }
    }
}
