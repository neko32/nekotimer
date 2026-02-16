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

    html! {
        <div class="sidebar">
            <h2>{"nekotimer"}</h2>
            <ul class="sidebar-timer-list">
                { for state.timers.iter().map(|timer| {
                    let id = timer.id.clone();
                    let timer_name = timer.name.clone();
                    let is_active = state.editing_timer
                        .as_ref()
                        .map(|t| t.id == timer.id)
                        .unwrap_or(false);
                    let can_execute = !timer.id.is_empty();
                    let class = if is_active {
                        "sidebar-timer-item active"
                    } else {
                        "sidebar-timer-item"
                    };
                    let state_for_select = state.clone();
                    let state_for_enter = state.clone();
                    let id_for_select = id.clone();
                    let id_for_preview = id.clone();
                    let on_select = Callback::from(move |_: MouseEvent| {
                        state_for_select.dispatch(AppAction::EditTimer(id_for_select.clone()));
                    });
                    let on_mouse_enter = Callback::from(move |_: MouseEvent| {
                        state_for_enter.dispatch(AppAction::PreviewTimer(Some(id_for_preview.clone())));
                    });
                    let on_execute = Callback::from(move |e: MouseEvent| {
                        e.stop_propagation();
                        gloo_dialogs::alert(&format!("タイマー実行予定: {}", id));
                    });
                    html! {
                        <li
                            {class}
                            onclick={on_select}
                            onmouseenter={on_mouse_enter}
                        >
                            <span class="sidebar-timer-name">{ &timer_name }</span>
                            if can_execute {
                                <button
                                    class="btn btn-success btn-run"
                                    onclick={on_execute}
                                    title="実行"
                                >
                                    {"実行"}
                                </button>
                            }
                        </li>
                    }
                })}
            </ul>
            <div class="sidebar-actions">
                <button class="btn btn-add" onclick={on_add}>{"Add"}</button>
            </div>
        </div>
    }
}
