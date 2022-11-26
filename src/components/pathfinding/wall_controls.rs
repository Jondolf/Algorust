use yew::prelude::*;

use crate::components::input_items::*;

#[derive(Properties, Clone, PartialEq)]
pub struct WallControlsProps {
    pub on_clear_walls: Callback<()>,
    pub on_generate_maze: Callback<()>,
}

#[function_component]
pub fn WallControls(props: &WallControlsProps) -> Html {
    let WallControlsProps {
        on_clear_walls,
        on_generate_maze,
    } = props.clone();

    let on_clear_walls = Callback::from(move |_| {
        on_clear_walls.emit(());
    });

    let on_generate_maze = Callback::from(move |_| {
        on_generate_maze.emit(());
    });

    html! {
        <div class="wall-controls">
            <Button title="Clear walls" onclick={on_clear_walls} />
            <Button title="Generate maze" onclick={on_generate_maze} />
        </div>
    }
}
