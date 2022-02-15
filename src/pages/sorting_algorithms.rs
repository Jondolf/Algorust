use crate::components::{
    collapsible::Collapsible, sort_controls::SortControls, sort_graph::SortGraph,
};
use crate::utils::gen_i32_vec;
use sorting_algorithms::*;
use std::num::ParseIntError;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SortingAlgorithm<T: Clone + Copy + PartialEq + PartialOrd> {
    pub name: &'static str,
    pub sort: fn(Vec<T>) -> SortResult<T>,
}

pub const SORTING_ALGORITHMS: [SortingAlgorithm<i32>; 3] = [
    SortingAlgorithm {
        name: "Bubble sort",
        sort: bubble_sort::sort,
    },
    SortingAlgorithm {
        name: "Insertion sort",
        sort: insertion_sort::sort,
    },
    SortingAlgorithm {
        name: "Merge sort",
        sort: merge_sort::sort,
    },
];

pub enum Msg {
    UpdateInput(Vec<i32>),
    UpdateConfig(SortConfig),
    ChangeActiveStep(Result<usize, ParseIntError>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SortConfig {
    pub input_len: usize,
    pub min_val: isize,
    pub max_val: isize,
    pub sorting_algorithm: SortingAlgorithm<i32>,
}
impl Default for SortConfig {
    fn default() -> Self {
        Self {
            input_len: 20,
            min_val: 0,
            max_val: 99,
            sorting_algorithm: SORTING_ALGORITHMS[0].clone(),
        }
    }
}

pub struct SortingAlgorithms {
    input: Vec<i32>,
    output: SortResult<i32>,
    sort_config: SortConfig,
    steps: Vec<Step<i32>>,
    active_step_index: usize,
}

impl Component for SortingAlgorithms {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let sort_config = SortConfig::default();
        let input = gen_i32_vec(
            sort_config.input_len,
            sort_config.min_val,
            sort_config.max_val,
        );
        let output = (sort_config.sorting_algorithm.sort)(input.clone());
        let active_step = output.steps.len() - 1;
        SortingAlgorithms {
            input,
            output: SortResult::new(output.value, output.duration, output.steps.clone()),
            sort_config,
            steps: output.steps,
            active_step_index: active_step,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(val) => {
                self.input = val;
                self.update_values();
                true
            }
            Msg::UpdateConfig(val) => {
                self.sort_config = val;
                self.update_values();
                true
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
        let active_step = &self.steps[self.active_step_index];
        let input_str = &self
            .input
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let step_output_str = active_step
            .values
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let sort_duration = format!(
            "{:?} ms",
            match &self.output.duration {
                Some(dur) => dur.as_millis(),
                None => 0,
            }
        );
        let change_active_step = ctx.link().callback(|e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            Msg::ChangeActiveStep(el.value().parse::<usize>())
        });
        let update_input = ctx.link().callback(|val| Msg::UpdateInput(val));
        let update_config = ctx
            .link()
            .callback(|val: SortConfig| Msg::UpdateConfig(val));

        html! {
            <div id="SortingAlgorithms">
                <h1>{"Sorting algorithms"}</h1>

                <SortControls config={self.sort_config.clone()} {update_input} {update_config} />

                <div class="content">
                    <div class="input-container">
                <h2>{"Input"}</h2>

                        <Collapsible open={true} title={"Input graph"}>
                            <SortGraph step={Step::new(self.input.clone(), vec![])} />
                        </Collapsible>

                <Collapsible open={true} title={"Input values"}>
                    <pre>{ input_str }</pre>
                </Collapsible>
                    </div>

                    <div class="output-container">
                        <h2>{ format!("Output ({} steps, {})", self.steps.len(), sort_duration) }</h2>

                    <Collapsible open={true} title={"Output graph"}>
                        <SortGraph step={active_step.clone()} />
                    </Collapsible>

                    <Collapsible open={true} title={"Output values"}>
                        <pre>{ step_output_str }</pre>
                    </Collapsible>

                        <div class="step-selector">
                            <label for="active-step-input">
                                { format!("Step: {}", self.active_step_index) }
                            </label>
                            <input type="range" id="active-step-input" min="0" max={(self.steps.len() - 1).to_string()} value={self.active_step_index.to_string()} oninput={change_active_step} />
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl SortingAlgorithms {
    fn update_values(&mut self) {
        self.input = self.generate_values();
        let output = (self.sort_config.sorting_algorithm.sort)(self.input.clone());
        self.output = SortResult::new(output.value, output.duration, output.steps.clone());
        self.steps = output.steps;

        if self.active_step_index >= self.steps.len() {
            self.active_step_index = self.steps.len() - 1;
        }
    }

    fn generate_values(&self) -> Vec<i32> {
        gen_i32_vec(
            self.sort_config.input_len,
            self.sort_config.min_val,
            self.sort_config.max_val,
        )
    }
}
