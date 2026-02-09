use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::content_pane::ContentPane;
use crate::components::sidebar::Sidebar;
use crate::services::api;
use crate::state::{AppAction, AppState, AppStateContext};

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(AppState::default);

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::fetch_timers().await {
                    Ok(timers) => state.dispatch(AppAction::SetTimers(timers)),
                    Err(e) => log::error!("Failed to fetch timers: {}", e),
                }
            });
            || ()
        });
    }

    html! {
        <ContextProvider<AppStateContext> context={state}>
            <div class="app-container">
                <Sidebar />
                <ContentPane />
            </div>
        </ContextProvider<AppStateContext>>
    }
}
