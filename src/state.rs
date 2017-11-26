use complex::Complex;
use command::Command;

pub struct AppState {
    pub number: Option<Complex>,
    pub pending_op: Option<Command>,
    pub log: bool
}

impl AppState {
    pub fn new(complex: Option<Complex>, pending_op: Option<Command>, log: bool) -> AppState {
        AppState {
            number: complex,
            pending_op: pending_op,
            log: log
        }
    }

    pub fn default() -> AppState {
        AppState::new(None, None, false)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_appstate_create_func() {
        let input_complex = Complex::zero();
        let input_op = None;
        let expected = AppState { number: Complex::zero(), pending_op: None };

        let output = AppState::new(input_complex, input_op);

        assert_eq!(expected, output);
    }
}
