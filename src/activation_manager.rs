use activation_manager_params::RELATIVE_OFFSET;
use fugit::ExtU32;
use rtic_monotonics::stm32::prelude::*;

use crate::{auxiliary::TimeInstant, Mono};

mod activation_manager_params{
    pub const RELATIVE_OFFSET : u32 = 100;
}

pub struct ActivationManager{
    activation_time : TimeInstant,
}

impl ActivationManager{
    pub fn new() -> Self {
        ActivationManager{
            activation_time: Mono::now() + RELATIVE_OFFSET.millis(),
        }
    }

    pub fn get_activation_time(&self) -> TimeInstant {
        self.activation_time
    }
}