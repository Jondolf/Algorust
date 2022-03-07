use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    // Todo: Make the sidebar resizable via a handle
    html! {
        <div class="sidebar">
            { for props.children.iter() }
        </div>
    }
}
