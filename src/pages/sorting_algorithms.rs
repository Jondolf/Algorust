use crate::{
    components::{
        collapsible::Collapsible,
        sidebar::Sidebar,
        sorting_algorithms::{
            audio_controls::{AudioConfig, AudioControls},
            sort_controls::SortControls,
            sort_desc::SortDesc,
            sort_graph::SortGraph,
        },
        step_slider::StepSlider,
    },
    hooks::use_sort_audio::use_sort_audio,
    utils::{gen_u32_vec, knuth_shuffle},
};
use sorting_algorithms::*;
use std::{borrow::Borrow, collections::BTreeMap};
use web_sys::window;
use yew::prelude::*;
use yew_hooks::use_title;
use yew_router::prelude::*;

type SortSteps = Vec<Vec<SortCommand<u32>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct SortingAlgorithm {
    pub name: String,
    sort: fn(Vec<u32>, SortSteps) -> (Vec<u32>, SortSteps),
}
impl SortingAlgorithm {
    fn new(name: &str, sort: fn(Vec<u32>, SortSteps) -> (Vec<u32>, SortSteps)) -> Self {
        Self {
            name: name.to_string(),
            sort,
        }
    }
    fn sort(&self, input: Vec<u32>) -> SortResult<u32> {
        run_sort(input, self.sort)
    }
}
impl Default for SortingAlgorithm {
    fn default() -> Self {
        Self {
            name: String::from("Bubble sort"),
            sort: bubble_sort,
        }
    }
}

pub fn get_sorting_algorithms() -> BTreeMap<&'static str, SortingAlgorithm> {
    // `BTreeMap` because it keeps the order of the items.
    BTreeMap::from([
        (
            "bubble-sort",
            SortingAlgorithm::new("Bubble sort", bubble_sort),
        ),
        (
            "insertion-sort",
            SortingAlgorithm::new("Insertion sort", insertion_sort),
        ),
        (
            "merge-sort",
            SortingAlgorithm::new("Merge sort", merge_sort),
        ),
        ("heapsort", SortingAlgorithm::new("Heapsort", heapsort)),
        ("quicksort", SortingAlgorithm::new("Quicksort", quicksort)),
        (
            "bucket-sort",
            SortingAlgorithm::new("Bucket sort", bucket_sort),
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
    /// How long the playback of steps should take in seconds.
    pub playback_time: f32,
    pub audio_config: AudioConfig,
}
impl Default for SortConfig {
    fn default() -> Self {
        Self {
            input_len: 100,
            sorting_algorithm: SortingAlgorithm::default(),
            audio_enabled: true,
            playback_time: 10.0,
            audio_config: AudioConfig::default(),
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

        use_state(|| {
            let mut config = SortConfig::default();
            if let Some(algorithm) = get_sorting_algorithms().get(algorithm_name.as_str()) {
                config.sorting_algorithm = algorithm.to_owned();
            }
            config
        })
    };
    let input = use_state(|| knuth_shuffle(gen_u32_vec(config.input_len)));
    let output = use_state_eq(|| config.sorting_algorithm.sort(input.to_vec()));
    let active_step_index: UseStateHandle<usize> = use_state_eq(|| 0);
    // The active step is empty at the input step, step 0.
    let active_step = if *active_step_index == 0 {
        vec![]
    } else {
        (*output.steps)[*active_step_index - 1].clone()
    };
    let output_at_active_step = use_state_eq(|| {
        get_output_at_step_index(&*(*input).borrow(), &output.steps, *active_step_index)
    });

    let route = use_route::<SortingAlgorithmsRoute>();

    let update_values = {
        let input = input.clone();
        let output = output.clone();
        let active_step_index = active_step_index.clone();
        let output_at_active_step = output_at_active_step.clone();

        move |new_input: Vec<u32>, config: &SortConfig| {
            let new_output = config.sorting_algorithm.sort(new_input.clone());
            let new_active_step = 0;
            active_step_index.set(new_active_step);
            output_at_active_step.set(get_output_at_step_index(
                &*(*new_input).borrow(),
                &new_output.steps,
                new_active_step,
            ));
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
        let update_values = update_values.clone();

        Callback::from(move |msg: (SortConfig, bool)| {
            if msg.1 {
                let new_input = knuth_shuffle(gen_u32_vec(msg.0.input_len));
                update_values(new_input, &msg.0);
            }
            config.set(msg.0);
        })
    };

    let update_audio_config = {
        let config = config.clone();
        let update_values = update_values.clone();

        Callback::from(move |msg: (AudioConfig, bool)| {
            if msg.1 {
                let new_input = knuth_shuffle(gen_u32_vec(config.input_len));
                update_values(new_input, &config);
            }
            config.set(SortConfig {
                audio_config: msg.0,
                ..(*config).clone()
            });
        })
    };

    let change_step = {
        let active_step_index = active_step_index.clone();
        let output = output.clone();
        let output_at_active_step = output_at_active_step.clone();

        Callback::from(move |val: usize| {
            output_at_active_step.set(get_output_at_step_index(
                &*(*input).borrow(),
                &output.steps,
                val,
            ));
            active_step_index.set(val);
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

    use_title(format!(
        "{} - Sorting algorithms",
        config.sorting_algorithm.name
    ));

    use_sort_audio(
        output_at_active_step.to_vec(),
        active_step.clone(),
        config.audio_config.clone(),
    );

    html! {
        <div class="page" id="SortingAlgorithms">
            <Sidebar>
                <h2>{"Config"}</h2>

                <Collapsible title="General" open={true} class="config-section">
                    <SortControls config={(*config).clone()} {update_input} {update_config} />
                </Collapsible>

                <Collapsible title="Audio" open={false} class="config-section">
                    <AudioControls config={config.audio_config.clone()} update_config={update_audio_config} />
                </Collapsible>
            </Sidebar>

            <main>
                <div class="visualization">
                    <h2>{ format!("{} steps, {:?} ms", output.steps.len(), &output.duration.unwrap().as_millis()) }</h2>

                    <SortGraph
                        items={output_at_active_step.to_vec()}
                        step={active_step.to_vec()}
                    />

                    <StepSlider
                        active_step_index={*active_step_index}
                        max={output.steps.len()}
                        on_change={change_step}
                        playback_time={config.playback_time}
                    />

                    <span class="step-info">
                        <label for="stepSlider">
                            {
                                if *active_step_index == 0 {
                                    format!("Step {} (input)", *active_step_index)
                                } else {
                                    format!("Step {}: ", *active_step_index)
                                }
                            }
                        </label>
                        {
                            active_step.iter().map(|command| match command {
                                SortCommand::Swap(from, to) => format!("SWAP indices {} and {}", from, to),
                                SortCommand::Set(i, val) => format!("SET value at index {} to {}", i, val),
                            }).collect::<Vec<String>>().join(";")
                        }
                    </span>
                </div>

                <SortDesc url={get_sort_desc_url(&config.sorting_algorithm.name)} />
            </main>
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
    run_sort_steps(&mut output, &steps[0..index]);
    output
}

fn get_sort_desc_url(algorithm_name: &str) -> String {
    let origin = window().unwrap().location().origin().unwrap();
    format!(
        "{}/src/{}/README.md",
        origin,
        algorithm_name.to_lowercase().replace(' ', "_")
    )
}

#[derive(Clone, PartialEq, Properties)]
struct SortingAlgorithms404PageProps {
    algorithm: String,
}

#[function_component(SortingAlgorithms404Page)]
fn sorting_algorithms_404_page(props: &SortingAlgorithms404PageProps) -> Html {
    use_title("404 - Sorting algorithms".to_string());

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
