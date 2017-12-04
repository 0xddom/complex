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
        Command::Root(n) => do_power(n, state),
        Command::Exit => Ok(state)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use complex::Complex;

    #[test]
    fn test_help_command__065() {
        let ini_state = AppState::default();

        assert_eq!(Ok(AppState::default()), eval_cmd(ini_state, Command::Help));
    }

    #[test]
    fn test_clear_command__066() {
        assert_eq!(Ok(AppState::default()), eval_cmd(AppState::new(None, None, true), Command::Clear));
    }

    #[test]
    fn test_print_real__067__068() {
        {
            let ini_state = AppState::new(Some(Complex::new(1.0, 0.0)), None, true);
            let expected = Ok(AppState::new(Some(Complex::new(1.0, 0.0)), None, false));

            assert_eq!(expected, eval_cmd(ini_state, Command::Real));
        }
        {
            let ini_state = AppState::new(None, None, true);
            let expected = Err((AppState::new(None, None, true), "You must insert a number first".into()));

            assert_eq!(expected, eval_cmd(ini_state, Command::Real));
        }
    }

    #[test]
    fn test_print_imaginary__069__070() {
        {
            let ini_state = AppState::new(Some(Complex::new(1.0, 0.0)), None, true);
            let expected = Ok(AppState::new(Some(Complex::new(1.0, 0.0)), None, false));

            assert_eq!(expected, eval_cmd(ini_state, Command::Imaginary));
        }
        {
            let ini_state = AppState::new(None, None, true);
            let expected = Err((AppState::new(None, None, true), "You must insert a number first".into()));

            assert_eq!(expected, eval_cmd(ini_state, Command::Imaginary));
        }
    }

    #[test]
    fn test_add_action__071__072__073__074() {
        {
            let input_state = AppState::new(Some(Complex::new(1.0, 0.0)), None, true);
            let expected = Ok(AppState::new(Some(Complex::new(1.0, 0.0)), Some(Command::Addition), true));

            let output = eval_cmd(input_state, Command::Addition);

            assert_eq!(expected, output);
        }
        {
            let input_state = AppState::new(None, None, true);
            let expected = Err((AppState::new(None, None, true), "You must insert a number first".into()));

            let output = eval_cmd(input_state, Command::Addition);

            assert_eq!(expected, output);
        }
        {
            let input_state = AppState::new(None, None, true);
            let expected = Err((AppState::new(None, None, true), "You must insert a number first".into()));

            let output = eval_cmd(input_state, Command::Subtraction);

            assert_eq!(expected, output);
        }
        {
            let input_state = AppState::new(None, None, true);
            let expected = Err((AppState::new(None, None, true), "You must insert a number first".into()));

            let output = eval_cmd(input_state, Command::Multiplication);

            assert_eq!(expected, output);
        }
    }

    #[test]
    fn test_number__075() {
        let input_num = Complex::new(1.0, 0.0);
        let input_state = AppState::new(None, None, false);
        let expected = Ok(AppState::new(Some(Complex::new(1.0, 0.0)), None, true));

        assert_eq!(expected, eval_cmd(input_state, Command::Number(input_num)));
    }

    #[test]
    fn test_power__076() {
        let input_num = 2.0;
        let input_state = AppState::new(Some(Complex::new(2.0, 2.0)), None, false);
        let expected = Ok(AppState::new(Some(Complex::new(0.0000000000000004898587196589414, 8.000000000000002)), None, true));
        
        assert_eq!(expected, eval_cmd(input_state, Command::Power(input_num)));
    }

    #[test]
    fn test_root__077() {
        let input_num = 2.0;
        let input_state = AppState::new(Some(Complex::new(2.0, 2.0)), None, false);
        let expected = Ok(AppState::new(Some(Complex::new(1.5537739740300374, 0.6435942529055827)), None, true));
        
        assert_eq!(expected, eval_cmd(input_state, Command::Root(input_num)));
    }

    #[test]
    fn test_exit__078() {
        let input_state = AppState::default();
        let expected = Ok(AppState::default());

        assert_eq!(expected, eval_cmd(input_state, Command::Exit));
    }
}
