extern crate sorting_algorithms;
use rand::Rng;
use sorting_algorithms::*;
use yew::prelude::*;

enum Msg {
    ChangeInputLen(u32),
    Geni32,
}

struct App {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    values: Vec<i32>,
    input_len: u32,
    output: SortResult<i32>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let values = gen_i32_vec(10, -999, 999);
        App {
            link,
            values: values.clone(),
            input_len: 10,
            output: bubble_sort::sort(&values),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeInputLen(len) => {
                if self.input_len != len {
                    self.input_len = len;
                    self.values = gen_i32_vec(self.input_len, 0, 999);
                    self.output = bubble_sort::sort(&self.values);
                    return true;
                }
                false
            }
            Msg::Geni32 => {
                self.values = gen_i32_vec(self.input_len, 0, 999);
                self.output = bubble_sort::sort(&self.values);
                true
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
    fn view(&self) -> Html {
        html! {
            <div id="app">
                <h1>{"Sorting algorithms"}</h1>
                <button onclick=self.link.callback(|_| Msg::Geni32)>{"Generate list of i32"}</button>
                <input type="number" placeholder="Input length" oninput=self.link.callback(|e: InputData| Msg::ChangeInputLen(e.value.parse::<u32>().unwrap())) />
                <pre>{format!("List to sort:\n{}", &self.values.iter().map(|val| val.to_string()).collect::<Vec<String>>().join(", "))}</pre>
                <pre>
                {
                    format!("Result:\n{}", &self.output.arr.iter().map(|val| val.to_string()).collect::<Vec<String>>().join(", "))
                }
                </pre>
                <pre>
                {
                    format!("{} steps:\n{}", &self.output.steps.len(), &self.output.steps.iter().map(|step| step.iter().map(|val| val.to_string()).collect::<Vec<String>>().join(", ")).collect::<Vec<String>>().join("\n"))
                }
                </pre>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

fn gen_i32_vec(len: u32, min: i32, max: i32) -> Vec<i32> {
    let mut vec = vec![];
    for _ in 0..len {
        vec.push(rand::thread_rng().gen_range(min..max));
    }
    vec
}
