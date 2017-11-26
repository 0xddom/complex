use std::io::{Write, stdout};
use state::{AppState};
use command::Command;
use complex::Complex;

macro_rules! num_fst {
    ($state:expr) => (Err(($state, "You must insert a number first".into())))
}

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
    match state {
        AppState { number: Some(cplx), pending_op: op, log: _ } => {
            println!("{}", cplx.real());
            Ok(AppState::new(Some(cplx), op, false))
        },
        s @ AppState { number: None, pending_op: _, log: _ } => num_fst!(s),
    }
}

pub fn print_imaginary(state: AppState) -> Result<AppState, (AppState, String)> {
    match state {
        AppState { number: Some(cplx), pending_op: op, log: _ } => {
            println!("{}", cplx.imaginary());
            Ok(AppState::new(Some(cplx), op, false))
        },
        s @ AppState { number: None, pending_op: _, log: _ } => num_fst!(s),
    }
}

pub fn add_action(state: AppState, action: Command) -> Result<AppState, (AppState, String)> {
    match state {
        AppState { number: Some(cplx), pending_op: _, log} => {
            Ok(AppState::new(Some(cplx), Some(action), log))
        },
        s @ AppState { number: None, pending_op: _, log: _ } => num_fst!(s),
    }
}

pub fn add_number(num: Complex, state: AppState) -> Result<AppState, (AppState, String)> {
    match state {
        s @ AppState { number: None, pending_op: Some(_), log: _ } => num_fst!(s),
        AppState { number: Some(number), pending_op: Some(Command::Addition), log: _ } => {
            let new_num = number + num;
            println!("{}", new_num);
            Ok(AppState::new(Some(new_num), None, true))
        },
        AppState { number: Some(number), pending_op: Some(Command::Subtraction), log: _ } => {
            let new_num = number - num;
            println!("{}", new_num);
            Ok(AppState::new(Some(new_num), None, true))
        },
        AppState { number: Some(number), pending_op: Some(Command::Multiplication), log: _ } => {
            let new_num = number * num;
            println!("{}", new_num);
            Ok(AppState::new(Some(new_num), None, true))
        },
        AppState { number: _, pending_op: None, log: _ } => Ok(AppState::new(Some(num), None, true)),
        s => Ok(s)
    }
}

pub fn do_power(num: f32, state: AppState) -> Result<AppState, (AppState, String)> {
    match state {
        AppState { number: Some(cplx), pending_op: op, log: _ } =>
            Ok(AppState::new(Some(cplx.power(num)), state.pending_op, true)),
        s @ AppState { None, pending_op: _, log: _ } => num_fst!(s)
    }
}

pub fn do_root(num: f32, state: AppState) -> Result<AppState, (AppState, String)> {
    match state {
        AppState { number: Some(cplx), pending_op: op, log: _ } =>
            Ok(AppState::new(Some(cplx.root(num)), state.pending_op, true)),
        s @ AppState { None, pending_op: _, log: _ } => num_fst!(s)
    }
}
