use std::rc::Rc;

use crate::{
    components::sorting::audio_controls::AudioConfig,
    utils::audio::{Note, Synth},
};

use sorting::SortCommand;
use yew::prelude::*;

#[hook]
pub fn use_sort_audio(
    items: UseStateHandle<Vec<u32>>,
    step: UseStateHandle<Vec<SortCommand<u32>>>,
    config: AudioConfig,
) {
    let synth = use_mut_ref(Synth::new);

    use_effect(move || {
        if config.enabled {
            synth.borrow_mut().stop_all();

            let mut notes: Vec<Note> = vec![];

            let ctx = Rc::clone(&synth.borrow().ctx);

            for command in step.iter() {
                let val = match command {
                    SortCommand::Swap(_, to) => items[*to],
                    SortCommand::Set(index, _) => items[*index],
                } as f32;
                let ratio = val / *items.iter().max().unwrap() as f32;
                let frequency =
                    config.min_frequency + (config.max_frequency - config.min_frequency) * ratio;

                notes.push(Note::new(&ctx, frequency, config.sound_type));
            }

            synth.borrow_mut().play(notes, config.note_duration);
        }
        || ()
    });
}
