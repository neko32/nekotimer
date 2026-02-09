use web_sys::HtmlInputElement;
use yew::prelude::*;

use nekotimer_shared::{CountdownBlock, TimerBlock};

#[derive(Properties, PartialEq)]
pub struct CountdownBlockProps {
    pub block: CountdownBlock,
    pub on_change: Callback<TimerBlock>,
}

#[function_component(CountdownBlockEditor)]
pub fn countdown_block_editor(props: &CountdownBlockProps) -> Html {
    let block = props.block.clone();
    let on_change = props.on_change.clone();

    let make_handler = |field: &'static str| {
        let block = block.clone();
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut b = block.clone();
            match field {
                "name" => b.name = input.value(),
                "minutes" => b.minutes = input.value().parse().unwrap_or(0),
                "seconds" => b.seconds = input.value().parse().unwrap_or(0),
                "repeat_count" => b.repeat_count = input.value().parse().unwrap_or(0),
                "interval_minutes" => b.interval_minutes = input.value().parse().unwrap_or(0),
                "interval_seconds" => b.interval_seconds = input.value().parse().unwrap_or(0),
                _ => {}
            }
            on_change.emit(TimerBlock::Countdown(b));
        })
    };

    html! {
        <div class="block-fields">
            <div class="block-field">
                <label>{"名前"}</label>
                <input type="text" value={block.name.clone()} oninput={make_handler("name")}
                    placeholder="ブロック名" />
            </div>
            <div class="block-field-row">
                <div class="block-field">
                    <label>{"分"}</label>
                    <input type="number" value={block.minutes.to_string()} oninput={make_handler("minutes")}
                        min="0" max="1440" />
                </div>
                <div class="block-field">
                    <label>{"秒"}</label>
                    <input type="number" value={block.seconds.to_string()} oninput={make_handler("seconds")}
                        min="1" max="59" />
                </div>
            </div>
            <div class="block-field">
                <label>{"繰り返し回数"}</label>
                <input type="number" value={block.repeat_count.to_string()} oninput={make_handler("repeat_count")}
                    min="0" max="100" />
            </div>
            <div class="block-field-row">
                <div class="block-field">
                    <label>{"インターバル分"}</label>
                    <input type="number" value={block.interval_minutes.to_string()} oninput={make_handler("interval_minutes")}
                        min="0" max="1440" />
                </div>
                <div class="block-field">
                    <label>{"インターバル秒"}</label>
                    <input type="number" value={block.interval_seconds.to_string()} oninput={make_handler("interval_seconds")}
                        min="0" max="59" />
                </div>
            </div>
        </div>
    }
}
