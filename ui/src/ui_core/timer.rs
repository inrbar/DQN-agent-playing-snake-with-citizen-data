use std::time::Instant;
const FRAME_TIME : f32 = 1.0/9.0;

pub struct Timer{
    pub start_time: Instant,
    pub elapsed_secs: f32,
    pub sec_counter: f32
}

impl Timer {

    //this name is shit
    pub fn has_frame_passed(&mut self) -> bool{
        self.elapsed_secs = self.start_time.elapsed().as_secs_f32();
        match self.elapsed_secs >= self.sec_counter {
            true => {
                self.sec_counter += FRAME_TIME;
                true
            },
            false => {
                false
            }
        }
        
    }

    pub fn reset_timer(&mut self){
        self.start_time = Instant::now();
        self.sec_counter = 1.0;
    }
    
}