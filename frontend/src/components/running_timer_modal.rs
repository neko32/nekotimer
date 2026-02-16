use yew::prelude::*;

use nekotimer_shared::TimerBlock;

use crate::state::{AppAction, AppStateContext, CountdownPhase};

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

/// ブロックの設定データ表示用。ウェイト: xx分yy秒 ウェイト。カウントダウン: xx分yy秒 カウントダウン (繰り返しz回, インターバル aa分yy秒)
fn block_config_line(block: &TimerBlock) -> String {
    match block {
        TimerBlock::Wait(w) => format!("{}分{}秒 ウェイト", w.minutes, w.seconds),
        TimerBlock::Countdown(c) => format!(
            "{}分{}秒 カウントダウン (繰り返し{}回, インターバル {}分{}秒)",
            c.minutes, c.seconds, c.repeat_count, c.interval_minutes, c.interval_seconds
        ),
    }
}

fn is_countdown_block(block: &TimerBlock) -> bool {
    matches!(block, TimerBlock::Countdown(_))
}

#[function_component(RunningTimerModal)]
pub fn running_timer_modal() -> Html {
    let state = use_context::<AppStateContext>().expect("no context found");

    let running = match &state.running {
        Some(r) => r.clone(),
        None => return html! {},
    };

    let on_close = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(AppAction::CloseRunningModal);
        })
    };

    let current_ix = running.current_block_index;
    let remaining = running.remaining_secs;
    let is_complete = running.is_complete;
    let countdown_run = running.countdown_run;
    let countdown_phase = running.countdown_phase.clone();
    let current_block_is_countdown = running
        .timer
        .blocks
        .get(current_ix)
        .map_or(false, is_countdown_block);

    html! {
        <div class="modal-overlay running-timer-modal-overlay" role="dialog" aria-modal="true">
            <div class="modal-box running-timer-modal">
                <div class="running-timer-modal-header">
                    <h2 class="modal-title">{"タイマー実行中"}</h2>
                    <p class="running-timer-name">{ &running.timer.name }</p>
                </div>

                if is_complete {
                    <div class="running-timer-complete">{"完了！"}</div>
                } else {
                    <ul class="running-timer-block-list">
                        { for running.timer.blocks.iter().enumerate().map(|(i, block)| {
                            let is_current = i == current_ix;
                            let is_done = is_complete || i < current_ix;
                            let class = if is_done {
                                "running-timer-block-item done"
                            } else if is_current {
                                "running-timer-block-item current"
                            } else {
                                "running-timer-block-item"
                            };
                            let show_countdown_phase = is_current && current_block_is_countdown && countdown_run.is_some();
                            let phase_countdown_active = countdown_phase.as_ref() == Some(&CountdownPhase::Countdown);
                            let phase_interval_active = countdown_phase.as_ref() == Some(&CountdownPhase::Interval);
                            let config_line = block_config_line(block);
                            html! {
                                <li class={class}>
                                    <span class="block-type">{ block_type_name(block) }</span>
                                    <span class="block-name">{ block_name(block) }</span>
                                    <div class="block-config">{ &config_line }</div>
                                    if is_current {
                                        if show_countdown_phase {
                                            if let Some((cur, tot)) = countdown_run {
                                                <div class="block-countdown-run">{"現在 "}{ cur }{ " / " }{ tot }{ " 回" }</div>
                                            }
                                            <div class="block-phase-row">
                                                <span class={if phase_countdown_active { "block-phase active" } else { "block-phase" }}>
                                                    {"カウントダウン"}
                                                </span>
                                                <span class={if phase_interval_active { "block-phase active" } else { "block-phase" }}>
                                                    {"インターバル"}
                                                </span>
                                            </div>
                                        }
                                        <div class="block-remaining">
                                            {"残り "}{ remaining }{ " 秒"}
                                        </div>
                                    }
                                </li>
                            }
                        })}
                    </ul>
                }

                <div class="modal-actions running-timer-modal-actions">
                    <button type="button" class="btn btn-primary" onclick={on_close}>
                        { if is_complete { "閉じる" } else { "中止" } }
                    </button>
                </div>
            </div>
        </div>
    }
}
