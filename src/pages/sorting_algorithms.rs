use crate::{
    components::sorting_algorithms::{
        sort_controls::SortControls, sort_desc::SortDesc, sort_graph::SortGraph,
    },
    utils::{gen_u32_vec, knuth_shuffle},
};
use sorting_algorithms::*;
use std::collections::BTreeMap;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SortingAlgorithm {
    pub name: String,
    pub run: fn(Vec<u32>) -> SortResult<u32>,
}
impl SortingAlgorithm {
    fn new(name: &str, run: fn(Vec<u32>) -> SortResult<u32>) -> Self {
        Self {
            name: name.to_string(),
            run,
        }
    }
}
impl Default for SortingAlgorithm {
    fn default() -> Self {
        Self {
            name: String::from("Bubble sort"),
            run: bubble_sort::sort,
        }
    }
}

pub fn get_sorting_algorithms() -> BTreeMap<&'static str, SortingAlgorithm> {
    // `BTreeMap` because it keeps the order of the items.
    BTreeMap::from([
        (
            "bubble-sort",
            SortingAlgorithm::new("Bubble sort", bubble_sort::sort),
        ),
        (
            "insertion-sort",
            SortingAlgorithm::new("Insertion sort", insertion_sort::sort),
        ),
        (
            "merge-sort",
            SortingAlgorithm::new("Merge sort", merge_sort::sort),
        ),
    ])
}

#[derive(Clone, Debug, Routable, PartialEq)]
pub enum SortingAlgorithmsRoute {
    #[at("/sorting-algorithms")]
    SortingAlgorithms,
    #[at("/sorting-algorithms/:algorithm")]
    SortingAlgorithm { algorithm: String },
}

pub fn switch_sorting_algorithms(route: &SortingAlgorithmsRoute) -> Html {
    match route {
        SortingAlgorithmsRoute::SortingAlgorithms => html! {
            <Redirect<SortingAlgorithmsRoute> to={SortingAlgorithmsRoute::SortingAlgorithm { algorithm: "bubble-sort".to_string()} } />
        },
        SortingAlgorithmsRoute::SortingAlgorithm { algorithm } => {
            if get_sorting_algorithms().contains_key(algorithm.as_str()) {
                html! {
                    <SortingAlgorithmsPage algorithm={algorithm.to_string()} />
                }
            } else {
                html! {
                    <SortingAlgorithms404Page algorithm={algorithm.to_string()} />
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SortConfig {
    pub input_len: usize,
    pub sorting_algorithm: SortingAlgorithm,
    pub audio_enabled: bool,
}
impl Default for SortConfig {
    fn default() -> Self {
        Self {
            input_len: 100,
            sorting_algorithm: SortingAlgorithm::default(),
            audio_enabled: true,
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct SortingAlgorithmsPageProps {
    #[prop_or("bubble-sort".to_string())]
    pub algorithm: String,
}

#[function_component(SortingAlgorithmsPage)]
pub fn sorting_algorithms_page(props: &SortingAlgorithmsPageProps) -> Html {
    let config = {
        let algorithm_name = props.algorithm.to_string();

        use_state_eq(|| {
            let mut config = SortConfig::default();
            if let Some(algorithm) = get_sorting_algorithms().get(algorithm_name.as_str()) {
                config.sorting_algorithm = algorithm.to_owned();
            }
            config
        })
    };
    let input = use_state_eq(|| knuth_shuffle(gen_u32_vec(config.input_len)));
    let output = use_state_eq(|| (config.sorting_algorithm.run)((*input).clone()));
    let active_step_index: UseStateHandle<usize> = use_state_eq(|| 0);

    let route = use_route::<SortingAlgorithmsRoute>();

    let update_values = {
        let input = input.clone();
        let output = output.clone();
        let active_step_index = active_step_index.clone();

        move |new_input: Vec<u32>, config: &SortConfig| {
            let new_output = (config.sorting_algorithm.run)(new_input.clone());
            active_step_index.set(0);
            input.set(new_input);
            output.set(new_output);
        }
    };

    let update_input = {
        let config = config.clone();
        let update_values = update_values.clone();

        Callback::from(move |val| {
            update_values(val, &config);
        })
    };

    let update_config = {
        let config = config.clone();

        Callback::from(move |msg: (SortConfig, bool)| {
            if msg.1 {
                let new_input = knuth_shuffle(gen_u32_vec(msg.0.input_len));
                update_values(new_input, &msg.0);
            }
            config.set(msg.0);
        })
    };

    let update_active_step = {
        let active_step_index = active_step_index.clone();

        Callback::from(move |event: InputEvent| {
            let el: HtmlInputElement = event.target_unchecked_into();
            let value = el.value().parse::<usize>().unwrap();
            active_step_index.set(value);
        })
    };

    {
        let config = config.clone();
        let update_config = update_config.clone();

        use_effect_with_deps(
            move |route| {
                let algorithm_name = match route.as_ref().unwrap() {
                    SortingAlgorithmsRoute::SortingAlgorithm { algorithm } => algorithm,
                    _ => "bubble-sort",
                };
                if let Some(algorithm) = get_sorting_algorithms().get(algorithm_name) {
                    update_config.emit((
                        SortConfig {
                            sorting_algorithm: algorithm.to_owned(),
                            ..(*config).clone()
                        },
                        true,
                    ))
                } else {
                    update_config.emit(((*config).clone(), false));
                };
                || ()
            },
            route,
        );
    }

    html! {
        <div id="SortingAlgorithms">
                <h1>{"Sorting algorithms"}</h1>

                <SortControls config={(*config).clone()} {update_input} {update_config} />

                <div class="content">
                    <h2>{ format!("{} steps, {}", output.steps.len(), get_sort_duration_ms(&output.clone())) }</h2>

                    {
                        if *active_step_index == 0 {
                            html! { <SortGraph items={(*input).clone()} /> }
                        } else {
                            html! {
                                <SortGraph
                                    items={get_output_at_step_index(&input, &output.steps, *active_step_index)}
                                    step={(*output.steps)[*active_step_index - 1].clone()}
                                    audio_enabled={config.audio_enabled}
                                />
                            }
                        }
                    }

                    <div class="step-selector">
                        <label for="active-step-input">
                            {
                                if *active_step_index == 0 {
                                    format!("Step: {} (input)", *active_step_index)
                                } else {
                                    format!("Step: {}", *active_step_index)
                                }
                            }
                        </label>
                        <input
                            type="range"
                            id="active-step-input"
                            min="0"
                            max={(output.steps.len()).to_string()}
                            value={active_step_index.to_string()}
                            oninput={update_active_step}
                        />
                    </div>
                </div>

                <SortDesc url={get_sort_desc_url(&config.sorting_algorithm.name)} />
            </div>
    }
}

/// Gets the output at a given step's index by running [`SortCommand`]s on the input.
fn get_output_at_step_index(
    input: &[u32],
    steps: &[Vec<SortCommand<u32>>],
    index: usize,
) -> Vec<u32> {
    let mut output = input.to_vec();
    run_sort_steps(&mut output, steps[0..index].to_vec());
    output
}

fn get_sort_duration_ms(output: &SortResult<u32>) -> String {
    format!("{:?} ms", &output.duration.unwrap().as_millis())
}

fn get_sort_desc_url(algorithm_name: &str) -> String {
    let origin = window().unwrap().location().origin().unwrap();
    format!(
        "{}/src/{}/README.md",
        origin,
        algorithm_name.to_lowercase().replace(" ", "_")
    )
}

#[derive(Clone, PartialEq, Properties)]
struct SortingAlgorithms404PageProps {
    algorithm: String,
}

#[function_component(SortingAlgorithms404Page)]
fn sorting_algorithms_404_page(props: &SortingAlgorithms404PageProps) -> Html {
    html! {
        <>
            <h1>{ "404" }</h1>
            <p>{ format!("The algorithm \"{}\" was not found.", props.algorithm) }</p>
            <Link<SortingAlgorithmsRoute> to={SortingAlgorithmsRoute::SortingAlgorithms}>
                { "Back to sorting algorithms" }
            </Link<SortingAlgorithmsRoute>>
        </>
    }
}
