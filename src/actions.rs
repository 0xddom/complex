use std::io::{Write, stdout};
use state::{AppState};
use command::Command;
use complex::Complex;

pub fn print_help(state: AppState) -> Result<AppState, (AppState, String)> {
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
    Ok(state)
}

pub fn clear() -> Result<AppState, (AppState, String)> {
    Ok(AppState::default())
}

pub fn print_real(state: AppState) -> Result<AppState, (AppState, String)> {
    match state.number {
        Some(cplx) => {
            println!("{}", cplx.real());
            Ok(AppState::new(Some(cplx), state.pending_op))
        },
        None => Err((AppState::new(None, state.pending_op), "You must insert a number first".into()))
    }
}

pub fn print_imaginary(state: AppState) -> Result<AppState, (AppState, String)> {
    match state.number {
        Some(cplx) => {
            println!("{}", cplx.imaginary());
            Ok(AppState::new(Some(cplx), state.pending_op))
        },
        None => Err((AppState::new(None, state.pending_op), "You must insert a number first".into()))
    }
}

pub fn add_action(state: AppState, action: Command) -> Result<AppState, (AppState, String)> {
    match state.number {
        Some(num) => Ok(AppState::new(Some(num), Some(action))),
        None => Err((state, "You must insert a number first".into()))
    }
}

pub fn add_number(num: Complex, state: AppState) -> Result<AppState, (AppState, String)> {
    match state.pending_op {
        Some(Command::Addition) => match state.number {
            Some(number) => {
                let new_num = number + num;
                println!("{}", new_num);
                Ok(AppState::new(Some(new_num), None))
            },
            None => Err((AppState::new(None, None), "You must insert a number first".into()))
        },
        Some(Command::Subtraction) => match state.number {
            Some(number) => {
                let new_num = number - num;
                println!("{}", new_num);
                Ok(AppState::new(Some(new_num), None))
            },
            None => Err((AppState::new(None, None), "You must insert a number first".into()))
        },
        Some(Command::Multiplication) => match state.number {
            Some(number) => {
                let new_num = number * num;
                println!("{}", new_num);
                Ok(AppState::new(Some(new_num), None))
            },
            None => Err((AppState::new(None, None), "You must insert a number first".into()))
        },
        _ => Ok(AppState::new(Some(num), None))
    }
}
