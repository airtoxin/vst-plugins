#[macro_use] extern crate vst2;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Plugin, Info};

#[derive(Default)]
struct DigiDist;

impl Plugin for DigiDist {
    fn get_info(&self) -> Info {
        Info {
            name: "DigiDist".to_string(),
            vendor: "airtoxin".to_string(),
            unique_id: 20190113,

            // fill in the rest with the default values
            ..Info::default()
        }
    }
}

plugin_main!(DigiDist);
