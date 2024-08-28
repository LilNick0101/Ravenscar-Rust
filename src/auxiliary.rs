use auxiliary_params::FACTOR;
use rtic_monotonics::fugit::Instant;

mod auxiliary_params{
    pub const FACTOR : u8 = 3;
}

pub type TimeInstant = Instant<u32,1,1000>;

pub struct Auxiliary{
    request_counter : u32,
    run_counter : u32,
}

impl Auxiliary {
    pub fn new() -> Self {
        Auxiliary{
            request_counter : 0,
            run_counter : 0,
        }
    }

    pub fn due_activation(&mut self, param : u32) -> bool {
        self.request_counter = (self.request_counter + 1) % 5;

        self.request_counter == (param % 5)
    }

    pub fn check_due(&mut self) -> bool {
        self.run_counter = (self.run_counter + 1) % 1000;
        let divisor = self.run_counter / FACTOR as u32;

        (divisor * FACTOR as u32) == self.run_counter
    }
}