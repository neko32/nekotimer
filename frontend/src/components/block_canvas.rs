use web_sys::DragEvent;
use yew::prelude::*;

use nekotimer_shared::{CountdownBlock, TimerBlock, WaitBlock};

use crate::components::block_drop_zone::BlockDropZone;
use crate::components::countdown_block::CountdownBlockEditor;
use crate::components::wait_block::WaitBlockEditor;
use crate::state::{AppAction, AppStateContext};

#[function_component(BlockCanvas)]
pub fn block_canvas() -> Html {
    let state = use_context::<AppStateContext>().expect("no context found");

    let blocks = match &state.editing_timer {
        Some(t) => t.blocks.clone(),
        None => return html! {},
    };

    let on_palette_drag_wait = {
        Callback::from(move |e: DragEvent| {
            if let Some(dt) = e.data_transfer() {
                let _ = dt.set_data("text/plain", "new:wait");
            }
        })
    };

    let on_palette_drag_countdown = {
        Callback::from(move |e: DragEvent| {
            if let Some(dt) = e.data_transfer() {
                let _ = dt.set_data("text/plain", "new:countdown");
            }
        })
    };

    html! {
        <div class="block-canvas">
            <h3 class="canvas-title">{"ブロック"}</h3>

            <div class="block-list">
                <BlockDropZone index={0} />
                { for blocks.iter().enumerate().map(|(i, block)| {
                    let index = i;
                    let ondragstart = {
                        Callback::from(move |e: DragEvent| {
                            if let Some(dt) = e.data_transfer() {
                                let _ = dt.set_data("text/plain", &format!("existing:{}", index));
                            }
                        })
                    };

                    let on_remove = {
                        let state = state.clone();
                        Callback::from(move |_: MouseEvent| {
                            state.dispatch(AppAction::RemoveBlock(index));
                        })
                    };

                    let on_update = {
                        let state = state.clone();
                        Callback::from(move |block: TimerBlock| {
                            state.dispatch(AppAction::UpdateBlock(index, block));
                        })
                    };

                    let block_class = match block {
                        TimerBlock::Wait(_) => "block-item",
                        TimerBlock::Countdown(_) => "block-item countdown",
                    };

                    let can_remove = blocks.len() > 1;

                    html! {
                        <>
                            <div class={block_class}
                                draggable="true"
                                {ondragstart}
                            >
                                <div class="block-header">
                                    { match block {
                                        TimerBlock::Wait(_) => html! {
                                            <span class="block-type-label">{"WAIT"}</span>
                                        },
                                        TimerBlock::Countdown(_) => html! {
                                            <span class="block-type-label countdown">{"COUNTDOWN"}</span>
                                        },
                                    }}
                                    if can_remove {
                                        <button class="block-remove-btn" onclick={on_remove}>
                                            { "\u{00d7}" }
                                        </button>
                                    }
                                </div>
                                { match block {
                                    TimerBlock::Wait(w) => html! {
                                        <WaitBlockEditor block={w.clone()} on_change={on_update.clone()} />
                                    },
                                    TimerBlock::Countdown(c) => html! {
                                        <CountdownBlockEditor block={c.clone()} on_change={on_update.clone()} />
                                    },
                                }}
                            </div>
                            <BlockDropZone index={i + 1} />
                        </>
                    }
                })}
            </div>

            <div class="block-palette">
                <div class="palette-item" draggable="true" ondragstart={on_palette_drag_wait}>
                    {"+ 待機ブロック"}
                </div>
                <div class="palette-item" draggable="true" ondragstart={on_palette_drag_countdown}>
                    {"+ カウントダウンブロック"}
                </div>
            </div>
        </div>
    }
}

pub fn default_wait_block() -> TimerBlock {
    TimerBlock::Wait(WaitBlock {
        name: String::new(),
        minutes: 0,
        seconds: 10,
    })
}

pub fn default_countdown_block() -> TimerBlock {
    TimerBlock::Countdown(CountdownBlock {
        name: String::new(),
        minutes: 1,
        seconds: 0,
        repeat_count: 1,
        interval_minutes: 0,
        interval_seconds: 10,
    })
}
