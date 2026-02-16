use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::content_pane::ContentPane;
use crate::components::sidebar::Sidebar;
use crate::components::running_timer_modal::RunningTimerModal;
use crate::components::unsaved_changes_modal::UnsavedChangesModal;
use crate::services::api;
use crate::services::timer_runner;
use crate::state::{AppAction, AppState, AppStateContext};

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(AppState::default);

    {
        use_effect_with((), |_| {
            timer_runner::init_audio_cache();
            || ()
        });
    }
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

    let show_unsaved_modal = state.pending_navigation.is_some();
    let show_running_modal = state.running.is_some();

    html! {
        <ContextProvider<AppStateContext> context={state}>
            <div class="app-container">
                <Sidebar />
                <ContentPane />
                if show_unsaved_modal {
                    <UnsavedChangesModal />
                }
                if show_running_modal {
                    <RunningTimerModal />
                }
            </div>
        </ContextProvider<AppStateContext>>
    }
}
