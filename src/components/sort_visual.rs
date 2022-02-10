use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SortGraphProps {
    pub values: Vec<i32>,
}

#[function_component(SortGraph)]
pub fn sort_visual(props: &SortGraphProps) -> Html {
    let max = match props.values.iter().max() {
        Some(val) => {
            let val = val.clone() as f64;
            if val == 0.0 {
                100.0
            } else {
                val
            }
        }
        None => 100.0,
    };
    let margin = 50.0 / props.values.len() as f64;
    let bar_nodes = props
        .values
        .iter()
        .map(|val| {
            let width = format!("{:?}%", 100.0 / props.values.len() as f64);
            let height = format!("{:?}%", (val.clone() as f64 / max) * 100.0);
            let margin = format!("{:?}px", if margin < 0.3 { 0.0 } else { margin });

            html! {
                <div style={format!("width: {}; height: {}; margin: 0 {}", width, height, margin)}></div>
            }
        })
        .collect::<Html>();

    html! {
        <div class={classes!("sort-visualization")}>
            { bar_nodes }
        </div>
    }
}
