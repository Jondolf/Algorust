use crate::{
    components::sorting_algorithms::audio_controls::AudioConfig,
    utils::audio::{Note, Synth},
};

use sorting_algorithms::SortCommand;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

pub fn use_sort_audio(items: Vec<u32>, step: Vec<SortCommand<u32>>, config: AudioConfig) {
    let synth = use_state_eq(|| Rc::new(RefCell::new(Synth::new())));

    use_effect_with_deps(
        move |step| {
            if config.enabled {
                synth.borrow_mut().stop_all();

                let mut notes: Vec<Note> = vec![];

                let ctx = Rc::clone(&(**synth).borrow().ctx);

                for command in step.iter() {
                    let val = match command {
                        SortCommand::Swap(_, to) => items[*to],
                        SortCommand::Set(index, _) => items[*index],
                    } as f32;
                    let ratio = val / *items.iter().max().unwrap() as f32;
                    let frequency = config.min_frequency
                        + (config.max_frequency - config.min_frequency) * ratio;

                    notes.push(Note::new(&ctx, frequency, config.sound_type));
                }

                synth.borrow_mut().play(notes, config.note_duration);
            }
            || ()
        },
        step,
    );
}
