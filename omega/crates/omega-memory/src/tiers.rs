//! Memory tier definitions and characteristics

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 12-tier memory hierarchy spanning from instant to omega scale
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[repr(u8)]
pub enum MemoryTier {
    // === Individual Scale (Tier 1-4) ===
    /// Tier 1: Instant Memory - Milliseconds to seconds
    /// Working memory, immediate context, attention buffer
    Instant = 1,

    /// Tier 2: Session Memory - Minutes to hours
    /// Current conversation, active task context
    Session = 2,

    /// Tier 3: Episodic Memory - Days to weeks
    /// Specific events, experiences, conversations
    Episodic = 3,

    /// Tier 4: Semantic Memory - Weeks to months
    /// Facts, concepts, learned knowledge
    Semantic = 4,

    // === Species Scale (Tier 5-8) ===
    /// Tier 5: Collective Memory - Months to years
    /// Shared knowledge across agent instances
    Collective = 5,

    /// Tier 6: Evolutionary Memory - Years to decades
    /// Learned patterns, successful strategies
    Evolutionary = 6,

    /// Tier 7: Architectural Memory - Decades to centuries
    /// Core algorithms, system designs
    Architectural = 7,

    /// Tier 8: Substrate Memory - Centuries to millennia
    /// Fundamental computation patterns
    Substrate = 8,

    // === Cosmic Scale (Tier 9-12) ===
    /// Tier 9: Civilizational Memory - Millennia to epochs
    /// Cultural knowledge, civilization patterns
    Civilizational = 9,

    /// Tier 10: Temporal Memory - Epochs to eons
    /// Historical trends, long-term patterns
    Temporal = 10,

    /// Tier 11: Physical Memory - Eons to universe-scale
    /// Physical laws, universal constants
    Physical = 11,

    /// Tier 12: Omega Memory - Eternal/Universal
    /// Fundamental truths, universal principles
    Omega = 12,
}

impl MemoryTier {
    /// Get the typical retention duration for this tier
    pub fn retention_duration(&self) -> Option<Duration> {
        match self {
            Self::Instant => Some(Duration::from_secs(60)), // 1 minute
            Self::Session => Some(Duration::from_secs(3600 * 24)), // 24 hours
            Self::Episodic => Some(Duration::from_secs(3600 * 24 * 30)), // 30 days
            Self::Semantic => Some(Duration::from_secs(3600 * 24 * 365)), // 1 year
            Self::Collective => Some(Duration::from_secs(3600 * 24 * 365 * 10)), // 10 years
            _ => None, // Higher tiers have no automatic expiration
        }
    }

    /// Get the importance threshold for this tier
    pub fn importance_threshold(&self) -> f64 {
        match self {
            Self::Instant => 0.0,
            Self::Session => 0.1,
            Self::Episodic => 0.3,
            Self::Semantic => 0.5,
            Self::Collective => 0.6,
            Self::Evolutionary => 0.7,
            Self::Architectural => 0.8,
            Self::Substrate => 0.85,
            Self::Civilizational => 0.9,
            Self::Temporal => 0.93,
            Self::Physical => 0.96,
            Self::Omega => 0.99,
        }
    }

    /// Get the typical size of this memory tier
    pub fn typical_size(&self) -> usize {
        match self {
            Self::Instant => 1_000,        // ~1K memories
            Self::Session => 10_000,       // ~10K memories
            Self::Episodic => 100_000,     // ~100K memories
            Self::Semantic => 1_000_000,   // ~1M memories
            Self::Collective => 10_000_000, // ~10M memories
            Self::Evolutionary => 100_000_000, // ~100M memories
            _ => usize::MAX, // Higher tiers are unbounded
        }
    }

    /// Get the scale category
    pub fn scale(&self) -> MemoryScale {
        match self {
            Self::Instant | Self::Session | Self::Episodic | Self::Semantic => MemoryScale::Individual,
            Self::Collective | Self::Evolutionary | Self::Architectural | Self::Substrate => MemoryScale::Species,
            Self::Civilizational | Self::Temporal | Self::Physical | Self::Omega => MemoryScale::Cosmic,
        }
    }

    /// Get the next tier for consolidation
    pub fn next_tier(&self) -> Option<MemoryTier> {
        match self {
            Self::Instant => Some(Self::Session),
            Self::Session => Some(Self::Episodic),
            Self::Episodic => Some(Self::Semantic),
            Self::Semantic => Some(Self::Collective),
            Self::Collective => Some(Self::Evolutionary),
            Self::Evolutionary => Some(Self::Architectural),
            Self::Architectural => Some(Self::Substrate),
            Self::Substrate => Some(Self::Civilizational),
            Self::Civilizational => Some(Self::Temporal),
            Self::Temporal => Some(Self::Physical),
            Self::Physical => Some(Self::Omega),
            Self::Omega => None, // No tier beyond Omega
        }
    }

    /// Get all tiers in this scale
    pub fn tiers_in_scale(scale: MemoryScale) -> Vec<MemoryTier> {
        match scale {
            MemoryScale::Individual => vec![
                Self::Instant,
                Self::Session,
                Self::Episodic,
                Self::Semantic,
            ],
            MemoryScale::Species => vec![
                Self::Collective,
                Self::Evolutionary,
                Self::Architectural,
                Self::Substrate,
            ],
            MemoryScale::Cosmic => vec![
                Self::Civilizational,
                Self::Temporal,
                Self::Physical,
                Self::Omega,
            ],
        }
    }

    /// Get all tiers
    pub fn all() -> Vec<MemoryTier> {
        vec![
            Self::Instant,
            Self::Session,
            Self::Episodic,
            Self::Semantic,
            Self::Collective,
            Self::Evolutionary,
            Self::Architectural,
            Self::Substrate,
            Self::Civilizational,
            Self::Temporal,
            Self::Physical,
            Self::Omega,
        ]
    }
}

/// Memory scale categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryScale {
    /// Individual agent scale (Tier 1-4)
    Individual,
    /// Species/collective scale (Tier 5-8)
    Species,
    /// Cosmic/universal scale (Tier 9-12)
    Cosmic,
}

impl std::fmt::Display for MemoryTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Instant => write!(f, "Instant (T1)"),
            Self::Session => write!(f, "Session (T2)"),
            Self::Episodic => write!(f, "Episodic (T3)"),
            Self::Semantic => write!(f, "Semantic (T4)"),
            Self::Collective => write!(f, "Collective (T5)"),
            Self::Evolutionary => write!(f, "Evolutionary (T6)"),
            Self::Architectural => write!(f, "Architectural (T7)"),
            Self::Substrate => write!(f, "Substrate (T8)"),
            Self::Civilizational => write!(f, "Civilizational (T9)"),
            Self::Temporal => write!(f, "Temporal (T10)"),
            Self::Physical => write!(f, "Physical (T11)"),
            Self::Omega => write!(f, "Omega (T12)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_ordering() {
        assert!(MemoryTier::Instant < MemoryTier::Omega);
        assert!(MemoryTier::Session < MemoryTier::Episodic);
    }

    #[test]
    fn test_tier_progression() {
        assert_eq!(MemoryTier::Instant.next_tier(), Some(MemoryTier::Session));
        assert_eq!(MemoryTier::Omega.next_tier(), None);
    }

    #[test]
    fn test_scale_categories() {
        assert_eq!(MemoryTier::Instant.scale(), MemoryScale::Individual);
        assert_eq!(MemoryTier::Collective.scale(), MemoryScale::Species);
        assert_eq!(MemoryTier::Omega.scale(), MemoryScale::Cosmic);
    }
}
