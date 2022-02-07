use crate::components::{
    collapsible::Collapsible, sort_controls::SortControls, sort_visual::SortGraph,
};
use crate::utils::gen_i32_vec;
use sorting_algorithms::*;
use std::num::ParseIntError;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SortingAlgorithm<T: Clone> {
    pub name: &'static str,
    pub sort: fn(Vec<T>) -> SortResult<T>,
}
impl<T: Clone> SortingAlgorithm<T> {
    fn new(name: &'static str, sort: fn(Vec<T>) -> SortResult<T>) -> Self {
        Self { name, sort }
    }
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
            sorting_algorithm: SortingAlgorithm::new("Bubble sort", bubble_sort::sort),
        }
    }
}

pub struct SortingAlgorithms {
    input: Vec<i32>,
    output: SortResult<i32>,
    sort_config: SortConfig,
    steps: Vec<Vec<i32>>,
    active_step: usize,
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
            active_step,
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
                    if self.active_step != val {
                        self.active_step = val;
                        return true;
                    }
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
        let update_input = ctx.link().callback(|val| Msg::UpdateInput(val));
        let update_config = ctx
            .link()
            .callback(|val: SortConfig| Msg::UpdateConfig(val));

        html! {
            <div id="SortingAlgorithms">
                <h1>{"Sorting algorithms"}</h1>

                <h2>{"Input"}</h2>

                <SortControls config={self.sort_config.clone()} {update_input} {update_config} />

                <Collapsible open={true} title={"Input value"}>
                    <pre>{ &self.input.iter().map(|val| val.to_string()).collect::<Vec<String>>().join(", ") }</pre>
                </Collapsible>

                <Collapsible open={false} title={"Input graph"}>
                    <SortGraph values={self.input.clone()} />
                </Collapsible>

                <div class={classes!("sort-visualizations")}>
                    <h2>{ format!("Output ({} steps)", self.steps.len()) }</h2>
                    <p>
                    {
                        format!("Sort duration: {:?} ms", match &self.output.duration {
                            Some(dur) => dur.as_millis(),
                            None => 10
                        })
                    }
                    </p>

                    <label for="active-step-input">{ format!("Step: {}", self.active_step + 1) }</label>
                    <input type="range" id="active-step-input" min="0" max={(self.steps.len()-1).to_string()} value={self.active_step.to_string()} oninput={change_active_step} />

                    <Collapsible open={true} title={"Output graph"}>
                        <SortGraph values={self.steps[self.active_step].clone()} />
                    </Collapsible>

                    <Collapsible open={true} title={"Output values"}>
                        <pre>{ &self.steps[self.active_step].iter().map(|val| val.to_string()).collect::<Vec<String>>().join(", ") }</pre>
                    </Collapsible>
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

        if self.active_step >= self.steps.len() {
            self.active_step = self.steps.len() - 1;
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
