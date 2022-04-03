use yew::prelude::*;

/// Different drawing modes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathTool {
    Start,
    End,
    Wall,
}

#[derive(Clone, PartialEq)]
struct PathToolButton {
    tool: PathTool,
    description: String,
    icon: Html,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PathToolbarProps {
    #[prop_or(PathTool::Wall)]
    pub active_tool: PathTool,
    pub on_tool_change: Callback<PathTool>,
}

#[function_component(PathToolbar)]
pub fn path_toolbar(props: &PathToolbarProps) -> Html {
    let tool_buttons = use_state_eq(|| {
        vec![
            PathToolButton {
                tool: PathTool::Wall,
                description: "Draw walls".to_string(),
                icon: html! { { "▣" } },
            },
            PathToolButton {
                tool: PathTool::Start,
                description: "Set the path's starting point".to_string(),
                icon: html! { <span style="color: var(--color-accent)">{ "▣" }</span> },
            },
            PathToolButton {
                tool: PathTool::End,
                description: "Set the path's end point".to_string(),
                icon: html! { <span style="color: orangered">{ "▣" }</span> },
            },
        ]
    });

    html! {
        <div class="path-toolbar">
            {
                (*tool_buttons).clone().into_iter().map(|PathToolButton { tool, description, icon }| {
                    let on_tool_change = props.on_tool_change.clone();
                    let active_tool = props.active_tool;

                    html! {
                        <button
                            class={classes!(
                                "toolbar-button",
                                if tool == active_tool { "active" } else { "" }
                            )}
                            onclick={move |_| on_tool_change.emit(tool)}
                            title={description}>
                            { icon }
                        </button>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
