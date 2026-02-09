use actix_web::{web, HttpResponse};
use nekotimer_shared::{ApiResponse, TimerConfig, ValidationError};
use uuid::Uuid;

use crate::AppState;
use crate::persistence;

pub async fn list_timers(data: web::Data<AppState>) -> HttpResponse {
    let config = data.config.lock().unwrap();
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(config.timers.clone()),
        errors: None::<Vec<ValidationError>>,
    })
}

pub async fn get_timer(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let config = data.config.lock().unwrap();
    match config.timers.iter().find(|t| t.id == id) {
        Some(timer) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(timer.clone()),
            errors: None::<Vec<ValidationError>>,
        }),
        None => HttpResponse::NotFound().json(ApiResponse::<TimerConfig> {
            success: false,
            data: None,
            errors: Some(vec![ValidationError {
                field: "id".into(),
                message: "タイマーが見つかりません".into(),
            }]),
        }),
    }
}

pub async fn create_timer(
    data: web::Data<AppState>,
    body: web::Json<TimerConfig>,
) -> HttpResponse {
    let mut timer = body.into_inner();
    timer.id = Uuid::new_v4().to_string();

    if let Err(errors) = timer.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<TimerConfig> {
            success: false,
            data: None,
            errors: Some(errors),
        });
    }

    let mut config = data.config.lock().unwrap();

    if config.timers.iter().any(|t| t.name == timer.name) {
        return HttpResponse::Conflict().json(ApiResponse::<TimerConfig> {
            success: false,
            data: None,
            errors: Some(vec![ValidationError {
                field: "name".into(),
                message: "同じ名前のタイマーが既に存在します".into(),
            }]),
        });
    }

    config.timers.push(timer.clone());
    if let Err(e) = persistence::save_config(&data.config_path, &config) {
        return HttpResponse::InternalServerError().json(ApiResponse::<TimerConfig> {
            success: false,
            data: None,
            errors: Some(vec![ValidationError {
                field: "system".into(),
                message: format!("保存に失敗しました: {}", e),
            }]),
        });
    }

    HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(timer),
        errors: None::<Vec<ValidationError>>,
    })
}

pub async fn update_timer(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<TimerConfig>,
) -> HttpResponse {
    let id = path.into_inner();
    let mut timer = body.into_inner();
    timer.id = id.clone();

    if let Err(errors) = timer.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<TimerConfig> {
            success: false,
            data: None,
            errors: Some(errors),
        });
    }

    let mut config = data.config.lock().unwrap();

    if let Some(existing) = config.timers.iter_mut().find(|t| t.id == id) {
        *existing = timer.clone();
        if let Err(e) = persistence::save_config(&data.config_path, &config) {
            return HttpResponse::InternalServerError().json(ApiResponse::<TimerConfig> {
                success: false,
                data: None,
                errors: Some(vec![ValidationError {
                    field: "system".into(),
                    message: format!("保存に失敗しました: {}", e),
                }]),
            });
        }
        HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(timer),
            errors: None::<Vec<ValidationError>>,
        })
    } else {
        HttpResponse::NotFound().json(ApiResponse::<TimerConfig> {
            success: false,
            data: None,
            errors: Some(vec![ValidationError {
                field: "id".into(),
                message: "タイマーが見つかりません".into(),
            }]),
        })
    }
}

pub async fn delete_timer(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let mut config = data.config.lock().unwrap();

    let len_before = config.timers.len();
    config.timers.retain(|t| t.id != id);

    if config.timers.len() == len_before {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            errors: Some(vec![ValidationError {
                field: "id".into(),
                message: "タイマーが見つかりません".into(),
            }]),
        });
    }

    if let Err(e) = persistence::save_config(&data.config_path, &config) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            errors: Some(vec![ValidationError {
                field: "system".into(),
                message: format!("保存に失敗しました: {}", e),
            }]),
        });
    }

    HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        errors: None,
    })
}
