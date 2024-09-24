use crate::auxiliary::TimeInstant;
use rtic_time::Monotonic;

use crate::Mono;

//tracks the time that a task takes
pub struct SystemOverhead{
    init_value : Option<TimeInstant>,
    start_sub_value : Option<TimeInstant>,
    end_sub_value : Option<TimeInstant>,
    jitter : u32,

}

impl SystemOverhead{
    pub fn new() -> Self{
        SystemOverhead{
            init_value : None,
            start_sub_value : None,
            end_sub_value : None,
            jitter : 0,
        }
    }

    pub fn start_tracking(&mut self){
        self.init_value = Some(Mono::now());
        self.start_sub_value = None;
        self.end_sub_value = None;
    }

    pub fn start_sub_program(&mut self){
        let st = Mono::now();
        self.start_sub_value = Some(st);
        if let Some(i) = self.init_value {
            self.jitter = (st - i).to_millis();
        }
    }

    pub fn end_sub_program(&mut self){
        self.end_sub_value = Some(Mono::now());
    }

    pub fn show_exec_results(&mut self, name : &'static str) {
        self.end_sub_program();
        if let Some(init_val) = self.init_value {
            let start_sub = self.start_sub_value.unwrap();
            let end_sub = self.end_sub_value.unwrap();
            let subprogram = end_sub - start_sub;
            defmt::info!("Item {}: {} ms subprogram, {} ms of jitter",name,subprogram.to_micros() as f64/1000.0,self.jitter);
        }
    }
}