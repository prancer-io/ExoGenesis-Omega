//! Circuit Breaker Pattern Implementation
//!
//! Provides automatic failure detection and recovery for resilient operations.
//! Prevents cascading failures by temporarily blocking operations to failing subsystems.

use parking_lot::RwLock;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};
use thiserror::Error;

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, operations proceed normally
    Closed,
    /// Circuit is open, operations are blocked
    Open,
    /// Circuit is testing recovery, limited operations allowed
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of consecutive failures before opening circuit
    pub failure_threshold: u32,
    /// Number of consecutive successes in half-open state before closing
    pub success_threshold: u32,
    /// Duration to wait in open state before transitioning to half-open
    pub timeout: Duration,
    /// Maximum number of requests allowed in half-open state
    pub half_open_max_requests: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            half_open_max_requests: 3,
        }
    }
}

/// Circuit breaker error types
#[derive(Debug, Error)]
pub enum CircuitBreakerError {
    #[error("Circuit breaker is open")]
    CircuitOpen,
    #[error("Operation failed: {0}")]
    OperationFailed(String),
    #[error("Half-open limit exceeded")]
    HalfOpenLimitExceeded,
}

/// Circuit breaker for preventing cascading failures
pub struct CircuitBreaker {
    state: RwLock<CircuitState>,
    failure_count: AtomicU32,
    success_count: AtomicU32,
    half_open_requests: AtomicU32,
    last_failure: RwLock<Option<Instant>>,
    last_state_change: RwLock<Instant>,
    config: CircuitBreakerConfig,
}

impl CircuitBreaker {
    /// Create a new circuit breaker with the given configuration
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: RwLock::new(CircuitState::Closed),
            failure_count: AtomicU32::new(0),
            success_count: AtomicU32::new(0),
            half_open_requests: AtomicU32::new(0),
            last_failure: RwLock::new(None),
            last_state_change: RwLock::new(Instant::now()),
            config,
        }
    }

    /// Execute an operation with circuit breaker protection
    pub async fn call<F, T, E>(&self, operation: F) -> Result<T, CircuitBreakerError>
    where
        F: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        // Check if we can execute
        self.check_state()?;

        // Execute the operation
        match operation.await {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure();
                Err(CircuitBreakerError::OperationFailed(e.to_string()))
            }
        }
    }

    /// Execute a synchronous operation with circuit breaker protection
    pub fn call_sync<F, T, E>(&self, operation: F) -> Result<T, CircuitBreakerError>
    where
        F: FnOnce() -> Result<T, E>,
        E: std::fmt::Display,
    {
        // Check if we can execute
        self.check_state()?;

        // Execute the operation
        match operation() {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure();
                Err(CircuitBreakerError::OperationFailed(e.to_string()))
            }
        }
    }

    /// Check if operation can proceed based on current state
    fn check_state(&self) -> Result<(), CircuitBreakerError> {
        let state = *self.state.read();

        match state {
            CircuitState::Closed => Ok(()),
            CircuitState::Open => {
                // Check if timeout has elapsed
                if let Some(last_failure) = *self.last_failure.read() {
                    if last_failure.elapsed() >= self.config.timeout {
                        self.transition_to_half_open();
                        Ok(())
                    } else {
                        Err(CircuitBreakerError::CircuitOpen)
                    }
                } else {
                    Err(CircuitBreakerError::CircuitOpen)
                }
            }
            CircuitState::HalfOpen => {
                let current_requests = self.half_open_requests.load(Ordering::SeqCst);
                if current_requests < self.config.half_open_max_requests {
                    self.half_open_requests.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                } else {
                    Err(CircuitBreakerError::HalfOpenLimitExceeded)
                }
            }
        }
    }

    /// Handle successful operation
    fn on_success(&self) {
        let state = *self.state.read();

        match state {
            CircuitState::Closed => {
                self.failure_count.store(0, Ordering::SeqCst);
            }
            CircuitState::HalfOpen => {
                let success_count = self.success_count.fetch_add(1, Ordering::SeqCst) + 1;
                if success_count >= self.config.success_threshold {
                    self.transition_to_closed();
                }
            }
            CircuitState::Open => {
                // Should not happen, but reset if it does
                self.transition_to_closed();
            }
        }
    }

    /// Handle failed operation
    fn on_failure(&self) {
        let state = *self.state.read();

        match state {
            CircuitState::Closed => {
                let failure_count = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
                if failure_count >= self.config.failure_threshold {
                    self.transition_to_open();
                }
            }
            CircuitState::HalfOpen => {
                self.transition_to_open();
            }
            CircuitState::Open => {
                *self.last_failure.write() = Some(Instant::now());
            }
        }
    }

    /// Transition to closed state
    fn transition_to_closed(&self) {
        *self.state.write() = CircuitState::Closed;
        self.failure_count.store(0, Ordering::SeqCst);
        self.success_count.store(0, Ordering::SeqCst);
        self.half_open_requests.store(0, Ordering::SeqCst);
        *self.last_state_change.write() = Instant::now();
    }

    /// Transition to open state
    fn transition_to_open(&self) {
        *self.state.write() = CircuitState::Open;
        *self.last_failure.write() = Some(Instant::now());
        self.success_count.store(0, Ordering::SeqCst);
        self.half_open_requests.store(0, Ordering::SeqCst);
        *self.last_state_change.write() = Instant::now();
    }

    /// Transition to half-open state
    fn transition_to_half_open(&self) {
        *self.state.write() = CircuitState::HalfOpen;
        self.failure_count.store(0, Ordering::SeqCst);
        self.success_count.store(0, Ordering::SeqCst);
        self.half_open_requests.store(0, Ordering::SeqCst);
        *self.last_state_change.write() = Instant::now();
    }

    /// Reset the circuit breaker to closed state
    pub fn reset(&self) {
        self.transition_to_closed();
    }

    /// Get current circuit state
    pub fn state(&self) -> CircuitState {
        *self.state.read()
    }

    /// Get current failure count
    pub fn failure_count(&self) -> u32 {
        self.failure_count.load(Ordering::SeqCst)
    }

    /// Get current success count
    pub fn success_count(&self) -> u32 {
        self.success_count.load(Ordering::SeqCst)
    }

    /// Get time since last state change
    pub fn time_since_state_change(&self) -> Duration {
        self.last_state_change.read().elapsed()
    }

    /// Get time since last failure
    pub fn time_since_last_failure(&self) -> Option<Duration> {
        self.last_failure.read().map(|instant| instant.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_circuit_breaker_starts_closed() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig::default());
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_successful_operation_keeps_circuit_closed() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig::default());

        for _ in 0..10 {
            let result = cb.call(async { Ok::<_, String>("success") }).await;
            assert!(result.is_ok());
            assert_eq!(cb.state(), CircuitState::Closed);
        }
    }

    #[tokio::test]
    async fn test_circuit_opens_after_threshold_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        // First 2 failures should keep circuit closed
        for _ in 0..2 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
            assert_eq!(cb.state(), CircuitState::Closed);
        }

        // Third failure should open circuit
        let _ = cb.call(async { Err::<String, _>("error") }).await;
        assert_eq!(cb.state(), CircuitState::Open);
    }

    #[tokio::test]
    async fn test_circuit_rejects_operations_when_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            timeout: Duration::from_secs(10),
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        // Trigger failures to open circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
        }

        assert_eq!(cb.state(), CircuitState::Open);

        // Should reject operations
        let result = cb.call(async { Ok::<_, String>("success") }).await;
        assert!(matches!(result, Err(CircuitBreakerError::CircuitOpen)));
    }

    #[tokio::test]
    async fn test_circuit_transitions_to_half_open_after_timeout() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            timeout: Duration::from_millis(100),
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
        }

        assert_eq!(cb.state(), CircuitState::Open);

        // Wait for timeout
        sleep(Duration::from_millis(150)).await;

        // Next call should transition to half-open
        let result = cb.call(async { Ok::<_, String>("success") }).await;
        assert!(result.is_ok());
        assert_eq!(cb.state(), CircuitState::HalfOpen);
    }

    #[tokio::test]
    async fn test_half_open_closes_after_success_threshold() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
        }

        // Wait for timeout
        sleep(Duration::from_millis(150)).await;

        // First success (transitions to half-open)
        let _ = cb.call(async { Ok::<_, String>("success") }).await;
        assert_eq!(cb.state(), CircuitState::HalfOpen);

        // Second success (should close circuit)
        let _ = cb.call(async { Ok::<_, String>("success") }).await;
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_half_open_reopens_on_failure() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            timeout: Duration::from_millis(100),
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
        }

        // Wait for timeout
        sleep(Duration::from_millis(150)).await;

        // Transition to half-open with success
        let _ = cb.call(async { Ok::<_, String>("success") }).await;
        assert_eq!(cb.state(), CircuitState::HalfOpen);

        // Failure should reopen circuit
        let _ = cb.call(async { Err::<String, _>("error") }).await;
        assert_eq!(cb.state(), CircuitState::Open);
    }

    #[tokio::test]
    async fn test_reset_closes_circuit() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
        }

        assert_eq!(cb.state(), CircuitState::Open);

        // Reset should close circuit
        cb.reset();
        assert_eq!(cb.state(), CircuitState::Closed);
        assert_eq!(cb.failure_count(), 0);
    }

    #[tokio::test]
    async fn test_synchronous_operations() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig::default());

        let result = cb.call_sync(|| Ok::<_, String>("success"));
        assert!(result.is_ok());

        let result = cb.call_sync(|| Err::<String, _>("error"));
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let cb = Arc::new(CircuitBreaker::new(CircuitBreakerConfig::default()));
        let mut handles = vec![];

        for i in 0..10 {
            let cb_clone = Arc::clone(&cb);
            let handle = tokio::spawn(async move {
                cb_clone
                    .call(async move {
                        sleep(Duration::from_millis(10)).await;
                        Ok::<_, String>(format!("success {}", i))
                    })
                    .await
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }

        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_half_open_max_requests_limit() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            timeout: Duration::from_millis(100),
            half_open_max_requests: 1,
            success_threshold: 10, // Make sure it doesn't close too quickly
            ..Default::default()
        };
        let cb = Arc::new(CircuitBreaker::new(config));

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
        }

        assert_eq!(cb.state(), CircuitState::Open);

        // Wait for timeout to transition to half-open
        sleep(Duration::from_millis(150)).await;

        // First request transitions to half-open and succeeds (doesn't count towards limit)
        let result = cb.call(async { Ok::<_, String>("success") }).await;
        assert!(result.is_ok());
        assert_eq!(cb.state(), CircuitState::HalfOpen);

        // Second request should succeed (first request counted towards limit)
        let result = cb.call(async { Ok::<_, String>("success") }).await;
        assert!(result.is_ok());

        // Third request should be rejected as we've reached the limit of 1
        let result = cb.call(async { Ok::<_, String>("success") }).await;
        assert!(matches!(
            result,
            Err(CircuitBreakerError::HalfOpenLimitExceeded)
        ));
    }

    #[tokio::test]
    async fn test_failure_count_increments() {
        let config = CircuitBreakerConfig {
            failure_threshold: 5,
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        assert_eq!(cb.failure_count(), 0);

        for i in 1..=3 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
            assert_eq!(cb.failure_count(), i);
        }
    }

    #[tokio::test]
    async fn test_success_resets_failure_count_when_closed() {
        let config = CircuitBreakerConfig {
            failure_threshold: 5,
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        // Generate some failures
        for _ in 0..3 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
        }

        assert_eq!(cb.failure_count(), 3);

        // Success should reset count
        let _ = cb.call(async { Ok::<_, String>("success") }).await;
        assert_eq!(cb.failure_count(), 0);
    }

    #[tokio::test]
    async fn test_time_tracking() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<String, _>("error") }).await;
        }

        sleep(Duration::from_millis(50)).await;

        let time_since_change = cb.time_since_state_change();
        assert!(time_since_change >= Duration::from_millis(50));

        let time_since_failure = cb.time_since_last_failure();
        assert!(time_since_failure.is_some());
        assert!(time_since_failure.unwrap() >= Duration::from_millis(50));
    }
}
