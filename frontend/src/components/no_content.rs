use yew::prelude::*;

#[function_component(NoContent)]
pub fn no_content() -> Html {
    html! {
        <div class="no-content">
            <p>{"サイドバーの「Add」ボタンを押してタイマーを作成してください"}</p>
        </div>
    }
}
