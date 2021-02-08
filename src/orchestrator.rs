use std::{thread, time};

pub struct Orchestrator{
    pub ticks_delay: u64,
    pub green_energy_timeout: u64,
}

impl Orchestrator {
    /// @param ticks_delay delay between ticks of the orchestrator in seconds
    /// @param green_energy_timeout delay before forcing jobs processing without green energy in seconds
    pub fn new(ticks_delay: u64, green_energy_timeout: u64) -> Orchestrator{
        Orchestrator{
            ticks_delay,
            green_energy_timeout
        }
    }

    pub fn start(&self){
        loop {
            println!("orchestrator tick");
            thread::sleep(time::Duration::from_secs(self.ticks_delay));
        }
    }
}

