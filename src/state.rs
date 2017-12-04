use complex::Complex;
use command::Command;

#[derive(Debug, PartialEq)]
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

