#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct GPIOState {
    pull_up: bool,
    value: u8,
}

impl GPIOState {
    pub fn new() -> GPIOState {
        GPIOState {
            pull_up: false,
            value: 0,
        }
    }
}
