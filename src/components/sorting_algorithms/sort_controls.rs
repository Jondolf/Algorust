use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::{
    components::input_items::*,
    pages::sorting_algorithms::{get_sorting_algorithms, SortConfig, SortingAlgorithmsRoute},
    utils::{gen_u32_vec, knuth_shuffle},
};

#[derive(Properties, Clone, PartialEq)]
pub struct SortControlsProps {
    pub config: SortConfig,
    pub update_input: Callback<Vec<u32>>,
    pub update_config: Callback<(SortConfig, bool)>,
}

#[function_component(SortControls)]
pub fn sort_controls(props: &SortControlsProps) -> Html {
    let SortControlsProps {
        config,
        update_input,
        update_config,
    } = props.clone();

    let history = use_history().unwrap();

    let algorithm_names = use_state_eq(|| {
        get_sorting_algorithms()
            .values()
            .map(|algorithm| algorithm.name.to_string())
            .collect::<Vec<String>>()
    });

    let gen_input = {
        let config = config.clone();

        Callback::from(move |_e: MouseEvent| {
            update_input.emit(knuth_shuffle(gen_u32_vec(config.input_len)));
        })
    };
    let change_input_len = {
        let config = config.clone();
        let update_config = update_config.clone();

        Callback::from(move |e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            if let Ok(input_len) = el.value().parse::<usize>() {
                if input_len > 1 && input_len != config.input_len {
                    update_config.emit((
                        SortConfig {
                            input_len,
                            ..config.clone()
                        },
                        true,
                    ));
                }
            }
        })
    };
    let change_playback_time = {
        let config = config.clone();
        let update_config = update_config.clone();

        Callback::from(move |e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            if let Ok(playback_time) = el.value().parse::<f32>() {
                update_config.emit((
                    SortConfig {
                        playback_time,
                        ..config.clone()
                    },
                    true,
                ));
            }
        })
    };
    let change_algorithm = Callback::from(move |e: Event| {
        let el: HtmlInputElement = e.target_unchecked_into();
        history.push(SortingAlgorithmsRoute::SortingAlgorithm {
            algorithm: el.value().replace(" ", "-").to_lowercase(),
        });
    });
    let toggle_audio = {
        let config = config.clone();

        Callback::from(move |_| {
            update_config.emit((
                SortConfig {
                    audio_enabled: !config.audio_enabled,
                    ..config.clone()
                },
                false,
            ));
        })
    };

    html! {
        <div class="sort-controls">
            <Button title="Generate input" onclick={gen_input} />
            <IntInput<usize>
                title="Input length"
                value={props.config.input_len}
                oninput={change_input_len}
                min={2}
            />
            <FloatInput<f32>
                title="Playback time (seconds)"
                value={props.config.playback_time}
                oninput={change_playback_time}
                min={0.0}
            />
            <SelectInput
                title="Algorithm"
                options={(*algorithm_names).clone()}
                selected_value={config.sorting_algorithm.name}
                onchange={change_algorithm}
            />
            <Checkbox title="Audio enabled" value={config.audio_enabled} oninput={toggle_audio} />
        </div>
    }
}
