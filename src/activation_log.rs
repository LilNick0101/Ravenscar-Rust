use rtic_time::Monotonic;

use crate::auxiliary::TimeInstant;
use crate::Mono;

pub struct ActivationLog{
    activation_counter : u8,
    activation_time : TimeInstant
}

impl ActivationLog{
    pub fn new() -> Self {
        ActivationLog{
            activation_counter : 0,
            activation_time : Mono::now()
        }
    }

    pub fn write(&mut self,activation_time : TimeInstant){
        self.activation_time = activation_time;
        self.activation_counter = (self.activation_counter + 1) % 100;
    }

    pub fn read(&self) -> (u8,TimeInstant){
        (self.activation_counter,self.activation_time)
    }
}