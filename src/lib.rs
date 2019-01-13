#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Plugin, Info, Category};

struct DigiDist {
    threshold: f32
}

impl Default for DigiDist {
    fn default() -> DigiDist {
        DigiDist {
            threshold: 1.0,
        }
    }
}

impl Plugin for DigiDist {
    fn get_info(&self) -> Info {
        Info {
            name: "DigiDist".to_string(),
            vendor: "airtoxin".to_string(),
            unique_id: 20190113,

            inputs: 2,
            outputs: 2,
            parameters: 1,

            category: Category::Effect,

            ..Info::default()
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.threshold * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            // We don't want to divide by zero, so we'll clamp the value
            0 => self.threshold = value.max(0.01),
            _ => (),
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // For each buffer, transform the samples
        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {

                if *input_sample >= 0.0 {
                    *output_sample = input_sample.min(self.threshold) / self.threshold;
                }
                else {
                    *output_sample = input_sample.max(-self.threshold) / self.threshold;
                }
            }
        }
    }
}

plugin_main!(DigiDist);
