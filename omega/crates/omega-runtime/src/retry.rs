//! Retry Policy Implementation
//!
//! Provides configurable retry logic with exponential backoff and jitter
//! for handling transient failures in distributed systems.

use rand::Rng;
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, warn};

/// Retry policy configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial delay before first retry
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Backoff multiplier for exponential backoff
    pub multiplier: f64,
    /// Whether to add jitter to delays (recommended to prevent thundering herd)
    pub use_jitter: bool,
    /// Jitter factor (0.0 to 1.0, where 0.5 means +/- 50% variation)
    pub jitter_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
            use_jitter: true,
            jitter_factor: 0.3,
        }
    }
}

/// Retry policy error types
#[derive(Debug, Error)]
pub enum RetryError<E> {
    #[error("Operation failed after {attempts} attempts: {last_error}")]
    MaxRetriesExceeded { attempts: u32, last_error: E },
    #[error("Retry aborted: {0}")]
    Aborted(String),
}

/// Retry policy for executing operations with automatic retries
pub struct RetryPolicy {
    config: RetryConfig,
}

impl RetryPolicy {
    /// Create a new retry policy with the given configuration
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Create a retry policy with default configuration
    pub fn default() -> Self {
        Self {
            config: RetryConfig::default(),
        }
    }

    /// Execute an async operation with retry logic
    pub async fn execute<F, Fut, T, E>(&self, mut operation: F) -> Result<T, RetryError<E>>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display + Clone,
    {
        let mut attempts = 0;
        #[allow(unused_assignments)]
        let mut _last_error: Option<E> = None;

        loop {
            attempts += 1;

            debug!("Retry attempt {}/{}", attempts, self.config.max_retries + 1);

            match operation().await {
                Ok(result) => {
                    if attempts > 1 {
                        debug!("Operation succeeded after {} attempts", attempts);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    warn!("Attempt {} failed: {}", attempts, e);

                    if attempts > self.config.max_retries {
                        return Err(RetryError::MaxRetriesExceeded {
                            attempts,
                            last_error: e,
                        });
                    }

                    _last_error = Some(e);

                    // Calculate delay with exponential backoff
                    let delay = self.calculate_delay(attempts);
                    debug!("Waiting {:?} before retry", delay);

                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    /// Execute a synchronous operation with retry logic
    pub fn execute_sync<F, T, E>(&self, mut operation: F) -> Result<T, RetryError<E>>
    where
        F: FnMut() -> Result<T, E>,
        E: std::fmt::Display + Clone,
    {
        let mut attempts = 0;
        #[allow(unused_assignments)]
        let mut _last_error: Option<E> = None;

        loop {
            attempts += 1;

            debug!("Retry attempt {}/{}", attempts, self.config.max_retries + 1);

            match operation() {
                Ok(result) => {
                    if attempts > 1 {
                        debug!("Operation succeeded after {} attempts", attempts);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    warn!("Attempt {} failed: {}", attempts, e);

                    if attempts > self.config.max_retries {
                        return Err(RetryError::MaxRetriesExceeded {
                            attempts,
                            last_error: e,
                        });
                    }

                    _last_error = Some(e);

                    // Calculate delay with exponential backoff
                    let delay = self.calculate_delay(attempts);
                    debug!("Waiting {:?} before retry", delay);

                    std::thread::sleep(delay);
                }
            }
        }
    }

    /// Execute with custom retry condition
    pub async fn execute_with_condition<F, Fut, T, E, C>(
        &self,
        mut operation: F,
        mut should_retry: C,
    ) -> Result<T, RetryError<E>>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display + Clone,
        C: FnMut(&E) -> bool,
    {
        let mut attempts = 0;

        loop {
            attempts += 1;

            debug!("Retry attempt {}/{}", attempts, self.config.max_retries + 1);

            match operation().await {
                Ok(result) => {
                    if attempts > 1 {
                        debug!("Operation succeeded after {} attempts", attempts);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    if !should_retry(&e) {
                        debug!("Error is not retryable: {}", e);
                        return Err(RetryError::Aborted(format!(
                            "Non-retryable error after {} attempts: {}",
                            attempts, e
                        )));
                    }

                    warn!("Attempt {} failed: {}", attempts, e);

                    if attempts > self.config.max_retries {
                        return Err(RetryError::MaxRetriesExceeded {
                            attempts,
                            last_error: e,
                        });
                    }

                    // Calculate delay with exponential backoff
                    let delay = self.calculate_delay(attempts);
                    debug!("Waiting {:?} before retry", delay);

                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    /// Calculate delay for the given attempt number
    fn calculate_delay(&self, attempt: u32) -> Duration {
        // Calculate base delay using exponential backoff
        let base_delay_ms = self.config.initial_delay.as_millis() as f64
            * self.config.multiplier.powi((attempt - 1) as i32);

        let base_delay = Duration::from_millis(base_delay_ms as u64);

        // Cap at max delay
        let capped_delay = if base_delay > self.config.max_delay {
            self.config.max_delay
        } else {
            base_delay
        };

        // Add jitter if enabled
        if self.config.use_jitter {
            self.add_jitter(capped_delay)
        } else {
            capped_delay
        }
    }

    /// Add jitter to a delay to prevent thundering herd
    fn add_jitter(&self, delay: Duration) -> Duration {
        let mut rng = rand::thread_rng();
        let delay_ms = delay.as_millis() as f64;

        // Calculate jitter range
        let jitter_range = delay_ms * self.config.jitter_factor;

        // Add random jitter in range [-jitter_range, +jitter_range]
        let jitter = rng.gen_range(-jitter_range..=jitter_range);
        let jittered_ms = (delay_ms + jitter).max(0.0);

        Duration::from_millis(jittered_ms as u64)
    }

    /// Get the configuration
    pub fn config(&self) -> &RetryConfig {
        &self.config
    }
}

/// Builder for retry configuration
pub struct RetryConfigBuilder {
    config: RetryConfig,
}

impl RetryConfigBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: RetryConfig::default(),
        }
    }

    /// Set maximum number of retries
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.config.max_retries = max_retries;
        self
    }

    /// Set initial delay
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.config.initial_delay = delay;
        self
    }

    /// Set maximum delay
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.config.max_delay = delay;
        self
    }

    /// Set backoff multiplier
    pub fn multiplier(mut self, multiplier: f64) -> Self {
        self.config.multiplier = multiplier;
        self
    }

    /// Enable or disable jitter
    pub fn use_jitter(mut self, use_jitter: bool) -> Self {
        self.config.use_jitter = use_jitter;
        self
    }

    /// Set jitter factor
    pub fn jitter_factor(mut self, factor: f64) -> Self {
        self.config.jitter_factor = factor.clamp(0.0, 1.0);
        self
    }

    /// Build the configuration
    pub fn build(self) -> RetryConfig {
        self.config
    }
}

impl Default for RetryConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_immediate_success() {
        let policy = RetryPolicy::default();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let result = policy
            .execute(|| async {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<_, String>("success")
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_on_failure() {
        let config = RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(10),
            ..Default::default()
        };
        let policy = RetryPolicy::new(config);
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let result = policy
            .execute(|| async {
                let count = counter_clone.fetch_add(1, Ordering::SeqCst);
                if count < 2 {
                    Err("temporary failure")
                } else {
                    Ok("success")
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_max_retries_exceeded() {
        let config = RetryConfig {
            max_retries: 2,
            initial_delay: Duration::from_millis(10),
            ..Default::default()
        };
        let policy = RetryPolicy::new(config);
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let result = policy
            .execute(|| async {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Err::<String, _>("persistent failure")
            })
            .await;

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 3); // Initial + 2 retries

        match result {
            Err(RetryError::MaxRetriesExceeded { attempts, .. }) => {
                assert_eq!(attempts, 3);
            }
            _ => panic!("Expected MaxRetriesExceeded error"),
        }
    }

    #[tokio::test]
    async fn test_exponential_backoff() {
        let config = RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(50),
            multiplier: 2.0,
            use_jitter: false,
            ..Default::default()
        };
        let policy = RetryPolicy::new(config);

        let delay1 = policy.calculate_delay(1);
        let delay2 = policy.calculate_delay(2);
        let delay3 = policy.calculate_delay(3);

        assert_eq!(delay1, Duration::from_millis(50));
        assert_eq!(delay2, Duration::from_millis(100));
        assert_eq!(delay3, Duration::from_millis(200));
    }

    #[tokio::test]
    async fn test_max_delay_cap() {
        let config = RetryConfig {
            max_retries: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(500),
            multiplier: 2.0,
            use_jitter: false,
            ..Default::default()
        };
        let policy = RetryPolicy::new(config);

        let delay5 = policy.calculate_delay(5);
        assert_eq!(delay5, Duration::from_millis(500)); // Capped at max_delay
    }

    #[tokio::test]
    async fn test_jitter_adds_variation() {
        let config = RetryConfig {
            max_retries: 1,
            initial_delay: Duration::from_millis(100),
            use_jitter: true,
            jitter_factor: 0.5,
            ..Default::default()
        };
        let policy = RetryPolicy::new(config);

        // Generate multiple delays and check they vary
        let mut delays = vec![];
        for _ in 0..10 {
            let delay = policy.calculate_delay(1);
            delays.push(delay);
        }

        // Check that delays are in reasonable range (50-150ms with 50% jitter)
        for delay in &delays {
            let ms = delay.as_millis();
            assert!(ms >= 50 && ms <= 150, "Delay {} outside expected range", ms);
        }

        // Check that we got some variation
        let all_same = delays.iter().all(|d| d == &delays[0]);
        assert!(!all_same, "All delays are the same, jitter not working");
    }

    #[tokio::test]
    async fn test_synchronous_retry() {
        let config = RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(10),
            ..Default::default()
        };
        let policy = RetryPolicy::new(config);
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let result = policy.execute_sync(|| {
            let count = counter_clone.fetch_add(1, Ordering::SeqCst);
            if count < 2 {
                Err("temporary failure")
            } else {
                Ok("success")
            }
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_with_custom_condition() {
        let config = RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(10),
            ..Default::default()
        };
        let policy = RetryPolicy::new(config);
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        // Only retry on "temporary" errors, not "permanent" ones
        let result = policy
            .execute_with_condition(
                || {
                    let counter = Arc::clone(&counter_clone);
                    async move {
                        counter.fetch_add(1, Ordering::SeqCst);
                        Err::<String, _>("permanent error")
                    }
                },
                |e| e.contains("temporary"),
            )
            .await;

        // Should abort on first attempt since error is not retryable
        assert!(matches!(result, Err(RetryError::Aborted(_))));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_builder() {
        let config = RetryConfigBuilder::new()
            .max_retries(5)
            .initial_delay(Duration::from_millis(50))
            .max_delay(Duration::from_secs(10))
            .multiplier(3.0)
            .use_jitter(false)
            .build();

        assert_eq!(config.max_retries, 5);
        assert_eq!(config.initial_delay, Duration::from_millis(50));
        assert_eq!(config.max_delay, Duration::from_secs(10));
        assert_eq!(config.multiplier, 3.0);
        assert!(!config.use_jitter);
    }

    #[tokio::test]
    async fn test_zero_retries() {
        let config = RetryConfig {
            max_retries: 0,
            initial_delay: Duration::from_millis(10),
            ..Default::default()
        };
        let policy = RetryPolicy::new(config);
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let result = policy
            .execute(|| async {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Err::<String, _>("error")
            })
            .await;

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 1); // Only initial attempt
    }

    #[tokio::test]
    async fn test_concurrent_retries() {
        let policy = Arc::new(RetryPolicy::default());
        let mut handles = vec![];

        for i in 0..5 {
            let policy_clone = Arc::clone(&policy);
            let handle = tokio::spawn(async move {
                policy_clone
                    .execute(|| async move {
                        if i % 2 == 0 {
                            Ok(format!("success {}", i))
                        } else {
                            Err(format!("error {}", i))
                        }
                    })
                    .await
            });
            handles.push(handle);
        }

        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await.unwrap();
            if i % 2 == 0 {
                assert!(result.is_ok());
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[tokio::test]
    async fn test_jitter_factor_clamping() {
        let config = RetryConfigBuilder::new()
            .jitter_factor(1.5) // Should be clamped to 1.0
            .build();

        assert_eq!(config.jitter_factor, 1.0);

        let config = RetryConfigBuilder::new()
            .jitter_factor(-0.5) // Should be clamped to 0.0
            .build();

        assert_eq!(config.jitter_factor, 0.0);
    }

    #[tokio::test]
    async fn test_timing_accuracy() {
        let config = RetryConfig {
            max_retries: 2,
            initial_delay: Duration::from_millis(100),
            multiplier: 2.0,
            use_jitter: false,
            ..Default::default()
        };
        let policy = RetryPolicy::new(config);
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let start = std::time::Instant::now();

        let _ = policy
            .execute(|| async {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Err::<String, _>("error")
            })
            .await;

        let elapsed = start.elapsed();

        // Should take at least 100ms (first retry) + 200ms (second retry) = 300ms
        assert!(
            elapsed >= Duration::from_millis(300),
            "Elapsed time {:?} less than expected",
            elapsed
        );
    }
}
