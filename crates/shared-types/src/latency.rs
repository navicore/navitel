use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyMetadata {
    pub stream_id: Uuid,
    pub chunk_id: Uuid,
    pub ingestion_time: DateTime<Utc>,
    pub stages: Vec<ProcessingStage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStage {
    pub name: String,
    pub component: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub metrics: Option<StageMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageMetrics {
    pub items_processed: u64,
    pub bytes_processed: u64,
    pub errors: u64,
    pub custom: serde_json::Value,
}

impl LatencyMetadata {
    pub fn new(stream_id: Uuid) -> Self {
        Self {
            stream_id,
            chunk_id: Uuid::new_v4(),
            ingestion_time: Utc::now(),
            stages: Vec::new(),
        }
    }

    pub fn start_stage(&mut self, name: impl Into<String>, component: impl Into<String>) {
        self.stages.push(ProcessingStage {
            name: name.into(),
            component: component.into(),
            start_time: Utc::now(),
            end_time: None,
            metrics: None,
        });
    }

    pub fn end_stage(&mut self) {
        if let Some(stage) = self.stages.last_mut()
            && stage.end_time.is_none()
        {
            stage.end_time = Some(Utc::now());
        }
    }

    pub fn total_latency(&self) -> Duration {
        let now = Utc::now();
        let elapsed = now - self.ingestion_time;
        Duration::from_millis(elapsed.num_milliseconds().unsigned_abs())
    }

    pub fn stage_latency(&self, stage_name: &str) -> Option<Duration> {
        self.stages
            .iter()
            .find(|s| s.name == stage_name)
            .and_then(|s| {
                s.end_time.map(|end| {
                    let elapsed = end - s.start_time;
                    Duration::from_millis(elapsed.num_milliseconds().unsigned_abs())
                })
            })
    }
}
