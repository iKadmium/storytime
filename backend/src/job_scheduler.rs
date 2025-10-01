use chrono::{DateTime, Utc};
use cron::Schedule;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, OnceLock};
use tokio::sync::{Mutex, broadcast};
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

use crate::controllers::job_controller::{load_all_jobs, run_job_internal};
use crate::models::{Job, Settings};

/// A scheduled job with its cron schedule and metadata
#[derive(Debug, Clone)]
pub struct ScheduledJob {
    pub job: Job,
    pub schedule: Schedule,
    pub next_run: DateTime<Utc>,
    pub slug: String,
}

/// Job scheduler that manages and executes cron jobs
pub struct JobScheduler {
    scheduled_jobs: Arc<Mutex<HashMap<String, ScheduledJob>>>,
    settings: Arc<Settings>,
    shutdown_tx: Option<broadcast::Sender<()>>,
    is_running: Arc<Mutex<bool>>,
}

impl JobScheduler {
    /// Create a new job scheduler
    pub fn new(settings: Arc<Settings>) -> Self {
        Self {
            scheduled_jobs: Arc::new(Mutex::new(HashMap::new())),
            settings,
            shutdown_tx: None,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start the job scheduler
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            warn!("Job scheduler is already running");
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        info!("Starting job scheduler");

        // Load all jobs from disk and schedule them
        self.load_and_schedule_jobs().await?;

        // Create shutdown channel
        let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let scheduled_jobs = Arc::clone(&self.scheduled_jobs);
        let settings = Arc::clone(&self.settings);
        let is_running = Arc::clone(&self.is_running);

        // Start the scheduler loop in a background task
        tokio::spawn(async move {
            info!("Job scheduler loop started");

            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        info!("Job scheduler received shutdown signal");
                        break;
                    }
                    _ = sleep(Duration::from_secs(60)) => {
                        // Check for jobs to run every minute
                        if let Err(e) = Self::check_and_run_jobs(&scheduled_jobs, &settings).await {
                            error!("Error checking and running jobs: {}", e);
                        }
                    }
                }
            }

            let mut is_running = is_running.lock().await;
            *is_running = false;
            info!("Job scheduler stopped");
        });

        info!("Job scheduler started successfully");
        Ok(())
    }

    /// Stop the job scheduler
    pub async fn stop(&mut self) {
        if let Some(ref shutdown_tx) = self.shutdown_tx {
            let _ = shutdown_tx.send(());
        }

        // Wait a moment for the scheduler to stop gracefully
        sleep(Duration::from_millis(100)).await;

        let mut is_running = self.is_running.lock().await;
        *is_running = false;

        info!("Job scheduler stop requested");
    }

    /// Load all jobs from disk and schedule them
    async fn load_and_schedule_jobs(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let jobs = load_all_jobs().await?;
        let mut scheduled_jobs = self.scheduled_jobs.lock().await;

        scheduled_jobs.clear();

        for job in jobs {
            if let Err(e) = self.schedule_job_internal(&mut scheduled_jobs, job).await {
                error!("Failed to schedule job: {}", e);
            }
        }

        info!("Loaded and scheduled {} jobs", scheduled_jobs.len());
        Ok(())
    }

    /// Schedule a single job
    async fn schedule_job_internal(
        &self,
        scheduled_jobs: &mut HashMap<String, ScheduledJob>,
        job: Job,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Parse the cron expression
        let schedule = Schedule::from_str(&job.cadence)
            .map_err(|e| format!("Invalid cron expression '{}': {}", job.cadence, e))?;

        // Calculate next run time
        let next_run = schedule
            .upcoming(Utc)
            .next()
            .ok_or("Could not calculate next run time")?;

        // Use the job's UUID as the key, or generate a fallback slug for compatibility
        let job_key = match job.id {
            Some(id) => id.to_string(),
            None => {
                // Fallback to legacy slug generation for jobs without IDs
                format!(
                    "{}-{}",
                    job.characters.first().unwrap_or(&"unknown".to_string()),
                    job.prompts.first().unwrap_or(&"unknown".to_string())
                )
                .to_lowercase()
                .replace(' ', "-")
            }
        };

        let scheduled_job = ScheduledJob {
            job,
            schedule,
            next_run,
            slug: job_key.clone(),
        };

        scheduled_jobs.insert(job_key.clone(), scheduled_job);

        info!("Scheduled job '{}' with next run at {}", job_key, next_run);
        Ok(())
    }

    /// Check for jobs that need to run and execute them
    async fn check_and_run_jobs(
        scheduled_jobs: &Arc<Mutex<HashMap<String, ScheduledJob>>>,
        settings: &Arc<Settings>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = Utc::now();
        let mut jobs_to_run = Vec::new();
        // Collect jobs that need to run
        {
            let mut jobs = scheduled_jobs.lock().await;
            for (job_key, scheduled_job) in jobs.iter_mut() {
                if scheduled_job.next_run <= now {
                    info!("Job '{}' is due for execution", job_key);
                    jobs_to_run.push(scheduled_job.job.clone());

                    // Update next run time
                    if let Some(next_run) = scheduled_job.schedule.upcoming(Utc).next() {
                        scheduled_job.next_run = next_run;
                        info!("Updated next run time for '{}' to {}", job_key, next_run);
                    } else {
                        warn!("Could not calculate next run time for job '{}'", job_key);
                    }
                }
            }
        }

        // Execute jobs outside the lock
        for job in jobs_to_run {
            let settings_clone = Arc::clone(settings);

            // Run job in a separate task to avoid blocking the scheduler
            tokio::spawn(async move {
                match run_job_internal(job, settings_clone, true).await {
                    Ok(response) => {
                        info!("Job executed successfully: {}", response.0.message);
                    }
                    Err((status, error_response)) => {
                        error!(
                            "Job execution failed with status {}: {}",
                            status, error_response.0.message
                        );
                    }
                }
            });
        }

        Ok(())
    }

    /// Reload jobs from disk (useful for when jobs are added/updated)
    pub async fn reload_jobs(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Reloading jobs from disk");
        self.load_and_schedule_jobs().await
    }

    /// Get information about scheduled jobs
    pub async fn get_scheduled_jobs(&self) -> Vec<ScheduledJob> {
        let jobs = self.scheduled_jobs.lock().await;
        jobs.values().cloned().collect()
    }

    /// Check if the scheduler is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.lock().await
    }
}

/// Global job scheduler instance using OnceLock for thread safety
static SCHEDULER: OnceLock<Arc<Mutex<JobScheduler>>> = OnceLock::new();

/// Start the global job scheduler
pub async fn start_scheduler(
    settings: Arc<Settings>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let scheduler = SCHEDULER.get_or_init(|| Arc::new(Mutex::new(JobScheduler::new(settings))));

    let mut scheduler_lock = scheduler.lock().await;
    scheduler_lock.start().await
}

/// Stop the global job scheduler
pub async fn stop_scheduler() {
    if let Some(scheduler) = SCHEDULER.get() {
        let mut scheduler_lock = scheduler.lock().await;
        scheduler_lock.stop().await;
    }
}

/// Reload jobs in the global scheduler
pub async fn reload_jobs() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(scheduler) = SCHEDULER.get() {
        let scheduler_lock = scheduler.lock().await;
        scheduler_lock.reload_jobs().await
    } else {
        Err("Scheduler not initialized".into())
    }
}

/// Get scheduler status
pub async fn get_scheduler_info()
-> Result<(bool, Vec<ScheduledJob>), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(scheduler) = SCHEDULER.get() {
        let scheduler_lock = scheduler.lock().await;
        let is_running = scheduler_lock.is_running().await;
        let scheduled_jobs = scheduler_lock.get_scheduled_jobs().await;
        Ok((is_running, scheduled_jobs))
    } else {
        Err("Scheduler not initialized".into())
    }
}
