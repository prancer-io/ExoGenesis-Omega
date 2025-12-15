//! Health Monitoring System
//!
//! Provides comprehensive health monitoring for subsystems with configurable
//! thresholds, automatic degradation detection, and aggregated health status.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use thiserror::Error;
use tracing::{debug, warn};

/// Health status of a subsystem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HealthStatus {
    /// Subsystem is operating normally
    Healthy,
    /// Subsystem is degraded but operational
    Degraded,
    /// Subsystem is unhealthy or non-operational
    Unhealthy,
}

impl HealthStatus {
    /// Check if status is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }

    /// Check if status is degraded
    pub fn is_degraded(&self) -> bool {
        matches!(self, HealthStatus::Degraded)
    }

    /// Check if status is unhealthy
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy)
    }

    /// Get numeric score (0-100)
    pub fn score(&self) -> u8 {
        match self {
            HealthStatus::Healthy => 100,
            HealthStatus::Degraded => 50,
            HealthStatus::Unhealthy => 0,
        }
    }
}

/// Health information for a subsystem
#[derive(Debug, Clone)]
pub struct SubsystemHealth {
    /// Subsystem name
    pub name: String,
    /// Current health status
    pub status: HealthStatus,
    /// Last health check timestamp
    pub last_check: Instant,
    /// Consecutive failure count
    pub consecutive_failures: u32,
    /// Consecutive success count
    pub consecutive_successes: u32,
    /// Total check count
    pub total_checks: u64,
    /// Total failure count
    pub total_failures: u64,
    /// Optional metadata
    pub metadata: HashMap<String, String>,
}

impl SubsystemHealth {
    /// Create new subsystem health
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: HealthStatus::Healthy,
            last_check: Instant::now(),
            consecutive_failures: 0,
            consecutive_successes: 0,
            total_checks: 0,
            total_failures: 0,
            metadata: HashMap::new(),
        }
    }

    /// Get time since last check
    pub fn time_since_check(&self) -> Duration {
        self.last_check.elapsed()
    }

    /// Get failure rate as percentage
    pub fn failure_rate(&self) -> f64 {
        if self.total_checks == 0 {
            0.0
        } else {
            (self.total_failures as f64 / self.total_checks as f64) * 100.0
        }
    }

    /// Check if subsystem is stale (hasn't been checked recently)
    pub fn is_stale(&self, threshold: Duration) -> bool {
        self.time_since_check() > threshold
    }
}

/// Health monitor configuration
#[derive(Debug, Clone)]
pub struct HealthMonitorConfig {
    /// Number of consecutive failures before marking as degraded
    pub degraded_threshold: u32,
    /// Number of consecutive failures before marking as unhealthy
    pub unhealthy_threshold: u32,
    /// Number of consecutive successes needed to recover from degraded
    pub recovery_threshold: u32,
    /// Duration after which a subsystem is considered stale
    pub stale_threshold: Duration,
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            degraded_threshold: 3,
            unhealthy_threshold: 5,
            recovery_threshold: 5,
            stale_threshold: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Health monitoring error types
#[derive(Debug, Error)]
pub enum HealthError {
    #[error("Subsystem not found: {0}")]
    SubsystemNotFound(String),
    #[error("Invalid health status transition: {0}")]
    InvalidTransition(String),
}

/// Health monitor for tracking subsystem health
pub struct HealthMonitor {
    subsystems: RwLock<HashMap<String, SubsystemHealth>>,
    config: HealthMonitorConfig,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(config: HealthMonitorConfig) -> Self {
        Self {
            subsystems: RwLock::new(HashMap::new()),
            config,
        }
    }

    /// Create with default configuration
    pub fn with_default_config() -> Self {
        Self::new(HealthMonitorConfig::default())
    }

    /// Register a new subsystem for monitoring
    pub fn register_subsystem(&self, name: String) {
        let mut subsystems = self.subsystems.write();
        if !subsystems.contains_key(&name) {
            debug!("Registering subsystem for health monitoring: {}", name);
            subsystems.insert(name.clone(), SubsystemHealth::new(name));
        }
    }

    /// Unregister a subsystem
    pub fn unregister_subsystem(&self, name: &str) -> Result<(), HealthError> {
        let mut subsystems = self.subsystems.write();
        subsystems
            .remove(name)
            .ok_or_else(|| HealthError::SubsystemNotFound(name.to_string()))?;
        debug!("Unregistered subsystem: {}", name);
        Ok(())
    }

    /// Update health status based on check result
    pub fn update_health(&self, name: &str, is_healthy: bool) -> Result<(), HealthError> {
        let mut subsystems = self.subsystems.write();
        let health = subsystems
            .get_mut(name)
            .ok_or_else(|| HealthError::SubsystemNotFound(name.to_string()))?;

        health.last_check = Instant::now();
        health.total_checks += 1;

        if is_healthy {
            health.consecutive_successes += 1;
            health.consecutive_failures = 0;

            // Update status based on recovery
            match health.status {
                HealthStatus::Unhealthy | HealthStatus::Degraded => {
                    if health.consecutive_successes >= self.config.recovery_threshold {
                        debug!("Subsystem {} recovered to healthy", name);
                        health.status = HealthStatus::Healthy;
                    }
                }
                HealthStatus::Healthy => {}
            }
        } else {
            health.consecutive_failures += 1;
            health.consecutive_successes = 0;
            health.total_failures += 1;

            // Update status based on failures
            let old_status = health.status;
            if health.consecutive_failures >= self.config.unhealthy_threshold {
                health.status = HealthStatus::Unhealthy;
                if old_status != HealthStatus::Unhealthy {
                    warn!("Subsystem {} marked as unhealthy", name);
                }
            } else if health.consecutive_failures >= self.config.degraded_threshold {
                health.status = HealthStatus::Degraded;
                if old_status != HealthStatus::Degraded {
                    warn!("Subsystem {} marked as degraded", name);
                }
            }
        }

        Ok(())
    }

    /// Manually set health status
    pub fn set_health(
        &self,
        name: &str,
        status: HealthStatus,
    ) -> Result<(), HealthError> {
        let mut subsystems = self.subsystems.write();
        let health = subsystems
            .get_mut(name)
            .ok_or_else(|| HealthError::SubsystemNotFound(name.to_string()))?;

        health.status = status;
        health.last_check = Instant::now();

        Ok(())
    }

    /// Check health of a specific subsystem
    pub fn check_health(&self, name: &str) -> Result<SubsystemHealth, HealthError> {
        let subsystems = self.subsystems.read();
        subsystems
            .get(name)
            .cloned()
            .ok_or_else(|| HealthError::SubsystemNotFound(name.to_string()))
    }

    /// Get overall system health status
    pub fn overall_status(&self) -> HealthStatus {
        let subsystems = self.subsystems.read();

        if subsystems.is_empty() {
            return HealthStatus::Healthy;
        }

        let mut has_unhealthy = false;
        let mut has_degraded = false;

        for health in subsystems.values() {
            match health.status {
                HealthStatus::Unhealthy => has_unhealthy = true,
                HealthStatus::Degraded => has_degraded = true,
                HealthStatus::Healthy => {}
            }

            // Check for stale subsystems
            if health.is_stale(self.config.stale_threshold) {
                warn!("Subsystem {} is stale (last check {:?} ago)", health.name, health.time_since_check());
                has_degraded = true;
            }
        }

        if has_unhealthy {
            HealthStatus::Unhealthy
        } else if has_degraded {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }

    /// Get all subsystem health statuses
    pub fn all_subsystems(&self) -> HashMap<String, SubsystemHealth> {
        self.subsystems.read().clone()
    }

    /// Get count of subsystems by status
    pub fn status_counts(&self) -> HashMap<HealthStatus, usize> {
        let subsystems = self.subsystems.read();
        let mut counts = HashMap::new();

        for health in subsystems.values() {
            *counts.entry(health.status).or_insert(0) += 1;
        }

        counts
    }

    /// Get unhealthy subsystems
    pub fn unhealthy_subsystems(&self) -> Vec<String> {
        let subsystems = self.subsystems.read();
        subsystems
            .values()
            .filter(|h| h.status.is_unhealthy())
            .map(|h| h.name.clone())
            .collect()
    }

    /// Get degraded subsystems
    pub fn degraded_subsystems(&self) -> Vec<String> {
        let subsystems = self.subsystems.read();
        subsystems
            .values()
            .filter(|h| h.status.is_degraded())
            .map(|h| h.name.clone())
            .collect()
    }

    /// Update subsystem metadata
    pub fn update_metadata(
        &self,
        name: &str,
        key: String,
        value: String,
    ) -> Result<(), HealthError> {
        let mut subsystems = self.subsystems.write();
        let health = subsystems
            .get_mut(name)
            .ok_or_else(|| HealthError::SubsystemNotFound(name.to_string()))?;

        health.metadata.insert(key, value);
        Ok(())
    }

    /// Get subsystem count
    pub fn subsystem_count(&self) -> usize {
        self.subsystems.read().len()
    }

    /// Clear all subsystems
    pub fn clear(&self) {
        self.subsystems.write().clear();
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_health_monitor_creation() {
        let monitor = HealthMonitor::default();
        assert_eq!(monitor.subsystem_count(), 0);
        assert_eq!(monitor.overall_status(), HealthStatus::Healthy);
    }

    #[test]
    fn test_register_subsystem() {
        let monitor = HealthMonitor::default();
        monitor.register_subsystem("test-subsystem".to_string());
        assert_eq!(monitor.subsystem_count(), 1);

        let health = monitor.check_health("test-subsystem").unwrap();
        assert_eq!(health.name, "test-subsystem");
        assert_eq!(health.status, HealthStatus::Healthy);
    }

    #[test]
    fn test_register_duplicate_subsystem() {
        let monitor = HealthMonitor::default();
        monitor.register_subsystem("test".to_string());
        monitor.register_subsystem("test".to_string());
        assert_eq!(monitor.subsystem_count(), 1);
    }

    #[test]
    fn test_unregister_subsystem() {
        let monitor = HealthMonitor::default();
        monitor.register_subsystem("test".to_string());
        assert_eq!(monitor.subsystem_count(), 1);

        monitor.unregister_subsystem("test").unwrap();
        assert_eq!(monitor.subsystem_count(), 0);
    }

    #[test]
    fn test_update_health_success() {
        let monitor = HealthMonitor::default();
        monitor.register_subsystem("test".to_string());

        monitor.update_health("test", true).unwrap();

        let health = monitor.check_health("test").unwrap();
        assert_eq!(health.consecutive_successes, 1);
        assert_eq!(health.total_checks, 1);
        assert_eq!(health.status, HealthStatus::Healthy);
    }

    #[test]
    fn test_update_health_failure() {
        let config = HealthMonitorConfig {
            degraded_threshold: 2,
            unhealthy_threshold: 4,
            ..Default::default()
        };
        let monitor = HealthMonitor::new(config);
        monitor.register_subsystem("test".to_string());

        // First failure - still healthy
        monitor.update_health("test", false).unwrap();
        let health = monitor.check_health("test").unwrap();
        assert_eq!(health.status, HealthStatus::Healthy);

        // Second failure - degraded
        monitor.update_health("test", false).unwrap();
        let health = monitor.check_health("test").unwrap();
        assert_eq!(health.status, HealthStatus::Degraded);

        // More failures - unhealthy
        monitor.update_health("test", false).unwrap();
        monitor.update_health("test", false).unwrap();
        let health = monitor.check_health("test").unwrap();
        assert_eq!(health.status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_recovery_from_degraded() {
        let config = HealthMonitorConfig {
            degraded_threshold: 2,
            recovery_threshold: 3,
            ..Default::default()
        };
        let monitor = HealthMonitor::new(config);
        monitor.register_subsystem("test".to_string());

        // Make it degraded
        monitor.update_health("test", false).unwrap();
        monitor.update_health("test", false).unwrap();
        assert_eq!(
            monitor.check_health("test").unwrap().status,
            HealthStatus::Degraded
        );

        // Recover with successes
        monitor.update_health("test", true).unwrap();
        monitor.update_health("test", true).unwrap();
        assert_eq!(
            monitor.check_health("test").unwrap().status,
            HealthStatus::Degraded
        );

        monitor.update_health("test", true).unwrap();
        assert_eq!(
            monitor.check_health("test").unwrap().status,
            HealthStatus::Healthy
        );
    }

    #[test]
    fn test_overall_status_healthy() {
        let monitor = HealthMonitor::default();
        monitor.register_subsystem("test1".to_string());
        monitor.register_subsystem("test2".to_string());

        monitor.update_health("test1", true).unwrap();
        monitor.update_health("test2", true).unwrap();

        assert_eq!(monitor.overall_status(), HealthStatus::Healthy);
    }

    #[test]
    fn test_overall_status_degraded() {
        let config = HealthMonitorConfig {
            degraded_threshold: 2,
            ..Default::default()
        };
        let monitor = HealthMonitor::new(config);
        monitor.register_subsystem("test1".to_string());
        monitor.register_subsystem("test2".to_string());

        monitor.update_health("test1", true).unwrap();

        // Make test2 degraded
        monitor.update_health("test2", false).unwrap();
        monitor.update_health("test2", false).unwrap();

        assert_eq!(monitor.overall_status(), HealthStatus::Degraded);
    }

    #[test]
    fn test_overall_status_unhealthy() {
        let config = HealthMonitorConfig {
            unhealthy_threshold: 2,
            ..Default::default()
        };
        let monitor = HealthMonitor::new(config);
        monitor.register_subsystem("test1".to_string());
        monitor.register_subsystem("test2".to_string());

        monitor.update_health("test1", true).unwrap();

        // Make test2 unhealthy
        monitor.update_health("test2", false).unwrap();
        monitor.update_health("test2", false).unwrap();

        assert_eq!(monitor.overall_status(), HealthStatus::Unhealthy);
    }

    #[test]
    fn test_status_counts() {
        let config = HealthMonitorConfig {
            degraded_threshold: 2,
            unhealthy_threshold: 4,
            ..Default::default()
        };
        let monitor = HealthMonitor::new(config);

        monitor.register_subsystem("healthy".to_string());
        monitor.register_subsystem("degraded".to_string());
        monitor.register_subsystem("unhealthy".to_string());

        // Make degraded
        monitor.update_health("degraded", false).unwrap();
        monitor.update_health("degraded", false).unwrap();

        // Make unhealthy
        for _ in 0..4 {
            monitor.update_health("unhealthy", false).unwrap();
        }

        let counts = monitor.status_counts();
        assert_eq!(counts.get(&HealthStatus::Healthy), Some(&1));
        assert_eq!(counts.get(&HealthStatus::Degraded), Some(&1));
        assert_eq!(counts.get(&HealthStatus::Unhealthy), Some(&1));
    }

    #[test]
    fn test_unhealthy_subsystems() {
        let config = HealthMonitorConfig {
            unhealthy_threshold: 2,
            ..Default::default()
        };
        let monitor = HealthMonitor::new(config);

        monitor.register_subsystem("healthy".to_string());
        monitor.register_subsystem("unhealthy1".to_string());
        monitor.register_subsystem("unhealthy2".to_string());

        monitor.update_health("unhealthy1", false).unwrap();
        monitor.update_health("unhealthy1", false).unwrap();
        monitor.update_health("unhealthy2", false).unwrap();
        monitor.update_health("unhealthy2", false).unwrap();

        let unhealthy = monitor.unhealthy_subsystems();
        assert_eq!(unhealthy.len(), 2);
        assert!(unhealthy.contains(&"unhealthy1".to_string()));
        assert!(unhealthy.contains(&"unhealthy2".to_string()));
    }

    #[test]
    fn test_metadata() {
        let monitor = HealthMonitor::default();
        monitor.register_subsystem("test".to_string());

        monitor
            .update_metadata("test", "version".to_string(), "1.0.0".to_string())
            .unwrap();

        let health = monitor.check_health("test").unwrap();
        assert_eq!(health.metadata.get("version"), Some(&"1.0.0".to_string()));
    }

    #[test]
    fn test_failure_rate() {
        let monitor = HealthMonitor::default();
        monitor.register_subsystem("test".to_string());

        monitor.update_health("test", true).unwrap();
        monitor.update_health("test", false).unwrap();
        monitor.update_health("test", true).unwrap();
        monitor.update_health("test", false).unwrap();

        let health = monitor.check_health("test").unwrap();
        assert_eq!(health.failure_rate(), 50.0);
    }

    #[test]
    fn test_stale_detection() {
        let config = HealthMonitorConfig {
            stale_threshold: Duration::from_millis(100),
            ..Default::default()
        };
        let monitor = HealthMonitor::new(config);
        monitor.register_subsystem("test".to_string());

        // Initial state - not stale
        let health = monitor.check_health("test").unwrap();
        assert!(!health.is_stale(Duration::from_millis(100)));

        // Wait and check stale
        sleep(Duration::from_millis(150));
        let health = monitor.check_health("test").unwrap();
        assert!(health.is_stale(Duration::from_millis(100)));
    }

    #[test]
    fn test_set_health_manually() {
        let monitor = HealthMonitor::default();
        monitor.register_subsystem("test".to_string());

        monitor
            .set_health("test", HealthStatus::Degraded)
            .unwrap();

        let health = monitor.check_health("test").unwrap();
        assert_eq!(health.status, HealthStatus::Degraded);
    }

    #[test]
    fn test_clear_subsystems() {
        let monitor = HealthMonitor::default();
        monitor.register_subsystem("test1".to_string());
        monitor.register_subsystem("test2".to_string());
        assert_eq!(monitor.subsystem_count(), 2);

        monitor.clear();
        assert_eq!(monitor.subsystem_count(), 0);
    }

    #[test]
    fn test_health_status_score() {
        assert_eq!(HealthStatus::Healthy.score(), 100);
        assert_eq!(HealthStatus::Degraded.score(), 50);
        assert_eq!(HealthStatus::Unhealthy.score(), 0);
    }

    #[test]
    fn test_subsystem_not_found_error() {
        let monitor = HealthMonitor::default();

        let result = monitor.check_health("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result, Err(HealthError::SubsystemNotFound(_))));
    }
}
