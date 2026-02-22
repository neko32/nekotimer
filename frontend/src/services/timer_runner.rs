//! タイマー通し実行: ブロック順に Wait / Countdown を実行し、効果音を鳴らす。
//! キャンセルトークンと進捗 dispatch に対応。
//! 効果音は起動時にプリロードしてキャッシュし、再生時は毎回ロードしない。

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use gloo_timers::future::TimeoutFuture;
use nekotimer_shared::{CountdownBlock, TimerBlock, TimerConfig, WaitBlock};
use web_sys::{AudioContext, HtmlAudioElement};

use crate::state::{AppAction, CountdownPhase, AppStateContext};

const TICK_UNDER_SECS: u32 = 5;
const SOUND_TICK: &str = "/sound/tick.mp3";
const SOUND_BLOCK_END: &str = "/sound/block_end.mp3";
const SOUND_ALL_END: &str = "/sound/all_end.mp3";
const SOUND_COUNTDOWN_BLK_NEXT: &str = "/sound/countdown_blk_next.mp3";

thread_local! {
    static AUDIO_CACHE: RefCell<Option<HashMap<String, HtmlAudioElement>>> = RefCell::new(None);
    /// iOS WebKit などで AudioContext が suspended のままになるのを防ぐ。ユーザー操作（実行ボタン）内で呼ぶ。
    static AUDIO_CONTEXT: RefCell<Option<AudioContext>> = RefCell::new(None);
}

/// iOS WebKit 向け: ユーザー操作（実行ボタン押下）内で呼び、AudioContext を resume する。
/// これによりオーディオが unlocked され、その後の HtmlAudioElement 再生が可能になることがある。
pub fn unlock_audio_for_ios() {
    AUDIO_CONTEXT.with(|cell| {
        if cell.borrow().is_none() {
            if let Ok(ctx) = AudioContext::new() {
                let _ = ctx.resume();
                *cell.borrow_mut() = Some(ctx);
            }
        } else {
            if let Some(ref ctx) = *cell.borrow() {
                let _ = ctx.resume();
            }
        }
    });
}

/// ウェブアセンブリ起動時に呼び、各 MP3 をプリロードしてキャッシュする。
pub fn init_audio_cache() {
    AUDIO_CACHE.with(|cache| {
        if cache.borrow().is_none() {
            let mut m = HashMap::new();
            for path in [SOUND_TICK, SOUND_BLOCK_END, SOUND_ALL_END, SOUND_COUNTDOWN_BLK_NEXT] {
                if let Ok(audio) = HtmlAudioElement::new_with_src(path) {
                    let _ = audio.load();
                    m.insert(path.to_string(), audio);
                }
            }
            *cache.borrow_mut() = Some(m);
        }
    });
}

fn play_sound(path: &str) {
    AUDIO_CACHE.with(|cache| {
        if let Ok(guard) = cache.try_borrow() {
            if let Some(ref map) = *guard {
                if let Some(audio) = map.get(path) {
                    let _ = audio.set_current_time(0.0);
                    let _ = audio.play();
                }
            }
        }
    });
}

fn block_initial_remaining(block: &TimerBlock) -> u32 {
    match block {
        TimerBlock::Wait(w) => w.minutes * 60 + w.seconds,
        TimerBlock::Countdown(c) => c.minutes * 60 + c.seconds,
    }
}

/// 1秒待ち、残り秒数を on_tick で報告。キャンセルされていたら false。
async fn wait_one_second_report(
    remaining_after_this_sec: u32,
    cancel: &Rc<Cell<bool>>,
    on_tick: &impl Fn(u32),
) -> bool {
    if cancel.get() {
        return false;
    }
    TimeoutFuture::new(1000).await;
    if remaining_after_this_sec <= TICK_UNDER_SECS && remaining_after_this_sec >= 1 {
        play_sound(SOUND_TICK);
    }
    on_tick(remaining_after_this_sec);
    !cancel.get()
}

/// 指定秒数だけ待機。残り5秒以下で1秒ごとに tick。毎秒 on_tick(残り秒) を呼ぶ。
async fn wait_seconds_with_tick(
    total_secs: u32,
    cancel: &Rc<Cell<bool>>,
    on_tick: &impl Fn(u32),
) -> bool {
    if total_secs == 0 {
        return true;
    }
    for elapsed in 1..=total_secs {
        let remaining = total_secs - elapsed;
        if !wait_one_second_report(remaining, cancel, on_tick).await {
            return false;
        }
    }
    true
}

fn dispatch_progress(
    state: &AppStateContext,
    block_ix: usize,
    remaining_secs: u32,
    countdown_run: Option<(u32, u32)>,
    countdown_phase: Option<CountdownPhase>,
) {
    state.dispatch(AppAction::SetRunningProgress {
        block_ix,
        remaining_secs,
        countdown_run,
        countdown_phase,
    });
}

async fn run_wait_block(
    w: &WaitBlock,
    block_ix: usize,
    cancel: &Rc<Cell<bool>>,
    state: &AppStateContext,
) -> bool {
    let total_secs = w.minutes * 60 + w.seconds;
    let state = state.clone();
    let on_tick = move |remaining: u32| {
        dispatch_progress(&state, block_ix, remaining, None, None);
    };
    wait_seconds_with_tick(total_secs, cancel, &on_tick).await
}

async fn run_countdown_block(
    c: &CountdownBlock,
    block_ix: usize,
    cancel: &Rc<Cell<bool>>,
    state: &AppStateContext,
) -> bool {
    let countdown_secs = c.minutes * 60 + c.seconds;
    let interval_secs = c.interval_minutes * 60 + c.interval_seconds;
    let repeat = c.repeat_count.max(1);
    let state = state.clone();

    for run in 0..repeat {
        if cancel.get() {
            return false;
        }
        let run_1based = run + 1;
        dispatch_progress(
            &state,
            block_ix,
            countdown_secs,
            Some((run_1based, repeat)),
            Some(CountdownPhase::Countdown),
        );
        let on_tick = {
            let state = state.clone();
            move |remaining: u32| {
                dispatch_progress(
                    &state,
                    block_ix,
                    remaining,
                    Some((run_1based, repeat)),
                    Some(CountdownPhase::Countdown),
                );
            }
        };
        if !wait_seconds_with_tick(countdown_secs, cancel, &on_tick).await {
            return false;
        }
        // 最後のカウントダウンでない場合、カウントダウン終了後に再生
        if run + 1 < repeat {
            play_sound(SOUND_COUNTDOWN_BLK_NEXT);
        }
        if run + 1 < repeat {
            if cancel.get() {
                return false;
            }
            dispatch_progress(
                &state,
                block_ix,
                interval_secs,
                Some((run_1based, repeat)),
                Some(CountdownPhase::Interval),
            );
            let on_tick_int = {
                let state = state.clone();
                move |remaining: u32| {
                    dispatch_progress(
                        &state,
                        block_ix,
                        remaining,
                        Some((run_1based, repeat)),
                        Some(CountdownPhase::Interval),
                    );
                }
            };
            if !wait_seconds_with_tick(interval_secs, cancel, &on_tick_int).await {
                return false;
            }
            // インターバル終了・次のカウントダウンに移る場合に再生
            play_sound(SOUND_COUNTDOWN_BLK_NEXT);
        }
    }
    true
}

async fn run_block(
    block: &TimerBlock,
    block_ix: usize,
    is_last: bool,
    cancel: &Rc<Cell<bool>>,
    state: &AppStateContext,
) -> bool {
    let ok = match block {
        TimerBlock::Wait(w) => run_wait_block(w, block_ix, cancel, state).await,
        TimerBlock::Countdown(c) => run_countdown_block(c, block_ix, cancel, state).await,
    };
    if !ok {
        return false;
    }
    if is_last {
        play_sound(SOUND_ALL_END);
    } else {
        play_sound(SOUND_BLOCK_END);
    }
    true
}

/// タイマーを先頭ブロックから最後まで実行。キャンセル時は TimerExecutionStopped を dispatch。
pub async fn run_timer(timer: TimerConfig, cancel_token: Rc<Cell<bool>>, state: AppStateContext) {
    let blocks = timer.blocks.clone();
    if blocks.is_empty() {
        state.dispatch(AppAction::TimerExecutionStopped);
        return;
    }
    let initial = block_initial_remaining(&blocks[0]);
    dispatch_progress(&state, 0, initial, None, None);

    for (i, block) in blocks.iter().enumerate() {
        if cancel_token.get() {
            state.dispatch(AppAction::TimerExecutionStopped);
            return;
        }
        let is_last = i == blocks.len() - 1;
        if !run_block(block, i, is_last, &cancel_token, &state).await {
            state.dispatch(AppAction::TimerExecutionStopped);
            return;
        }
        let next_ix = i + 1;
        if next_ix < blocks.len() {
            let next_initial = block_initial_remaining(&blocks[next_ix]);
            dispatch_progress(&state, next_ix, next_initial, None, None);
        }
    }
    state.dispatch(AppAction::TimerExecutionComplete);
}
