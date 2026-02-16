use yew::prelude::*;
use crate::components::no_content::NoContent;
use crate::components::timer_builder::TimerBuilder;
use crate::components::timer_view::TimerView;
use crate::state::{AppStateContext, ViewMode};

#[function_component(ContentPane)]
pub fn content_pane() -> Html {
    let state = use_context::<AppStateContext>().expect("no context found");

    let body = match &state.view {
        ViewMode::NoContent => html! { <NoContent /> },
        ViewMode::Builder => html! { <TimerBuilder /> },
        ViewMode::ViewTimer(id) => {
            if let Some(timer) = state.timers.iter().find(|t| t.id == *id) {
                html! { <TimerView timer={timer.clone()} /> }
            } else {
                html! { <NoContent /> }
            }
        }
    };

    html! {
        <div class="content-pane">
            { body }
        </div>
    }
}
