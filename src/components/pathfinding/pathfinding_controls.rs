use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::{
    components::input_items::*,
    pages::pathfinding::{
        get_pathfinding_algorithms, PathfindingConfig, PathfindingConfigUpdate, PathfindingRoute,
    },
};

#[derive(Properties, Clone, PartialEq)]
pub struct PathfindingControlsProps {
    pub config: PathfindingConfig,
    pub update_config: Callback<(PathfindingConfig, PathfindingConfigUpdate)>,
}

#[function_component(PathfindingControls)]
pub fn pathfinding_controls(props: &PathfindingControlsProps) -> Html {
    let PathfindingControlsProps {
        config,
        update_config,
    } = props.clone();

    let history = use_history().unwrap();

    let algorithm_names = use_state_eq(|| {
        get_pathfinding_algorithms()
            .values()
            .map(|algorithm| algorithm.name.to_string())
            .collect::<Vec<String>>()
    });

    let change_graph_width = {
        let config = config.clone();
        let update_config = update_config.clone();

        Callback::from(move |graph_width| {
            if graph_width > 1 && graph_width != config.graph_width {
                update_config.emit((
                    PathfindingConfig {
                        graph_width,
                        ..config.clone()
                    },
                    PathfindingConfigUpdate::UpdatePathAndGraph,
                ));
            }
        })
    };

    let change_graph_height = {
        let config = config.clone();
        let update_config = update_config.clone();

        Callback::from(move |graph_height| {
            if graph_height > 1 && graph_height != config.graph_height {
                update_config.emit((
                    PathfindingConfig {
                        graph_height,
                        ..config.clone()
                    },
                    PathfindingConfigUpdate::UpdatePathAndGraph,
                ));
            }
        })
    };

    let toggle_move_diagonally = {
        let config = config.clone();

        Callback::from(move |_| {
            update_config.emit((
                PathfindingConfig {
                    move_diagonally: !config.move_diagonally,
                    ..config.clone()
                },
                PathfindingConfigUpdate::UpdatePathAndGraph,
            ));
        })
    };

    let change_algorithm = Callback::from(move |algorithm: String| {
        history.push(PathfindingRoute::PathfindingAlgorithm {
            algorithm: algorithm.replace(' ', "-").to_lowercase(),
        });
    });

    html! {
        <div class="pathfind-controls">
            <SelectInput
                title="Algorithm"
                options={(*algorithm_names).clone()}
                selected_value={config.algorithm.name}
                onchange={change_algorithm}
            />
            <IntInput<usize>
                title="Graph width"
                value={props.config.graph_width}
                oninput={change_graph_width}
                min={2}
            />
            <IntInput<usize>
                title="Graph height"
                value={props.config.graph_height}
                oninput={change_graph_height}
                min={2}
            />
            <Checkbox title="Move diagonally" value={config.move_diagonally} oninput={toggle_move_diagonally} />
        </div>
    }
}
