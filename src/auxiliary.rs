use auxiliary_params::FACTOR;
use rtic_monotonics::fugit::Instant;

mod auxiliary_params{
    pub const FACTOR : u16 = 3;
}

pub type TimeInstant = Instant<u64,1,1_000_000>;

pub struct Auxiliary{
    request_counter : u8,
    run_counter : u16,
}

impl Auxiliary {
    pub fn new() -> Self {
        Auxiliary{
            request_counter : 0,
            run_counter : 0,
        }
    }

    pub fn due_activation(&mut self, param : u8) -> bool {
        self.request_counter = (self.request_counter + 1) % 5;

        self.request_counter == (param % 5)
    }

    pub fn check_due(&mut self) -> bool {
        self.run_counter = (self.run_counter + 1) % 1000;
        let divisor = self.run_counter / FACTOR;

        (divisor * FACTOR) == self.run_counter
    }
}