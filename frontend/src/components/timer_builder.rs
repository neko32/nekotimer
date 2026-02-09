use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::block_canvas::BlockCanvas;
use crate::services::api;
use crate::state::{AppAction, AppStateContext};

#[function_component(TimerBuilder)]
pub fn timer_builder() -> Html {
    let state = use_context::<AppStateContext>().expect("no context found");

    let timer = match &state.editing_timer {
        Some(t) => t.clone(),
        None => return html! {},
    };

    let on_name_change = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.dispatch(AppAction::SetTimerName(input.value()));
        })
    };

    let on_save = {
        let state = state.clone();
        let timer = timer.clone();
        Callback::from(move |_: MouseEvent| {
            let state = state.clone();
            let timer = timer.clone();
            spawn_local(async move {
                if let Err(errors) = timer.validate() {
                    let error_messages: Vec<String> =
                        errors.iter().map(|e| e.message.clone()).collect();
                    state.dispatch(AppAction::SetValidationErrors(error_messages));
                    return;
                }

                let result = if timer.id.is_empty() {
                    api::create_timer(&timer).await
                } else {
                    api::update_timer(&timer.id, &timer).await
                };

                match result {
                    Ok(saved) => {
                        state.dispatch(AppAction::SaveSuccess(saved));
                    }
                    Err(errors) => {
                        state.dispatch(AppAction::SetValidationErrors(errors));
                    }
                }
            });
        })
    };

    html! {
        <div class="timer-builder">
            <h2 class="builder-title">
                if timer.id.is_empty() {
                    {"新規タイマー作成"}
                } else {
                    {"タイマー編集"}
                }
            </h2>
            <div class="builder-name-section">
                <label for="timer-name">{"タイマー名"}</label>
                <input
                    id="timer-name"
                    class="timer-name-input"
                    type="text"
                    placeholder="タイマー名を入力..."
                    value={timer.name.clone()}
                    oninput={on_name_change}
                    maxlength="64"
                />
            </div>

            <BlockCanvas />

            if !state.validation_errors.is_empty() {
                <div class="validation-errors">
                    <ul>
                        { for state.validation_errors.iter().map(|e| html! {
                            <li>{ e }</li>
                        })}
                    </ul>
                </div>
            }

            <button class="btn btn-primary save-btn" onclick={on_save}>{"Save"}</button>
        </div>
    }
}
