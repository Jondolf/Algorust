use instant::Duration;
use std::rc::Rc;
use web_sys::{AudioContext, DynamicsCompressorNode, GainNode, OscillatorNode, OscillatorType};

/// Used for playing `Note`s.
#[derive(Clone, PartialEq)]
pub struct Synth {
    pub ctx: Rc<AudioContext>,
    compressor: DynamicsCompressorNode,
    notes: Vec<Note>,
}

impl Synth {
    /// Creates a new `Synth` with an `AudioContext` and a compressor.
    pub fn new() -> Self {
        let ctx = Rc::new(AudioContext::new().unwrap());
        let compressor = ctx.create_dynamics_compressor().unwrap();
        compressor.knee().set_value(10.0); // Reduce audio distortion
        compressor
            .connect_with_audio_node(&ctx.destination())
            .unwrap();

        Self {
            ctx,
            compressor,
            notes: vec![],
        }
    }
    /// Play `Note`s for a given duration.
    pub fn play(&mut self, notes: Vec<Note>, duration: Duration) {
        for note in notes {
            note.gain
                .gain()
                .linear_ramp_to_value_at_time(1.0, self.ctx.current_time() + 0.05)
                .unwrap();

            note.osc.connect_with_audio_node(&note.gain).unwrap();
            note.gain.connect_with_audio_node(&self.compressor).unwrap();

            note.osc.start().unwrap();

            note.gain
                .gain()
                .linear_ramp_to_value_at_time(0.0, self.ctx.current_time() + duration.as_secs_f64())
                .unwrap();
            note.osc
                .stop_with_when(self.ctx.current_time() + duration.as_secs_f64())
                .unwrap();

            self.notes.push(note);
        }
    }
    /// Stops all currently playing notes with a 0.5 second fade.
    pub fn stop_all(&mut self) {
        for note in &self.notes {
            note.gain
                .gain()
                .linear_ramp_to_value_at_time(0.0, self.ctx.current_time() + 0.5)
                .unwrap();
        }
        self.notes = vec![];
    }
}

/// A container of data related to a note. To play the note, use a `Synth`.
#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub osc: OscillatorNode,
    pub gain: GainNode,
    pub frequency: f32,
    pub oscillator_type: OscillatorType,
}
impl Note {
    /// Creates a `Note` with an oscillator and gain. Doesn't play the note.
    pub fn new(ctx: &AudioContext, frequency: f32, oscillator_type: OscillatorType) -> Self {
        let osc = ctx.create_oscillator().unwrap();
        let gain = ctx.create_gain().unwrap();

        osc.set_type(oscillator_type);
        osc.frequency().set_value(frequency);
        gain.gain().set_value(0.0); // Muted by default

        Self {
            osc,
            gain,
            frequency,
            oscillator_type,
        }
    }
}
