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

pub fn do_power(num: f64, state: AppState) -> Result<AppState, (AppState, String)> {
    match state {
        AppState { number: Some(cplx), pending_op: op, log: _ } => {
            let new_num = cplx.power(num);
            println!("{}", new_num);
            Ok(AppState::new(Some(new_num), op, true))
        },
        s @ AppState { number: None, pending_op: _, log: _ } => num_fst!(s)
    }
}

pub fn do_root(num: f64, state: AppState) -> Result<AppState, (AppState, String)> {
    match state {
        AppState { number: Some(cplx), pending_op: op, log: _ } => {
            let new_num = cplx.root(num);
            println!("{}", new_num);
            Ok(AppState::new(Some(new_num), op, true))
        },
        s @ AppState { number: None, pending_op: _, log: _ } => num_fst!(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_command__029() {
        let ini_state = AppState::default();

        assert_eq!(Ok(AppState::default()), print_help(ini_state));
    }

    #[test]
    fn test_clear_command__030() {
        assert_eq!(Ok(AppState::default()), clear());
    }

    #[test]
    fn test_print_real__031__032() {
        {
            let ini_state = AppState::new(Some(Complex::new(1.0, 0.0)), None, true);
            let expected = Ok(AppState::new(Some(Complex::new(1.0, 0.0)), None, false));

            assert_eq!(expected, print_real(ini_state));
        }
        {
            let ini_state = AppState::new(None, None, true);
            let expected = Err((AppState::new(None, None, true), "You must insert a number first".into()));

            assert_eq!(expected, print_real(ini_state));
        }
    }

    #[test]
    fn test_print_imaginary__033__034() {
        {
            let ini_state = AppState::new(Some(Complex::new(1.0, 0.0)), None, true);
            let expected = Ok(AppState::new(Some(Complex::new(1.0, 0.0)), None, false));

            assert_eq!(expected, print_imaginary(ini_state));
        }
        {
            let ini_state = AppState::new(None, None, true);
            let expected = Err((AppState::new(None, None, true), "You must insert a number first".into()));

            assert_eq!(expected, print_imaginary(ini_state));
        }
    }

    #[test]
    fn test_add_action__035__036() {
        {
            let input_state = AppState::new(Some(Complex::new(1.0, 0.0)), None, true);
            let expected = Ok(AppState::new(Some(Complex::new(1.0, 0.0)), Some(Command::Addition), true));

            let output = add_action(input_state, Command::Addition);

            assert_eq!(expected, output);
        }
        {
            let input_state = AppState::new(None, None, true);
            let expected = Err((AppState::new(None, None, true), "You must insert a number first".into()));

            let output = add_action(input_state, Command::Addition);

            assert_eq!(expected, output);
        }
    }

    #[test]
    fn test_add_number_action__037__038__039__040__041__042() {
        {
            let input_num = Complex::new(1.0, 1.0);
            let input_state = AppState::new(None, Some(Command::Addition), false);
            let expected = Err((AppState::new(None, Some(Command::Addition), false), "You must insert a number first".into()));

            assert_eq!(expected, add_number(input_num, input_state));
        }
        {
            let input_num = Complex::new(1.0, 1.0);
            let input_state = AppState::new(Some(Complex::new(0.0, 0.0)), Some(Command::Addition), false);
            let expected = Ok(AppState::new(Some(Complex::new(1.0, 1.0)), None, true));

            assert_eq!(expected, add_number(input_num, input_state));
        }
        {
            let input_num = Complex::new(1.0, 1.0);
            let input_state = AppState::new(Some(Complex::new(1.0, 1.0)), Some(Command::Subtraction), false);
            let expected = Ok(AppState::new(Some(Complex::new(0.0, 0.0)), None, true));

            assert_eq!(expected, add_number(input_num, input_state));
        }
        {
            let input_num = Complex::new(1.0, 1.0);
            let input_state = AppState::new(Some(Complex::new(1.0, 1.0)), Some(Command::Multiplication), false);
            let expected = Ok(AppState::new(Some(Complex::new(0.0, 0.0)), None, true));

            assert_eq!(expected, add_number(input_num, input_state));
        }
        {
            let input_num = Complex::new(1.0, 0.0);
            let input_state = AppState::new(None, None, false);
            let expected = Ok(AppState::new(Some(Complex::new(1.0, 0.0)), None, true));

            assert_eq!(expected, add_number(input_num, input_state));
        }
        {
            let input_num = Complex::new(1.0, 0.0);
            let input_state = AppState::new(Some(Complex::new(0.0, 0.0)), Some(Command::Help), false);
            let expected = Ok(AppState::new(Some(Complex::new(0.0, 0.0)), Some(Command::Help), false));

            assert_eq!(expected, add_number(input_num, input_state));
        }
    }

    #[test]
    fn test_power__043__044() {
        {
            let input_num = 2.0;
            let input_state = AppState::new(Some(Complex::new(2.0, 2.0)), None, false);
            let expected = Ok(AppState::new(Some(Complex::new(0.0000000000000004898587196589414, 8.000000000000002)), None, true));

            assert_eq!(expected, do_power(input_num, input_state));
        }
        {
            let input_num = 2.0;
            let input_state = AppState::new(None, None, false);
            let expected = Err((AppState::new(None, None, false), "You must insert a number first".into()));

            assert_eq!(expected, do_power(input_num, input_state));
        }
    }

    #[test]
    fn test_root__045__046() {
        {
            let input_num = 2.0;
            let input_state = AppState::new(Some(Complex::new(2.0, 2.0)), None, false);
            let expected = Ok(AppState::new(Some(Complex::new(1.5537739740300374, 0.6435942529055827)), None, true));

            assert_eq!(expected, do_root(input_num, input_state));
        }
        {
            let input_num = 2.0;
            let input_state = AppState::new(None, None, false);
            let expected = Err((AppState::new(None, None, false), "You must insert a number first".into()));

            assert_eq!(expected, do_root(input_num, input_state));
        }
    }
}
