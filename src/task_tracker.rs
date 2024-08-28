use crate::auxiliary::TimeInstant;
use rtic_time::Monotonic;

use crate::Mono;

//tracks the time that a task takes
pub struct TaskTracker{
    start_value : Option<TimeInstant>,
    end_value : Option<TimeInstant>
}

impl TaskTracker{
    pub fn new() -> Self{
        TaskTracker{
            start_value : None,
            end_value : None,
        }
    }

    pub fn start_tracking(&mut self){
        self.start_value = Some(Mono::now());
        self.end_value = None;
    }

    pub fn end_tracking(&mut self, name : &'static str) {
        self.end_value = Some(Mono::now());
        if let Some(start) = self.start_value {
            if let Some(end) = self.end_value{
                let duration = end - start;
                defmt::info!("Item {} elapsed {} ms",name,duration.to_millis());
            }
        }
    }
}