#[macro_use]
extern crate vst;

use vst::api::Events;
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{Category, Info, Plugin};

use std::f64::consts::PI;

const TAU: f64 = PI * 2.0;

enum MidiStatus {
    On,
    Off,
}

impl MidiStatus {
    fn from(status: u8) -> Option<Self> {
        match status {
            128 => Some(MidiStatus::Off),
            144 => Some(MidiStatus::On),
            _ => None,
        }
    }
}

#[derive(Default)]
struct SinSynth {
    notes: u8,
    time: f64,
    sample_rate: f64,
}

impl SinSynth {
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

    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    fn get_sine_signal(&self, time: f64, frequency: f64) -> f64 {
        (time * frequency * TAU).sin()
    }
}

impl Plugin for SinSynth {
    fn get_info(&self) -> Info {
        Info {
            name: "SinSynth".to_string(),
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

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, output_buffer) = buffer.split();

        let mut time_buffer = self.time;
        let per_sample = self.time_per_sample();

        if self.notes == 0 {
            for output_channel in output_buffer.into_iter() {
                for output_sample in output_channel {
                    *output_sample = 0.0;
                }
            }
        } else {
            for output_channel in output_buffer.into_iter() {
                for output_sample in output_channel {
                    let signal = (self.get_sine_signal(time_buffer, 220.0)
                        + self.get_sine_signal(time_buffer, 220.0 * 2.0) / 2.0
                        + self.get_sine_signal(time_buffer, 220.0 * 3.0) / 3.0
                        + self.get_sine_signal(time_buffer, 220.0 * 4.0) / 4.0
                        + self.get_sine_signal(time_buffer, 220.0 * 5.0) / 5.0)
                        / 5.0;
                        *output_sample = signal as f32;
                    time_buffer += per_sample;
                }
            }
        }

        self.time += per_sample * buffer.samples() as f64;
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

plugin_main!(SinSynth);
