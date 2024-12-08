use std::time::Duration;

use xplm::{
    data::{borrowed::DataRef, DataRead, ReadOnly},
    flight_loop::{FlightLoop, FlightLoopCallback},
};

#[derive(Clone, Copy)]
pub struct FrameData {
    pub paused: bool,
    pub sim_interval: Duration,
    pub real_interval: Duration,
}
pub trait Update {
    fn update(&mut self, data: FrameData);
}

pub struct UpdateLoopHandler {
    sim_paused: DataRef<i32, ReadOnly>,
    sim_speed: DataRef<f32, ReadOnly>,
    listeners: Vec<Box<dyn Update>>,
}

impl UpdateLoopHandler {
    pub fn new(listeners: Vec<Box<dyn Update>>) -> Self {
        Self {
            sim_paused: DataRef::find("sim/time/paused").expect("Stock dataref to be found"),
            sim_speed: DataRef::find("sim/time/sim_speed_actual_ogl")
                .expect("Stock dataref to be found"),
            listeners,
        }
    }
    pub fn into_flightloop(self) -> FlightLoop {
        let mut result = FlightLoop::new(self);
        result.schedule_immediate();
        result
    }
}

impl FlightLoopCallback for UpdateLoopHandler {
    fn flight_loop(&mut self, state: &mut xplm::flight_loop::LoopState) {
        let paused = self.sim_paused.get() == 1;
        let real_interval = state.since_last_call();
        let sim_interval = match paused {
            true => Duration::new(0, 0),
            false => real_interval.mul_f32(self.sim_speed.get()),
        };
        let data = FrameData {
            paused,
            sim_interval,
            real_interval,
        };
        for listener in &mut self.listeners {
            listener.update(data);
        }
    }
}
