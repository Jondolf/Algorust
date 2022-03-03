use crate::utils::audio::{Note, Synth};

// std::time isn't supported on WASM platforms
use instant::Duration;
use sorting_algorithms::SortCommand;
use std::{cell::RefCell, rc::Rc};
use web_sys::OscillatorType;
use yew::prelude::*;

pub fn use_sort_audio(
    items: Vec<u32>,
    step: Vec<SortCommand<u32>>,
    note_duration: Duration,
    enabled: bool,
) {
    let synth = use_state_eq(|| Rc::new(RefCell::new(Synth::new())));

    use_effect_with_deps(
        move |step| {
            if enabled {
                synth.borrow_mut().stop_all();

                let max_frequency = 800.0;
                let min_frequency = 20.0;

                let mut notes: Vec<Note> = vec![];

                let ctx = Rc::clone(&(**synth).borrow().ctx);

                for command in step.iter() {
                    let val = match command {
                        SortCommand::Swap(_, to) => items[*to],
                        SortCommand::Set(index, _) => items[*index],
                    } as f32;
                    let ratio = val / *items.iter().max().unwrap() as f32;
                    let frequency = min_frequency + (max_frequency - min_frequency) * ratio;

                    notes.push(Note::new(&ctx, frequency, OscillatorType::Sine));
                }

                synth.borrow_mut().play(notes, note_duration);
            }
            || ()
        },
        step,
    );
}
