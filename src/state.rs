use complex::Complex;
use command::Command;

pub struct AppState {
    pub number: Option<Complex>,
    pub pending_op: Option<Command>
}

impl AppState {
    pub fn new(complex: Option<Complex>, pending_op: Option<Command>) -> AppState {
        AppState {
            number: complex,
            pending_op: pending_op
        }
    }

    pub fn default() -> AppState {
        AppState::new(None, None)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_appstate_create_func() {
        let input_state = State::Number;
        let input_complex = Complex::zero();
        let input_op = None;
        let expected = AppState { current_state: State::Number, number: Complex::zero(), pending_op: None };

        let output = AppState::new(input_state, input_complex, input_op);

        assert_eq!(expected, output);
    }
}
