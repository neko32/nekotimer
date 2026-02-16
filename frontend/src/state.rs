use std::cell::Cell;
use std::rc::Rc;
use yew::prelude::*;
use nekotimer_shared::{TimerConfig, TimerBlock, WaitBlock};

pub type AppStateContext = UseReducerHandle<AppState>;

#[derive(Clone, Debug, PartialEq)]
pub enum ViewMode {
    NoContent,
    Builder,
    /// サイドバーでマウスオーバーしたタイマーのプレビュー表示用（非編集）
    ViewTimer(String),
}

/// 未保存のまま遷移しようとしたときの保留先
#[derive(Clone, Debug, PartialEq)]
pub enum PendingNavigation {
    ToViewTimer(String),
    ToEditTimer(String),
    ToNewTimer,
}

/// カウントダウンブロック内のどちらの区間を実行中か
#[derive(Clone, Debug, PartialEq)]
pub enum CountdownPhase {
    /// カウントダウン区間
    Countdown,
    /// インターバル待機区間
    Interval,
}

/// タイマー実行中モーダル用の状態。閉じたら cancel_token を true にして実行中止。
#[derive(Clone, Debug)]
pub struct RunningInfo {
    pub timer: TimerConfig,
    pub current_block_index: usize,
    pub remaining_secs: u32,
    pub is_complete: bool,
    pub cancel_token: Rc<Cell<bool>>,
    /// カウントダウンブロック時のみ: (現在の回数 1-based, 繰り返し回数)
    pub countdown_run: Option<(u32, u32)>,
    /// カウントダウンブロック時のみ: 現在カウントダウン中かインターバル中か
    pub countdown_phase: Option<CountdownPhase>,
}

impl PartialEq for RunningInfo {
    fn eq(&self, other: &Self) -> bool {
        self.timer == other.timer
            && self.current_block_index == other.current_block_index
            && self.remaining_secs == other.remaining_secs
            && self.is_complete == other.is_complete
            && self.countdown_run == other.countdown_run
            && self.countdown_phase == other.countdown_phase
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub timers: Vec<TimerConfig>,
    pub view: ViewMode,
    pub editing_timer: Option<TimerConfig>,
    pub dragging_block_index: Option<usize>,
    pub dragging_new_block: Option<String>,
    pub validation_errors: Vec<String>,
    pub last_saved_id: Option<String>,
    pub form_dirty: bool,
    /// 未保存確認モーダルで選択待ちの遷移先
    pub pending_navigation: Option<PendingNavigation>,
    /// 編集を続ける選択後、ビルダーのタイマー名入力にフォーカスするフラグ
    pub focus_builder_name: bool,
    /// タイマー実行中モーダル（あるときのみ表示）
    pub running: Option<RunningInfo>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            timers: Vec::new(),
            view: ViewMode::NoContent,
            editing_timer: None,
            dragging_block_index: None,
            dragging_new_block: None,
            validation_errors: Vec::new(),
            last_saved_id: None,
            form_dirty: false,
            pending_navigation: None,
            focus_builder_name: false,
            running: None,
        }
    }
}

pub enum AppAction {
    SetTimers(Vec<TimerConfig>),
    StartNewTimer,
    /// サイドバーでタイマーにマウスオーバーしたときにプレビュー表示。None でプレビュー解除。
    PreviewTimer(Option<String>),
    EditTimer(String),
    SetTimerName(String),
    AddBlock(TimerBlock, usize),
    RemoveBlock(usize),
    UpdateBlock(usize, TimerBlock),
    MoveBlock { from: usize, to: usize },
    StartDraggingBlock(usize),
    StartDraggingNewBlock(String),
    StopDragging,
    SaveSuccess(TimerConfig),
    SetValidationErrors(Vec<String>),
    /// 未保存確認で「編集を止める」→ 破棄して遷移を実行
    ConfirmDiscard,
    /// 未保存確認で「編集を続ける」→ モーダルを閉じてビルダーに留まる
    CancelNavigate,
    /// ビルダー名入力へフォーカス済みの通知（フラグクリア用）
    ClearFocusBuilderName,
    /// タイマー実行開始（モーダル表示用。runner は Sidebar で spawn）
    StartTimerExecution(TimerConfig, Rc<Cell<bool>>),
    /// 実行中のブロック・残り秒数の更新（カウントダウン時は回数とフェーズも）
    SetRunningProgress {
        block_ix: usize,
        remaining_secs: u32,
        countdown_run: Option<(u32, u32)>,
        countdown_phase: Option<CountdownPhase>,
    },
    /// 全ブロック実行完了
    TimerExecutionComplete,
    /// モーダルを閉じて実行を中止（cancel_token を立てる）
    CloseRunningModal,
    /// 実行が中止 or 完了したので running をクリア
    TimerExecutionStopped,
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();

        match action {
            AppAction::SetTimers(timers) => {
                next.timers = timers;
            }
            AppAction::StartNewTimer => {
                if next.view == ViewMode::Builder && next.form_dirty {
                    next.pending_navigation = Some(PendingNavigation::ToNewTimer);
                    return Rc::new(next);
                }
                next.view = ViewMode::Builder;
                next.validation_errors.clear();
                next.form_dirty = false;
                next.editing_timer = Some(TimerConfig {
                    id: String::new(),
                    name: String::new(),
                    blocks: vec![TimerBlock::Wait(WaitBlock {
                        name: "default".into(),
                        minutes: 0,
                        seconds: 10,
                    })],
                });
            }
            AppAction::PreviewTimer(opt_id) => {
                if let Some(ref id) = opt_id {
                    if next.view == ViewMode::Builder && next.form_dirty {
                        next.pending_navigation = Some(PendingNavigation::ToViewTimer(id.clone()));
                        return Rc::new(next);
                    }
                }
                next.view = match opt_id {
                    Some(id) => ViewMode::ViewTimer(id),
                    None => ViewMode::NoContent,
                };
            }
            AppAction::EditTimer(id) => {
                if next.view == ViewMode::Builder && next.form_dirty {
                    next.pending_navigation = Some(PendingNavigation::ToEditTimer(id.clone()));
                    return Rc::new(next);
                }
                if let Some(timer) = next.timers.iter().find(|t| t.id == id) {
                    next.view = ViewMode::Builder;
                    next.validation_errors.clear();
                    next.form_dirty = false;
                    next.editing_timer = Some(timer.clone());
                }
            }
            AppAction::SetTimerName(name) => {
                if let Some(ref mut timer) = next.editing_timer {
                    timer.name = name;
                    next.form_dirty = true;
                }
            }
            AppAction::AddBlock(block, index) => {
                if let Some(ref mut timer) = next.editing_timer {
                    let idx = index.min(timer.blocks.len());
                    timer.blocks.insert(idx, block);
                    next.form_dirty = true;
                }
            }
            AppAction::RemoveBlock(index) => {
                if let Some(ref mut timer) = next.editing_timer {
                    if index < timer.blocks.len() && timer.blocks.len() > 1 {
                        timer.blocks.remove(index);
                        next.form_dirty = true;
                    }
                }
            }
            AppAction::UpdateBlock(index, block) => {
                if let Some(ref mut timer) = next.editing_timer {
                    if index < timer.blocks.len() {
                        timer.blocks[index] = block;
                        next.form_dirty = true;
                    }
                }
            }
            AppAction::MoveBlock { from, to } => {
                if let Some(ref mut timer) = next.editing_timer {
                    if from < timer.blocks.len() {
                        let block = timer.blocks.remove(from);
                        let adjusted_to = if from < to {
                            (to - 1).min(timer.blocks.len())
                        } else {
                            to.min(timer.blocks.len())
                        };
                        timer.blocks.insert(adjusted_to, block);
                        next.form_dirty = true;
                    }
                }
            }
            AppAction::StartDraggingBlock(index) => {
                next.dragging_block_index = Some(index);
                next.dragging_new_block = None;
            }
            AppAction::StartDraggingNewBlock(block_type) => {
                next.dragging_new_block = Some(block_type);
                next.dragging_block_index = None;
            }
            AppAction::StopDragging => {
                next.dragging_block_index = None;
                next.dragging_new_block = None;
            }
            AppAction::SaveSuccess(timer) => {
                next.last_saved_id = Some(timer.id.clone());
                let existing = next.timers.iter().position(|t| t.id == timer.id);
                match existing {
                    Some(pos) => next.timers[pos] = timer.clone(),
                    None => next.timers.push(timer.clone()),
                }
                next.editing_timer = Some(timer);
                next.validation_errors.clear();
                next.form_dirty = false;
            }
            AppAction::SetValidationErrors(errors) => {
                next.validation_errors = errors;
            }
            AppAction::ConfirmDiscard => {
                if let Some(pending) = next.pending_navigation.take() {
                    next.form_dirty = false;
                    next.validation_errors.clear();
                    match pending {
                        PendingNavigation::ToViewTimer(id) => {
                            next.view = ViewMode::ViewTimer(id);
                        }
                        PendingNavigation::ToEditTimer(id) => {
                            if let Some(timer) = next.timers.iter().find(|t| t.id == id) {
                                next.view = ViewMode::Builder;
                                next.editing_timer = Some(timer.clone());
                            }
                        }
                        PendingNavigation::ToNewTimer => {
                            next.view = ViewMode::Builder;
                            next.editing_timer = Some(TimerConfig {
                                id: String::new(),
                                name: String::new(),
                                blocks: vec![TimerBlock::Wait(WaitBlock {
                                    name: "default".into(),
                                    minutes: 0,
                                    seconds: 10,
                                })],
                            });
                        }
                    }
                }
            }
            AppAction::CancelNavigate => {
                next.pending_navigation = None;
                next.focus_builder_name = true;
            }
            AppAction::ClearFocusBuilderName => {
                next.focus_builder_name = false;
            }
            AppAction::StartTimerExecution(timer, cancel_token) => {
                next.running = Some(RunningInfo {
                    timer,
                    current_block_index: 0,
                    remaining_secs: 0,
                    is_complete: false,
                    cancel_token,
                    countdown_run: None,
                    countdown_phase: None,
                });
            }
            AppAction::SetRunningProgress {
                block_ix,
                remaining_secs,
                countdown_run,
                countdown_phase,
            } => {
                if let Some(ref mut r) = next.running {
                    r.current_block_index = block_ix;
                    r.remaining_secs = remaining_secs;
                    r.countdown_run = countdown_run;
                    r.countdown_phase = countdown_phase;
                }
            }
            AppAction::TimerExecutionComplete => {
                if let Some(ref mut r) = next.running {
                    r.is_complete = true;
                }
            }
            AppAction::CloseRunningModal => {
                if let Some(ref r) = next.running {
                    r.cancel_token.set(true);
                }
                next.running = None;
            }
            AppAction::TimerExecutionStopped => {
                next.running = None;
            }
        }

        Rc::new(next)
    }
}
