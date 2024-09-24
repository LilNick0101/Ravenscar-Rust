use crate::auxiliary::TimeInstant;
use rtic_time::Monotonic;

use crate::Mono;

//tracks the time that a task takes
pub struct TaskTracker{
    release_time : Option<TimeInstant>,
    start_sub_value : Option<TimeInstant>,
    end_sub_value : Option<TimeInstant>,

}

impl TaskTracker{
    pub fn new() -> Self{
        TaskTracker{
            release_time : None,
            start_sub_value : None,
            end_sub_value : None
        }
    }

    pub fn start_tracking(&mut self){
        self.release_time = Some(Mono::now());
        self.start_sub_value = None;
        self.end_sub_value = None;
    }

    pub fn start_sub_program(&mut self){
        self.start_sub_value = Some(Mono::now());
    }

    pub fn end_sub_program(&mut self){
        self.end_sub_value = Some(Mono::now());
    }

    pub fn end_tracking(&mut self, name : &'static str) {
        let now = Mono::now();
        if let Some(init_val) = self.release_time {
            //defmt::info!("Item {}: {} ms subprogram, {} ms of overhead",name,subprogram.to_millis(),duration.to_millis());
        }
    }
}