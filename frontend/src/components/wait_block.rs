use web_sys::HtmlInputElement;
use yew::prelude::*;

use nekotimer_shared::{TimerBlock, WaitBlock};

#[derive(Properties, PartialEq)]
pub struct WaitBlockProps {
    pub block: WaitBlock,
    pub on_change: Callback<TimerBlock>,
}

#[function_component(WaitBlockEditor)]
pub fn wait_block_editor(props: &WaitBlockProps) -> Html {
    let block = props.block.clone();
    let on_change = props.on_change.clone();

    let on_name = {
        let block = block.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut b = block.clone();
            b.name = input.value();
            on_change.emit(TimerBlock::Wait(b));
        })
    };

    let on_minutes = {
        let block = block.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut b = block.clone();
            b.minutes = input.value().parse().unwrap_or(0);
            on_change.emit(TimerBlock::Wait(b));
        })
    };

    let on_seconds = {
        let block = block.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut b = block.clone();
            b.seconds = input.value().parse().unwrap_or(0);
            on_change.emit(TimerBlock::Wait(b));
        })
    };

    html! {
        <div class="block-fields">
            <div class="block-field">
                <label>{"名前"}</label>
                <input type="text" value={block.name.clone()} oninput={on_name} placeholder="ブロック名" />
            </div>
            <div class="block-field">
                <label>{"分"}</label>
                <input type="number" value={block.minutes.to_string()} oninput={on_minutes}
                    min="0" max="1440" />
            </div>
            <div class="block-field">
                <label>{"秒"}</label>
                <input type="number" value={block.seconds.to_string()} oninput={on_seconds}
                    min="1" max="59" />
            </div>
        </div>
    }
}
