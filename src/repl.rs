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
        Command::Power(n) => do_power(n, state),
        Command::Root(n) => do_root(n, state),
        Command::Exit => Ok(state)
    }
}
