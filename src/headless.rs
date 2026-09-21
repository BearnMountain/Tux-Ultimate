
/// No gui, therefore only for running servers
pub struct Headless {
    tick: u64,
}

impl Headless {
    pub fn init() -> Self {
        return Self {
            tick: 0,
        };
    }
}
