use yew::prelude::*;
use crate::state::{AppAction, AppStateContext};

#[function_component(UnsavedChangesModal)]
pub fn unsaved_changes_modal() -> Html {
    let state = use_context::<AppStateContext>().expect("no context found");

    let on_continue = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(AppAction::CancelNavigate);
        })
    };

    let on_discard = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(AppAction::ConfirmDiscard);
        })
    };

    html! {
        <div class="modal-overlay" role="dialog" aria-modal="true" aria-labelledby="unsaved-modal-title">
            <div class="modal-box unsaved-modal">
                <h2 id="unsaved-modal-title" class="modal-title">{"未保存の変更があります"}</h2>
                <p class="modal-message">
                    {"編集内容が保存されていません。編集を続けますか？それとも変更を破棄して画面を切り替えますか？"}
                </p>
                <div class="modal-actions">
                    <button type="button" class="btn btn-primary" onclick={on_continue}>
                        {"編集を続ける"}
                    </button>
                    <button type="button" class="btn btn-outline-danger" onclick={on_discard}>
                        {"編集を止める"}
                    </button>
                </div>
            </div>
        </div>
    }
}
