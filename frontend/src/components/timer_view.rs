use yew::prelude::*;
use nekotimer_shared::{CountdownBlock, TimerBlock, TimerConfig, WaitBlock};

#[derive(Properties, Clone, PartialEq)]
pub struct TimerViewProps {
    pub timer: TimerConfig,
}

fn format_wait_line(w: &WaitBlock) -> String {
    format!("{}分{}秒 待機", w.minutes, w.seconds)
}

fn format_countdown_line(c: &CountdownBlock) -> String {
    format!(
        "{}分{}秒 カウントダウン、{}回繰り返し (インターバル待機: {}分{}秒)",
        c.minutes, c.seconds, c.repeat_count, c.interval_minutes, c.interval_seconds
    )
}

#[function_component(TimerView)]
pub fn timer_view(props: &TimerViewProps) -> Html {
    let state = use_context::<crate::state::AppStateContext>().expect("no context found");

    let on_edit = {
        let id = props.timer.id.clone();
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(crate::state::AppAction::EditTimer(id.clone()));
        })
    };

    html! {
        <div class="timer-view">
            <h2 class="timer-view-title">{ &props.timer.name }</h2>
            <div class="timer-view-blocks">
                { for props.timer.blocks.iter().map(|block| {
                    html! {
                        <div class="timer-view-block">
                            <div class="timer-view-block-type-row">
                                <span class="timer-view-block-type">{ block_type_name(block) }</span>
                                <span class="timer-view-block-name">{ block_name(block) }</span>
                            </div>
                            <div class="timer-view-block-detail">
                                { block_detail_line(block) }
                            </div>
                        </div>
                    }
                })}
            </div>
            <div class="timer-view-actions">
                <button class="btn btn-primary" onclick={on_edit}>{"編集する"}</button>
            </div>
        </div>
    }
}

fn block_type_name(block: &TimerBlock) -> &'static str {
    match block {
        TimerBlock::Wait(_) => "待機",
        TimerBlock::Countdown(_) => "カウントダウン",
    }
}

fn block_name(block: &TimerBlock) -> &str {
    match block {
        TimerBlock::Wait(w) => w.name.as_str(),
        TimerBlock::Countdown(c) => c.name.as_str(),
    }
}

fn block_detail_line(block: &TimerBlock) -> String {
    match block {
        TimerBlock::Wait(w) => format_wait_line(w),
        TimerBlock::Countdown(c) => format_countdown_line(c),
    }
}
