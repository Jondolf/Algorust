use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::{
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

    let algorithm_options = get_sorting_algorithms()
        .values()
        .map(|a| view_sorting_algorithm_option(&config.sorting_algorithm.name, &a.name))
        .collect::<Html>();

    let gen_input = {
        let config = config.clone();

        move |_e: MouseEvent| {
            update_input.emit(knuth_shuffle(gen_u32_vec(config.input_len)));
        }
    };
    let change_input_len = {
        let config = config.clone();
        let update_config = update_config.clone();

        move |e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            if let Ok(input_len) = el.value().parse::<usize>() {
                if input_len > 1 {
                    update_config.emit((
                        SortConfig {
                            input_len,
                            ..config.clone()
                        },
                        true,
                    ));
                }
            }
        }
    };
    let change_algorithm = move |e: Event| {
        let el: HtmlInputElement = e.target_unchecked_into();
        history.push(SortingAlgorithmsRoute::SortingAlgorithm {
            algorithm: el.value().replace(" ", "-").to_lowercase(),
        });
    };
    let toggle_audio = {
        let config = config.clone();

        move |_| {
            update_config.emit((
                SortConfig {
                    audio_enabled: !config.audio_enabled,
                    ..config.clone()
                },
                false,
            ));
        }
    };

    html! {
        <div class="sort-controls">
            <div class="input-item">
                <button onclick={gen_input}>{"Generate input"}</button>
            </div>
            <div class="input-item">
                <label for="inputLength">{"Input length"}</label>
                <input id="inputLength"
                    type="number"
                    placeholder="Input length"
                    min=1
                    value={props.config.input_len.to_string()}
                    oninput={change_input_len}
                />
            </div>
            <div class="input-item">
                <label for="sortingAlgorithm">{"Algorithm"}</label>
                <select id="sortingAlgorithm" name="Sorting algorithm" onchange={change_algorithm}>
                    { algorithm_options }
                </select>
            </div>
            <div class="input-item checkbox">
                <input id="audioEnabled" type="checkbox" checked={config.audio_enabled} onchange={toggle_audio} />
                <label for="audioEnabled">{"Audio enabled"}</label>
            </div>
        </div>
    }
}

fn view_sorting_algorithm_option(curr_algorithm: &str, name: &str) -> Html {
    html! {
        <option key={name.to_string()} value={name.to_string()} selected={curr_algorithm == name}>{name}</option>
    }
}
