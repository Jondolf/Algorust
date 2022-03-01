use crate::{
    components::sorting_algorithms::{
        sort_controls::SortControls, sort_desc::SortDesc, sort_graph::SortGraph,
    },
    utils::{gen_u32_vec, knuth_shuffle},
};
use sorting_algorithms::*;
use std::{collections::BTreeMap, num::ParseIntError};
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_router::{prelude::*, scope_ext::HistoryHandle};

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

pub fn sorting_algorithms() -> BTreeMap<&'static str, SortingAlgorithm> {
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
            <Redirect<SortingAlgorithmsRoute> to={SortingAlgorithmsRoute::SortingAlgorithm {algorithm: "bubble-sort".to_string()}} />
        },
        SortingAlgorithmsRoute::SortingAlgorithm { algorithm } => {
            if sorting_algorithms().contains_key(algorithm.as_str()) {
                html! {
                    <SortingAlgorithms algorithm={algorithm.to_string()} />
                }
            } else {
                html! {
                    <SortingAlgorithms404 algorithm={algorithm.to_string()} />
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct SortingAlgorithms404Props {
    algorithm: String,
}

#[function_component(SortingAlgorithms404)]
fn sorting_algorithms_404(props: &SortingAlgorithms404Props) -> Html {
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

pub enum Msg {
    UpdateInput(Vec<u32>),
    /// Receives a new config and a boolean that controls if the change causes a rerender.
    UpdateConfig(SortConfig, bool),
    ChangeActiveStep(Result<usize, ParseIntError>),
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
pub struct SortingAlgorithmsProps {
    #[prop_or("bubble-sort".to_string())]
    pub algorithm: String,
}

pub struct SortingAlgorithms {
    input: Vec<u32>,
    output: SortResult<u32>,
    sort_config: SortConfig,
    steps: Vec<Vec<SortCommand<u32>>>,
    active_step_index: usize,
    _history_listener: HistoryHandle,
}

impl Component for SortingAlgorithms {
    type Message = Msg;
    type Properties = SortingAlgorithmsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut sort_config = SortConfig::default();
        let algorithms = sorting_algorithms();
        if let Some(algorithm) = algorithms.get(_ctx.props().algorithm.as_str()) {
            sort_config.sorting_algorithm = algorithm.to_owned();
        }
        let input = knuth_shuffle(gen_u32_vec(sort_config.input_len));
        let output = (sort_config.sorting_algorithm.run)(input.clone());
        let active_step_index = 0;

        let _history_listener = {
            let sort_config = sort_config.clone();
            _ctx.link()
                .add_history_listener(_ctx.link().callback(move |history: AnyHistory| {
                    let algorithm_name = match history
                        .location()
                        .route::<SortingAlgorithmsRoute>()
                        .unwrap()
                    {
                        SortingAlgorithmsRoute::SortingAlgorithm { algorithm } => algorithm,
                        _ => "bubble-sort".to_string(),
                    };
                    if let Some(algorithm) = algorithms.get(algorithm_name.as_str()) {
                        Msg::UpdateConfig(
                            SortConfig {
                                sorting_algorithm: algorithm.to_owned(),
                                ..sort_config.clone()
                            },
                            true,
                        )
                    } else {
                        Msg::UpdateConfig(sort_config.clone(), false)
                    }
                }))
                .unwrap()
        };

        SortingAlgorithms {
            input,
            output: SortResult::new(output.output, output.duration, output.steps.clone()),
            sort_config,
            steps: output.steps,
            active_step_index,
            _history_listener,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(val) => {
                self.input = val;
                self.update_values();
                true
            }
            Msg::UpdateConfig(val, rerender) => {
                self.sort_config = val;
                if rerender {
                    self.update_values();
                }
                rerender
            }
            Msg::ChangeActiveStep(res) => {
                if let Ok(val) = res {
                    self.active_step_index = val;
                    return true;
                }
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let change_active_step = ctx.link().callback(|e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            Msg::ChangeActiveStep(el.value().parse::<usize>())
        });
        let update_input = ctx.link().callback(Msg::UpdateInput);
        let update_config = ctx
            .link()
            .callback(|msg: (SortConfig, bool)| Msg::UpdateConfig(msg.0, msg.1));

        html! {
            <div id="SortingAlgorithms">
                <h1>{"Sorting algorithms"}</h1>

                <SortControls config={self.sort_config.clone()} {update_input} {update_config} />

                <div class="content">
                    <h2>{ format!("{} steps, {}", self.steps.len(), self.get_sort_duration_ms()) }</h2>

                    {
                        if self.active_step_index == 0 {
                            html! { <SortGraph items={self.input.clone()} /> }
                        } else {
                            html! {
                                <SortGraph
                                    items={self.get_output_at_step_index(self.active_step_index)}
                                    step={self.steps[self.active_step_index - 1].clone()}
                                    audio_enabled={self.sort_config.audio_enabled}
                                />
                            }
                        }
                    }

                    <div class="step-selector">
                        <label for="active-step-input">
                            {
                                if self.active_step_index == 0 {
                                    format!("Step: {} (input)", self.active_step_index)
                                } else {
                                    format!("Step: {}", self.active_step_index)
                                }
                            }
                        </label>
                        <input
                            type="range"
                            id="active-step-input"
                            min="0"
                            max={(self.steps.len()).to_string()}
                            value={self.active_step_index.to_string()}
                            oninput={change_active_step}
                        />
                    </div>
                </div>

                <SortDesc url={self.get_sort_desc_url()} />
            </div>
        }
    }
}

impl SortingAlgorithms {
    fn update_values(&mut self) {
        self.input = knuth_shuffle(gen_u32_vec(self.sort_config.input_len));
        let output = (self.sort_config.sorting_algorithm.run)(self.input.clone());
        self.output = SortResult::new(output.output, output.duration, output.steps.clone());
        self.steps = output.steps;
        self.active_step_index = 0;
    }
    /// Gets the output at a given step's index by running [`SortCommand`]s on the input.
    fn get_output_at_step_index(&self, index: usize) -> Vec<u32> {
        let mut output = self.input.to_vec();
        run_sort_steps(&mut output, self.steps[0..index].to_vec());
        output
    }
    fn get_sort_duration_ms(&self) -> String {
        format!("{:?} ms", &self.output.duration.unwrap().as_millis())
    }
    fn get_sort_desc_url(&self) -> String {
        let origin = window().unwrap().location().origin().unwrap();
        format!(
            "{}/src/{}/README.md",
            origin,
            self.sort_config
                .sorting_algorithm
                .name
                .to_lowercase()
                .replace(" ", "_")
        )
    }
}
