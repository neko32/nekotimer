use yew::prelude::*;
use crate::components::no_content::NoContent;
use crate::components::timer_builder::TimerBuilder;
use crate::state::{AppStateContext, ViewMode};

#[function_component(ContentPane)]
pub fn content_pane() -> Html {
    let state = use_context::<AppStateContext>().expect("no context found");

    html! {
        <div class="content-pane">
            {
                match state.view {
                    ViewMode::NoContent => html! { <NoContent /> },
                    ViewMode::Builder => html! { <TimerBuilder /> },
                }
            }
        </div>
    }
}
