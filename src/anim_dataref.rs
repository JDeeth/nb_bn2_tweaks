use std::f32::EPSILON;

use xplm::data::{owned::OwnedData, DataRead, DataReadWrite, ReadOnly};

use crate::third_party_dataref::ThirdPartyDataref;
use crate::update_loop::{FrameData, Update};

pub struct AnimDataref {
    // dataref that moves at a linear speed
    source: ThirdPartyDataref,
    output: OwnedData<f32, ReadOnly>,
    rate_per_sec: f32,
}

impl AnimDataref {
    pub fn new(source_dataref: String, output_dataref: &str, rate_per_sec: f32) -> Self {
        let mut source = ThirdPartyDataref::new(source_dataref);
        let source_val = source.get().unwrap_or(0) as f32;
        let output =
            OwnedData::<f32, ReadOnly>::create_with_value(output_dataref, &source_val).unwrap();
        Self {
            source,
            output,
            rate_per_sec,
        }
    }
}

impl Update for AnimDataref {
    fn update(&mut self, data: FrameData) {
        let target = self.source.get().unwrap_or(0) as f32;
        let current = self.output.get();
        let diff = target - current;
        if diff.abs() < EPSILON {
            return;
        }
        let change = diff
            .abs()
            .min(self.rate_per_sec * data.sim_interval.as_secs_f32());
        self.output.set(
            current
                + match diff.is_sign_positive() {
                    true => change,
                    false => -change,
                },
        );
    }
}
