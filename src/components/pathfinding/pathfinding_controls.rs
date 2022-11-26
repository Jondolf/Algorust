use std::{cell::RefCell, rc::Rc};

use pathfinding::{Coord, Edge};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::input_items::*,
    pages::pathfinding::{
        get_pathfinding_algorithms, PathfindingAlgorithm, PathfindingConfig, PathfindingRoute,
    },
};

#[derive(Properties, Clone, PartialEq)]
pub struct PathfindingControlsProps<E: 'static + Edge> {
    pub config: Rc<RefCell<PathfindingConfig<E>>>,
    pub on_update_config: Callback<()>,
}

#[function_component]
pub fn PathfindingControls<E: 'static + Edge>(props: &PathfindingControlsProps<E>) -> Html {
    let PathfindingControlsProps {
        config,
        on_update_config,
    } = props.clone();

    let navigator = use_navigator().unwrap();

    let algorithm_names = use_state_eq(|| {
        get_pathfinding_algorithms()
            .values()
            .map(|algorithm: &PathfindingAlgorithm<Coord, E>| algorithm.name.to_string())
            .collect::<Vec<String>>()
    });

    let change_graph_width = {
        let config = config.clone();
        let on_update_config = on_update_config.clone();

        Callback::from(move |graph_width| {
            if graph_width > 1 && graph_width != config.borrow().graph_width {
                config.borrow_mut().graph_width = graph_width;
                on_update_config.emit(());
            }
        })
    };

    let change_graph_height = {
        let config = config.clone();
        let on_update_config = on_update_config.clone();

        Callback::from(move |graph_height| {
            if graph_height > 1 && graph_height != config.borrow().graph_height {
                config.borrow_mut().graph_height = graph_height;
                on_update_config.emit(());
            }
        })
    };

    let toggle_move_diagonally = {
        let config = config.clone();

        Callback::from(move |_| {
            let move_diagonally = config.borrow().move_diagonally;
            config.borrow_mut().move_diagonally = !move_diagonally;
            on_update_config.emit(());
        })
    };

    let change_playback_time = {
        let config = config.clone();

        Callback::from(move |playback_time| {
            config.borrow_mut().playback_time = playback_time;
        })
    };

    let change_algorithm = Callback::from(move |algorithm: String| {
        navigator.push(&PathfindingRoute::PathfindingAlgorithm {
            algorithm: algorithm.replace(' ', "-").to_lowercase(),
        });
    });

    html! {
        <div class="pathfind-controls">
            <SelectInput
                title="Algorithm"
                options={(*algorithm_names).clone()}
                selected_value={config.borrow().algorithm.name.to_string()}
                onchange={change_algorithm}
            />
            <IntInput<usize>
                title="Graph width"
                value={config.borrow().graph_width}
                oninput={change_graph_width}
                min={2}
            />
            <IntInput<usize>
                title="Graph height"
                value={config.borrow().graph_height}
                oninput={change_graph_height}
                min={2}
            />
            <FloatInput<f32>
                title="Playback time (seconds)"
                value={config.borrow().playback_time}
                oninput={change_playback_time}
                min={0.0}
            />
            <Checkbox title="Move diagonally" value={config.borrow().move_diagonally} oninput={toggle_move_diagonally} />
        </div>
    }
}
