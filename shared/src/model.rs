use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TimerConfig {
    pub id: String,
    pub name: String,
    pub blocks: Vec<TimerBlock>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum TimerBlock {
    Wait(WaitBlock),
    Countdown(CountdownBlock),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WaitBlock {
    pub name: String,
    pub minutes: u32,
    pub seconds: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CountdownBlock {
    pub name: String,
    pub minutes: u32,
    pub seconds: u32,
    pub repeat_count: u32,
    pub interval_minutes: u32,
    pub interval_seconds: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TimerConfigFile {
    pub timers: Vec<TimerConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Option<Vec<ValidationError>>,
}

impl TimerConfig {
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(ValidationError {
                field: "name".into(),
                message: "タイマー名は必須です".into(),
            });
        }
        if self.name.chars().count() > 64 {
            errors.push(ValidationError {
                field: "name".into(),
                message: "タイマー名は64文字以内にしてください".into(),
            });
        }
        for ch in self.name.chars() {
            if !(ch.is_alphanumeric() || ch == ' ' || ch == '_' || is_japanese(ch)) {
                errors.push(ValidationError {
                    field: "name".into(),
                    message: format!("タイマー名に無効な文字が含まれています: '{}'", ch),
                });
                break;
            }
        }

        if self.blocks.is_empty() {
            errors.push(ValidationError {
                field: "blocks".into(),
                message: "最低1つのブロックが必要です".into(),
            });
        }

        for (i, block) in self.blocks.iter().enumerate() {
            match block {
                TimerBlock::Wait(w) => {
                    if w.seconds < 1 || w.seconds > 59 {
                        errors.push(ValidationError {
                            field: format!("blocks[{}].seconds", i),
                            message: "待機秒は1〜59の範囲で入力してください".into(),
                        });
                    }
                    if w.minutes > 1440 {
                        errors.push(ValidationError {
                            field: format!("blocks[{}].minutes", i),
                            message: "待機分は0〜1440の範囲で入力してください".into(),
                        });
                    }
                }
                TimerBlock::Countdown(c) => {
                    if c.seconds < 1 || c.seconds > 59 {
                        errors.push(ValidationError {
                            field: format!("blocks[{}].seconds", i),
                            message: "カウントダウン秒は1〜59の範囲で入力してください".into(),
                        });
                    }
                    if c.minutes > 1440 {
                        errors.push(ValidationError {
                            field: format!("blocks[{}].minutes", i),
                            message: "カウントダウン分は0〜1440の範囲で入力してください".into(),
                        });
                    }
                    if c.repeat_count > 100 {
                        errors.push(ValidationError {
                            field: format!("blocks[{}].repeat_count", i),
                            message: "繰り返し回数は0〜100の範囲で入力してください".into(),
                        });
                    }
                    if c.interval_minutes > 1440 {
                        errors.push(ValidationError {
                            field: format!("blocks[{}].interval_minutes", i),
                            message: "インターバル分は0〜1440の範囲で入力してください".into(),
                        });
                    }
                    if c.interval_seconds > 59 {
                        errors.push(ValidationError {
                            field: format!("blocks[{}].interval_seconds", i),
                            message: "インターバル秒は0〜59の範囲で入力してください".into(),
                        });
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn is_japanese(ch: char) -> bool {
    matches!(ch,
        '\u{3040}'..='\u{309F}' |
        '\u{30A0}'..='\u{30FF}' |
        '\u{4E00}'..='\u{9FFF}' |
        '\u{FF00}'..='\u{FFEF}'
    )
}
