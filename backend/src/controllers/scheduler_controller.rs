use axum::{http::StatusCode, response::Json};

use crate::job_scheduler::{get_scheduler_info, reload_jobs};
use crate::models::ApiResponse;

/// Get scheduler status and information
pub async fn get_scheduler_status()
-> Result<Json<ApiResponse<SchedulerStatus>>, (StatusCode, Json<ApiResponse<()>>)> {
    match get_scheduler_info().await {
        Ok((is_running, scheduled_jobs)) => {
            let status = SchedulerStatus {
                is_running,
                job_count: scheduled_jobs.len(),
                scheduled_jobs: scheduled_jobs
                    .into_iter()
                    .map(|sj| ScheduledJobInfo {
                        slug: sj.slug,
                        cadence: sj.job.cadence,
                        characters: sj.job.characters,
                        prompts: sj.job.prompts,
                        next_run: sj.next_run.to_rfc3339(),
                    })
                    .collect(),
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(status),
                message: "Scheduler status retrieved successfully".to_string(),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get scheduler status: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to get scheduler status: {}", e),
                }),
            ))
        }
    }
}

/// Reload jobs from disk into the scheduler
pub async fn reload_scheduler_jobs()
-> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    match reload_jobs().await {
        Ok(()) => Ok(Json(ApiResponse {
            success: true,
            data: Some(()),
            message: "Jobs reloaded successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to reload jobs: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: format!("Failed to reload jobs: {}", e),
                }),
            ))
        }
    }
}

/// Response models for scheduler endpoints
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SchedulerStatus {
    pub is_running: bool,
    pub job_count: usize,
    pub scheduled_jobs: Vec<ScheduledJobInfo>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScheduledJobInfo {
    pub slug: String,
    pub cadence: String,
    pub characters: Vec<String>,
    pub prompts: Vec<String>,
    pub next_run: String, // ISO 8601 formatted string
}
