#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
use rand::random;

enum MidiStatus {
    On,
    Off
}

impl MidiStatus {
    fn from(status: u8) -> Option<Self> {
        match status {
            128 => Some(MidiStatus::Off),
            144 => Some(MidiStatus::On),
            _ => None
        }
    }
}

#[derive(Default)]
struct Whisper {
    notes: u8
}

impl Whisper {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match MidiStatus::from(data[0]) {
            Some(MidiStatus::Off) => self.process_note_off(),
            Some(MidiStatus::On) => self.process_note_on(),
            _ => (),
        }
    }

    fn process_note_on(&mut self) {
        self.notes += 1;
    }

    fn process_note_off(&mut self) {
        self.notes -= 1;
    }
}

impl Plugin for Whisper {
    fn get_info(&self) -> Info {
        Info {
            name: "Whisper".to_string(),
            vendor: "airtoxin".to_string(),
            unique_id: 1337342,
            inputs: 0,
            outputs: 2,
            category: Category::Synth,
            parameters: 0,
            initial_delay: 0,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, output_buffer) = buffer.split();

        if self.notes == 0 {
            for output_channel in output_buffer.into_iter() {
                for output_sample in output_channel {
                    *output_sample = 0.0;
                }
            }
        } else {
            for output_channel in output_buffer.into_iter() {
                for output_sample in output_channel {
                    *output_sample = (random::<f32>() - 0.5f32) * 2f32;
                }
            }
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event.into() {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }
}

plugin_main!(Whisper);
