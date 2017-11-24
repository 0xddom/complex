use complex::Complex;
use command::Command;

pub enum State {
    Number,
    Operation,
}

pub struct AppState {
    pub current_state: State,
    pub number: Complex,
    pub pending_op: Option<Command>
}

impl AppState {
    pub fn new(state: State, complex: Complex, pending_op: Option<Command>) -> AppState {
        AppState {
            current_state: state,
            number: complex,
            pending_op: pending_op
        }
    }

    pub fn initial_state() -> AppState {
        AppState::new_with_state(State::Number)
    }

    pub fn new_with_state(state: State) -> AppState {
        AppState::new(state, Complex::zero(), None)
    }

    pub fn state(self) -> State {
        self.current_state
    }

    pub fn number(self) -> Complex {
        self.number
    }
}
