#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin};

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
}

// Make sure you call this, or nothing will happen.
plugin_main!(Whisper);
