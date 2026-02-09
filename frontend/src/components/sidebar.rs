use yew::prelude::*;
use crate::state::{AppAction, AppStateContext};

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let state = use_context::<AppStateContext>().expect("no context found");

    let on_add = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(AppAction::StartNewTimer);
        })
    };

    let on_execute = {
        Callback::from(move |_: MouseEvent| {
            gloo_dialogs::alert("タイマー実行予定");
        })
    };

    let has_saved = state.last_saved_id.is_some();

    html! {
        <div class="sidebar">
            <h2>{"nekotimer"}</h2>
            <ul class="sidebar-timer-list">
                { for state.timers.iter().map(|timer| {
                    let id = timer.id.clone();
                    let is_active = state.editing_timer
                        .as_ref()
                        .map(|t| t.id == timer.id)
                        .unwrap_or(false);
                    let class = if is_active {
                        "sidebar-timer-item active"
                    } else {
                        "sidebar-timer-item"
                    };
                    let state = state.clone();
                    let onclick = Callback::from(move |_: MouseEvent| {
                        state.dispatch(AppAction::EditTimer(id.clone()));
                    });
                    html! {
                        <li {class} {onclick}>
                            { &timer.name }
                        </li>
                    }
                })}
            </ul>
            <div class="sidebar-actions">
                <button class="btn btn-add" onclick={on_add}>{"Add"}</button>
                if has_saved {
                    <button class="btn btn-success" onclick={on_execute}>{"実行"}</button>
                }
            </div>
        </div>
    }
}
