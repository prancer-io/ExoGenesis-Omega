//! Feature Degradation Management
//!
//! Provides graceful degradation of features when subsystems fail,
//! with fallback mechanisms and feature flag management.

use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, info, warn};

/// Degradation error types
#[derive(Debug, Error)]
pub enum DegradationError {
    #[error("Feature not found: {0}")]
    FeatureNotFound(String),
    #[error("Feature already registered: {0}")]
    FeatureAlreadyRegistered(String),
    #[error("Fallback execution failed: {0}")]
    FallbackFailed(String),
    #[error("Feature is disabled: {0}")]
    FeatureDisabled(String),
}

/// Fallback function type
pub type FallbackFn = Arc<dyn Fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>;

/// Feature metadata
#[derive(Clone)]
pub struct FeatureInfo {
    /// Feature name
    pub name: String,
    /// Whether feature is currently enabled
    pub enabled: bool,
    /// Number of times feature has been disabled
    pub disable_count: u32,
    /// Number of times feature has been re-enabled
    pub enable_count: u32,
    /// Number of times fallback has been executed
    pub fallback_executions: u32,
    /// Optional description
    pub description: Option<String>,
}

impl FeatureInfo {
    /// Create new feature info
    pub fn new(name: String) -> Self {
        Self {
            name,
            enabled: true,
            disable_count: 0,
            enable_count: 0,
            fallback_executions: 0,
            description: None,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

/// Degradation manager for feature flags and fallback management
pub struct DegradationManager {
    /// Set of disabled features
    disabled_features: RwLock<HashSet<String>>,
    /// Map of features to fallback functions
    fallbacks: RwLock<HashMap<String, FallbackFn>>,
    /// Feature metadata and statistics
    features: RwLock<HashMap<String, FeatureInfo>>,
    /// Feature dependencies (features that depend on others)
    dependencies: RwLock<HashMap<String, Vec<String>>>,
}

impl DegradationManager {
    /// Create a new degradation manager
    pub fn new() -> Self {
        Self {
            disabled_features: RwLock::new(HashSet::new()),
            fallbacks: RwLock::new(HashMap::new()),
            features: RwLock::new(HashMap::new()),
            dependencies: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new feature
    pub fn register_feature(&self, name: String) -> Result<(), DegradationError> {
        let mut features = self.features.write();
        if features.contains_key(&name) {
            return Err(DegradationError::FeatureAlreadyRegistered(name));
        }
        features.insert(name.clone(), FeatureInfo::new(name.clone()));
        debug!("Registered feature: {}", name);
        Ok(())
    }

    /// Register a feature with description
    pub fn register_feature_with_description(
        &self,
        name: String,
        description: String,
    ) -> Result<(), DegradationError> {
        let mut features = self.features.write();
        if features.contains_key(&name) {
            return Err(DegradationError::FeatureAlreadyRegistered(name));
        }
        features.insert(
            name.clone(),
            FeatureInfo::new(name.clone()).with_description(description),
        );
        debug!("Registered feature with description: {}", name);
        Ok(())
    }

    /// Disable a feature
    pub fn disable_feature(&self, name: &str) -> Result<(), DegradationError> {
        // Check if feature exists
        let mut features = self.features.write();
        let feature = features
            .get_mut(name)
            .ok_or_else(|| DegradationError::FeatureNotFound(name.to_string()))?;

        if feature.enabled {
            feature.enabled = false;
            feature.disable_count += 1;
            drop(features);

            self.disabled_features.write().insert(name.to_string());
            warn!("Disabled feature: {}", name);

            // Disable dependent features
            self.disable_dependents(name)?;
        }

        Ok(())
    }

    /// Enable a feature
    pub fn enable_feature(&self, name: &str) -> Result<(), DegradationError> {
        // Check if feature exists
        let mut features = self.features.write();
        let feature = features
            .get_mut(name)
            .ok_or_else(|| DegradationError::FeatureNotFound(name.to_string()))?;

        if !feature.enabled {
            feature.enabled = true;
            feature.enable_count += 1;
            drop(features);

            self.disabled_features.write().remove(name);
            info!("Enabled feature: {}", name);
        }

        Ok(())
    }

    /// Check if a feature is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        !self.disabled_features.read().contains(name)
    }

    /// Register a fallback function for a feature
    pub fn register_fallback<F>(&self, name: String, fallback: F) -> Result<(), DegradationError>
    where
        F: Fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync + 'static,
    {
        // Ensure feature is registered
        if !self.features.read().contains_key(&name) {
            self.register_feature(name.clone())?;
        }

        self.fallbacks
            .write()
            .insert(name.clone(), Arc::new(fallback));
        debug!("Registered fallback for feature: {}", name);
        Ok(())
    }

    /// Execute fallback for a feature
    pub fn execute_fallback(&self, name: &str) -> Result<(), DegradationError> {
        // Update statistics
        if let Some(feature) = self.features.write().get_mut(name) {
            feature.fallback_executions += 1;
        }

        let fallbacks = self.fallbacks.read();
        let fallback = fallbacks
            .get(name)
            .ok_or_else(|| DegradationError::FeatureNotFound(name.to_string()))?;

        debug!("Executing fallback for feature: {}", name);

        fallback().map_err(|e| DegradationError::FallbackFailed(e.to_string()))
    }

    /// Execute operation with feature check and fallback
    pub fn execute_with_fallback<F, T>(
        &self,
        feature_name: &str,
        operation: F,
    ) -> Result<T, DegradationError>
    where
        F: FnOnce() -> Result<T, Box<dyn std::error::Error + Send + Sync>>,
        T: Default,
    {
        if self.is_enabled(feature_name) {
            match operation() {
                Ok(result) => Ok(result),
                Err(e) => {
                    warn!("Operation failed for feature {}: {}", feature_name, e);
                    self.execute_fallback(feature_name)?;
                    Ok(T::default())
                }
            }
        } else {
            self.execute_fallback(feature_name)?;
            Ok(T::default())
        }
    }

    /// Register feature dependency
    pub fn add_dependency(
        &self,
        feature: String,
        depends_on: String,
    ) -> Result<(), DegradationError> {
        // Ensure both features exist
        if !self.features.read().contains_key(&feature) {
            return Err(DegradationError::FeatureNotFound(feature));
        }
        if !self.features.read().contains_key(&depends_on) {
            return Err(DegradationError::FeatureNotFound(depends_on));
        }

        self.dependencies
            .write()
            .entry(depends_on.clone())
            .or_default()
            .push(feature.clone());

        debug!("Added dependency: {} depends on {}", feature, depends_on);
        Ok(())
    }

    /// Disable all features that depend on a given feature
    fn disable_dependents(&self, feature: &str) -> Result<(), DegradationError> {
        let dependencies = self.dependencies.read();
        if let Some(dependents) = dependencies.get(feature) {
            let dependents_clone = dependents.clone();
            drop(dependencies);

            for dependent in dependents_clone {
                warn!(
                    "Disabling dependent feature {} because {} is disabled",
                    dependent, feature
                );
                self.disable_feature(&dependent)?;
            }
        }

        Ok(())
    }

    /// Get feature information
    pub fn feature_info(&self, name: &str) -> Result<FeatureInfo, DegradationError> {
        self.features
            .read()
            .get(name)
            .cloned()
            .ok_or_else(|| DegradationError::FeatureNotFound(name.to_string()))
    }

    /// Get all features
    pub fn all_features(&self) -> HashMap<String, FeatureInfo> {
        self.features.read().clone()
    }

    /// Get disabled features
    pub fn disabled_features(&self) -> Vec<String> {
        self.disabled_features.read().iter().cloned().collect()
    }

    /// Get enabled features
    pub fn enabled_features(&self) -> Vec<String> {
        let disabled = self.disabled_features.read();
        self.features
            .read()
            .keys()
            .filter(|k| !disabled.contains(*k))
            .cloned()
            .collect()
    }

    /// Get count of registered features
    pub fn feature_count(&self) -> usize {
        self.features.read().len()
    }

    /// Get count of disabled features
    pub fn disabled_count(&self) -> usize {
        self.disabled_features.read().len()
    }

    /// Clear all features and fallbacks
    pub fn clear(&self) {
        self.features.write().clear();
        self.disabled_features.write().clear();
        self.fallbacks.write().clear();
        self.dependencies.write().clear();
    }

    /// Batch disable multiple features
    pub fn batch_disable(&self, features: &[&str]) -> Result<(), DegradationError> {
        for feature in features {
            self.disable_feature(feature)?;
        }
        Ok(())
    }

    /// Batch enable multiple features
    pub fn batch_enable(&self, features: &[&str]) -> Result<(), DegradationError> {
        for feature in features {
            self.enable_feature(feature)?;
        }
        Ok(())
    }
}

impl Default for DegradationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn test_degradation_manager_creation() {
        let manager = DegradationManager::new();
        assert_eq!(manager.feature_count(), 0);
        assert_eq!(manager.disabled_count(), 0);
    }

    #[test]
    fn test_register_feature() {
        let manager = DegradationManager::new();
        manager.register_feature("test-feature".to_string()).unwrap();
        assert_eq!(manager.feature_count(), 1);
        assert!(manager.is_enabled("test-feature"));
    }

    #[test]
    fn test_register_duplicate_feature() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();
        let result = manager.register_feature("test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_disable_feature() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();

        assert!(manager.is_enabled("test"));
        manager.disable_feature("test").unwrap();
        assert!(!manager.is_enabled("test"));
        assert_eq!(manager.disabled_count(), 1);
    }

    #[test]
    fn test_enable_feature() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();

        manager.disable_feature("test").unwrap();
        assert!(!manager.is_enabled("test"));

        manager.enable_feature("test").unwrap();
        assert!(manager.is_enabled("test"));
        assert_eq!(manager.disabled_count(), 0);
    }

    #[test]
    fn test_disable_nonexistent_feature() {
        let manager = DegradationManager::new();
        let result = manager.disable_feature("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_register_fallback() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();

        let result = manager.register_fallback("test".to_string(), || Ok(()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_fallback() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        manager
            .register_fallback("test".to_string(), move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();

        manager.execute_fallback("test").unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_execute_fallback_failure() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();

        manager
            .register_fallback("test".to_string(), || {
                Err("fallback error".into())
            })
            .unwrap();

        let result = manager.execute_fallback("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_with_fallback_success() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        manager
            .register_fallback("test".to_string(), move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();

        let result: Result<i32, DegradationError> =
            manager.execute_with_fallback("test", || Ok(42));

        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 0); // Fallback not called
    }

    #[test]
    fn test_execute_with_fallback_on_failure() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        manager
            .register_fallback("test".to_string(), move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();

        let result: Result<i32, DegradationError> =
            manager.execute_with_fallback("test", || Err("operation failed".into()));

        assert_eq!(result.unwrap(), 0); // Default value
        assert_eq!(counter.load(Ordering::SeqCst), 1); // Fallback called
    }

    #[test]
    fn test_execute_with_fallback_when_disabled() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        manager
            .register_fallback("test".to_string(), move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();

        manager.disable_feature("test").unwrap();

        let result: Result<i32, DegradationError> =
            manager.execute_with_fallback("test", || Ok(42));

        assert_eq!(result.unwrap(), 0); // Default value
        assert_eq!(counter.load(Ordering::SeqCst), 1); // Fallback called
    }

    #[test]
    fn test_feature_dependencies() {
        let manager = DegradationManager::new();
        manager.register_feature("parent".to_string()).unwrap();
        manager.register_feature("child".to_string()).unwrap();

        manager
            .add_dependency("child".to_string(), "parent".to_string())
            .unwrap();

        assert!(manager.is_enabled("parent"));
        assert!(manager.is_enabled("child"));

        // Disabling parent should disable child
        manager.disable_feature("parent").unwrap();
        assert!(!manager.is_enabled("parent"));
        assert!(!manager.is_enabled("child"));
    }

    #[test]
    fn test_feature_info() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();

        let info = manager.feature_info("test").unwrap();
        assert_eq!(info.name, "test");
        assert!(info.enabled);
        assert_eq!(info.disable_count, 0);
    }

    #[test]
    fn test_feature_statistics() {
        let manager = DegradationManager::new();
        manager.register_feature("test".to_string()).unwrap();
        manager.register_fallback("test".to_string(), || Ok(())).unwrap();

        manager.disable_feature("test").unwrap();
        manager.enable_feature("test").unwrap();
        manager.disable_feature("test").unwrap();
        manager.execute_fallback("test").unwrap();

        let info = manager.feature_info("test").unwrap();
        assert_eq!(info.disable_count, 2);
        assert_eq!(info.enable_count, 1);
        assert_eq!(info.fallback_executions, 1);
    }

    #[test]
    fn test_disabled_features_list() {
        let manager = DegradationManager::new();
        manager.register_feature("feature1".to_string()).unwrap();
        manager.register_feature("feature2".to_string()).unwrap();
        manager.register_feature("feature3".to_string()).unwrap();

        manager.disable_feature("feature1").unwrap();
        manager.disable_feature("feature3").unwrap();

        let disabled = manager.disabled_features();
        assert_eq!(disabled.len(), 2);
        assert!(disabled.contains(&"feature1".to_string()));
        assert!(disabled.contains(&"feature3".to_string()));
    }

    #[test]
    fn test_enabled_features_list() {
        let manager = DegradationManager::new();
        manager.register_feature("feature1".to_string()).unwrap();
        manager.register_feature("feature2".to_string()).unwrap();
        manager.register_feature("feature3".to_string()).unwrap();

        manager.disable_feature("feature1").unwrap();

        let enabled = manager.enabled_features();
        assert_eq!(enabled.len(), 2);
        assert!(enabled.contains(&"feature2".to_string()));
        assert!(enabled.contains(&"feature3".to_string()));
    }

    #[test]
    fn test_clear() {
        let manager = DegradationManager::new();
        manager.register_feature("test1".to_string()).unwrap();
        manager.register_feature("test2".to_string()).unwrap();
        manager.disable_feature("test1").unwrap();

        assert_eq!(manager.feature_count(), 2);
        assert_eq!(manager.disabled_count(), 1);

        manager.clear();

        assert_eq!(manager.feature_count(), 0);
        assert_eq!(manager.disabled_count(), 0);
    }

    #[test]
    fn test_batch_disable() {
        let manager = DegradationManager::new();
        manager.register_feature("f1".to_string()).unwrap();
        manager.register_feature("f2".to_string()).unwrap();
        manager.register_feature("f3".to_string()).unwrap();

        manager.batch_disable(&["f1", "f2"]).unwrap();

        assert!(!manager.is_enabled("f1"));
        assert!(!manager.is_enabled("f2"));
        assert!(manager.is_enabled("f3"));
        assert_eq!(manager.disabled_count(), 2);
    }

    #[test]
    fn test_batch_enable() {
        let manager = DegradationManager::new();
        manager.register_feature("f1".to_string()).unwrap();
        manager.register_feature("f2".to_string()).unwrap();
        manager.disable_feature("f1").unwrap();
        manager.disable_feature("f2").unwrap();

        manager.batch_enable(&["f1", "f2"]).unwrap();

        assert!(manager.is_enabled("f1"));
        assert!(manager.is_enabled("f2"));
        assert_eq!(manager.disabled_count(), 0);
    }

    #[test]
    fn test_feature_with_description() {
        let manager = DegradationManager::new();
        manager
            .register_feature_with_description(
                "test".to_string(),
                "Test feature description".to_string(),
            )
            .unwrap();

        let info = manager.feature_info("test").unwrap();
        assert_eq!(
            info.description,
            Some("Test feature description".to_string())
        );
    }

    #[test]
    fn test_multiple_dependencies() {
        let manager = DegradationManager::new();
        manager.register_feature("parent".to_string()).unwrap();
        manager.register_feature("child1".to_string()).unwrap();
        manager.register_feature("child2".to_string()).unwrap();

        manager
            .add_dependency("child1".to_string(), "parent".to_string())
            .unwrap();
        manager
            .add_dependency("child2".to_string(), "parent".to_string())
            .unwrap();

        manager.disable_feature("parent").unwrap();

        assert!(!manager.is_enabled("child1"));
        assert!(!manager.is_enabled("child2"));
    }
}
