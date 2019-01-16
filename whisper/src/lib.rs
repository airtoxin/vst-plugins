#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin};
use vst::buffer::AudioBuffer;
use rand::random;

#[derive(Default)]
struct Whisper;

// We're implementing a trait `Plugin` that does all the VST-y stuff for us.
impl Plugin for Whisper {
    fn get_info(&self) -> Info {
        Info {
            name: "Whisper".to_string(),
            vendor: "airtoxin".to_string(),
            unique_id: 1337342,

            // For now, fill in the rest of our fields with `Default` info.
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        for (_, output_buffer) in buffer.zip() {
            // Let's iterate over every sample in our channel.
            for output_sample in output_buffer {
                // For every sample, we want to add a random value from
                // -1.0 to 1.0.
                *output_sample = (random::<f32>() - 0.5f32) * 2f32;
            }
        }
    }
}

// Make sure you call this, or nothing will happen.
plugin_main!(Whisper);
