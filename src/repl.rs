use complex::Complex;
use state::{AppState};
use command::{Command};
use actions::*;

pub fn eval_cmd(state: AppState, cmd: Command) -> Result<AppState, (AppState, String)> {
    match cmd {
        Command::Clear => clear(),
        Command::Help => print_help(state),
        Command::Real => print_real(state),
        Command::Imaginary => print_imaginary(state),
        action @ Command::Subtraction |
        action @ Command::Addition |
        action @ Command::Multiplication => add_action(state, action),
        Command::Number(num) => add_number(num, state),
        Command::Power(_) => Err((state, "eval_cmd::power::TODO".into())),
        Command::Root(_) => Err((state, "eval_cmd::root::TODO".into())),
        Command::Exit => Ok(state)
    }
}
