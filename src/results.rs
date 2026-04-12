use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub experiment: String,
    pub mode: String,            // "cpu" or "memory_engine"
    pub dataset_size_mb: u64,
    pub runtime_ms: u128,
    pub cycles: u64,
    pub memory_access_bytes: u64,
    pub data_moved_bytes: u64,
    pub operations: u64,
    pub operational_intensity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stride: Option<usize>,
}

impl ExperimentResult {
    pub fn new(
        experiment: &str,
        mode: &str,
        dataset_size_mb: u64,
        runtime_ms: u128,
        cycles: u64,
        memory_access_bytes: u64,
        data_moved_bytes: u64,
        operations: u64,
    ) -> Self {
        let operational_intensity = if data_moved_bytes > 0 {
            operations as f64 / data_moved_bytes as f64
        } else {
            0.0
        };

        ExperimentResult {
            experiment: experiment.to_string(),
            mode: mode.to_string(),
            dataset_size_mb,
            runtime_ms,
            cycles,
            memory_access_bytes,
            data_moved_bytes,
            operations,
            operational_intensity,
            stride: None,
        }
    }

    pub fn with_stride(
        experiment: &str,
        mode: &str,
        dataset_size_mb: u64,
        runtime_ms: u128,
        cycles: u64,
        memory_access_bytes: u64,
        data_moved_bytes: u64,
        operations: u64,
        stride: usize,
    ) -> Self {
        let operational_intensity = if data_moved_bytes > 0 {
            operations as f64 / data_moved_bytes as f64
        } else {
            0.0
        };

        ExperimentResult {
            experiment: experiment.to_string(),
            mode: mode.to_string(),
            dataset_size_mb,
            runtime_ms,
            cycles,
            memory_access_bytes,
            data_moved_bytes,
            operations,
            operational_intensity,
            stride: Some(stride),
        }
    }

    /// Export result as a single line of JSONL
    pub fn to_json_line(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    /// Append this result to a JSONL file
    pub fn append_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json_line = self.to_json_line()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        writeln!(file, "{}", json_line)?;
        Ok(())
    }

    /// Append multiple results to a JSONL file
    pub fn append_batch_to_file<P: AsRef<Path>>(results: &[ExperimentResult], path: P) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        for result in results {
            let json_line = result.to_json_line()?;
            writeln!(file, "{}", json_line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experiment_result_serialization() {
        let operations = 256 * 1024 * 1024 / 4;
        let result = ExperimentResult::new("scan", "cpu", 256, 92, 121241, 268000000, 133000000, operations);
        let json = result.to_json_line().unwrap();
        assert!(json.contains("\"experiment\":\"scan\""));
        assert!(json.contains("\"mode\":\"cpu\""));
        assert!(json.contains("\"operations\":"));
        assert!(json.contains("\"operational_intensity\":"));
    }
}
