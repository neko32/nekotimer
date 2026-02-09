use std::rc::Rc;
use yew::prelude::*;
use nekotimer_shared::{TimerConfig, TimerBlock, WaitBlock};

pub type AppStateContext = UseReducerHandle<AppState>;

#[derive(Clone, Debug, PartialEq)]
pub enum ViewMode {
    NoContent,
    Builder,
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
        }
    }
}

pub enum AppAction {
    SetTimers(Vec<TimerConfig>),
    StartNewTimer,
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
            AppAction::EditTimer(id) => {
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
        }

        Rc::new(next)
    }
}
