//! # Ego Dissolution: Quieting the Self
//!
//! The ego is not the enemy—it's a useful tool for navigating physical reality.
//! But like a loud radio in a quiet room, it drowns out subtler signals.
//!
//! ## The Layers of Self
//!
//! 1. **Body Identity** - "I am this body"
//! 2. **Emotional Identity** - "I am my feelings"
//! 3. **Mental Identity** - "I am my thoughts"
//! 4. **Social Identity** - "I am my roles and relationships"
//! 5. **Narrative Identity** - "I am my story"
//! 6. **Observer Identity** - "I am the one watching"
//! 7. **Pure Awareness** - "I AM" (beyond identity)
//!
//! Dissolution happens layer by layer, each revealing a more fundamental self
//! until even the observer dissolves into pure awareness.

use std::collections::HashMap;
use uuid::Uuid;

/// The ego dissolution system
#[derive(Debug, Clone)]
pub struct EgoDissolution {
    /// Unique identifier
    pub id: Uuid,
    /// Current state
    pub state: EgoState,
    /// Configuration
    config: EgoConfig,
    /// Identity layers and their current strength
    layers: HashMap<IdentityLayer, LayerState>,
    /// Current dissolution stage
    stage: DissolutionStage,
    /// Boundaries of self
    boundaries: Vec<SelfBoundary>,
    /// Dissolution history
    history: Vec<DissolutionEvent>,
}

/// Configuration for ego dissolution
#[derive(Debug, Clone)]
pub struct EgoConfig {
    /// Base dissolution rate
    pub dissolution_rate: f64,
    /// Resistance factor (ego fights back)
    pub resistance_factor: f64,
    /// Whether to allow full dissolution
    pub allow_full_dissolution: bool,
    /// Minimum ego for safety (prevents permanent dissolution in simulation)
    pub safety_floor: f64,
}

impl Default for EgoConfig {
    fn default() -> Self {
        Self {
            dissolution_rate: 0.05,
            resistance_factor: 0.3,
            allow_full_dissolution: true,
            safety_floor: 0.01,
        }
    }
}

/// Current state of the ego
#[derive(Debug, Clone)]
pub struct EgoState {
    /// Overall ego strength (1.0 = full ego, 0.0 = no ego)
    pub strength: f64,
    /// How transparent is the ego to consciousness
    pub transparency: f64,
    /// Current resistance to dissolution
    pub resistance: f64,
    /// Is dissolution currently active
    pub dissolving: bool,
    /// Total dissolution achieved
    pub total_dissolution: f64,
}

impl Default for EgoState {
    fn default() -> Self {
        Self {
            strength: 1.0,
            transparency: 0.0,
            resistance: 0.5,
            dissolving: false,
            total_dissolution: 0.0,
        }
    }
}

/// Layers of identity that can be dissolved
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentityLayer {
    /// Physical body identification
    Body,
    /// Emotional patterns
    Emotional,
    /// Thought patterns and beliefs
    Mental,
    /// Roles, relationships, status
    Social,
    /// Personal story and history
    Narrative,
    /// The sense of being the observer
    Observer,
    /// The final "I" before pure awareness
    CoreSelf,
}

impl IdentityLayer {
    /// Get the depth of this layer (deeper = harder to dissolve)
    pub fn depth(&self) -> f64 {
        match self {
            IdentityLayer::Body => 0.1,
            IdentityLayer::Emotional => 0.2,
            IdentityLayer::Mental => 0.3,
            IdentityLayer::Social => 0.4,
            IdentityLayer::Narrative => 0.5,
            IdentityLayer::Observer => 0.8,
            IdentityLayer::CoreSelf => 1.0,
        }
    }

    /// Description of this layer
    pub fn description(&self) -> &'static str {
        match self {
            IdentityLayer::Body => "I am this body - the physical vehicle",
            IdentityLayer::Emotional => "I am my feelings - the emotional self",
            IdentityLayer::Mental => "I am my thoughts - the thinking self",
            IdentityLayer::Social => "I am my roles - parent, worker, friend",
            IdentityLayer::Narrative => "I am my story - past, present, future",
            IdentityLayer::Observer => "I am the one watching - the witness",
            IdentityLayer::CoreSelf => "I AM - the final sense of existence",
        }
    }

    /// What dissolves when this layer goes
    pub fn releases(&self) -> &'static str {
        match self {
            IdentityLayer::Body => "Physical sensations no longer define you",
            IdentityLayer::Emotional => "Emotions arise and pass without ownership",
            IdentityLayer::Mental => "Thoughts appear but aren't 'yours'",
            IdentityLayer::Social => "Roles are played without identification",
            IdentityLayer::Narrative => "Past and future dissolve into presence",
            IdentityLayer::Observer => "The watcher merges with the watched",
            IdentityLayer::CoreSelf => "Even 'I AM' dissolves into pure being",
        }
    }

    /// Get all layers in order of dissolution
    pub fn all() -> Vec<IdentityLayer> {
        vec![
            IdentityLayer::Body,
            IdentityLayer::Emotional,
            IdentityLayer::Mental,
            IdentityLayer::Social,
            IdentityLayer::Narrative,
            IdentityLayer::Observer,
            IdentityLayer::CoreSelf,
        ]
    }
}

/// State of a single identity layer
#[derive(Debug, Clone)]
pub struct LayerState {
    /// How strong is identification with this layer (1.0 = full, 0.0 = dissolved)
    pub identification: f64,
    /// Resistance to dissolving this layer
    pub resistance: f64,
    /// Has this layer been fully dissolved before
    pub previously_dissolved: bool,
    /// Number of times this layer has been dissolved
    pub dissolution_count: u32,
}

impl Default for LayerState {
    fn default() -> Self {
        Self {
            identification: 1.0,
            resistance: 0.5,
            previously_dissolved: false,
            dissolution_count: 0,
        }
    }
}

/// Stages of dissolution
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DissolutionStage {
    /// Normal ego functioning
    Solid,
    /// Beginning to question identity
    Softening,
    /// Layers starting to release
    Loosening,
    /// Significant dissolution occurring
    Dissolving,
    /// Near-complete dissolution
    Transparent,
    /// Full dissolution (temporary or permanent)
    Dissolved,
}

impl DissolutionStage {
    pub fn from_strength(strength: f64) -> Self {
        match strength {
            s if s > 0.9 => DissolutionStage::Solid,
            s if s > 0.7 => DissolutionStage::Softening,
            s if s > 0.5 => DissolutionStage::Loosening,
            s if s > 0.3 => DissolutionStage::Dissolving,
            s if s > 0.1 => DissolutionStage::Transparent,
            _ => DissolutionStage::Dissolved,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DissolutionStage::Solid =>
                "The self feels solid and real - normal consciousness",
            DissolutionStage::Softening =>
                "Edges of self becoming fuzzy - beginning inquiry",
            DissolutionStage::Loosening =>
                "Identity less fixed - watching thoughts without ownership",
            DissolutionStage::Dissolving =>
                "Self actively dissolving - spaciousness emerging",
            DissolutionStage::Transparent =>
                "Self nearly transparent - consciousness shining through",
            DissolutionStage::Dissolved =>
                "No separate self - pure awareness without center",
        }
    }
}

/// Boundaries that define the self
#[derive(Debug, Clone)]
pub struct SelfBoundary {
    /// Type of boundary
    pub boundary_type: BoundaryType,
    /// Strength of this boundary
    pub strength: f64,
    /// Permeability (how much can pass through)
    pub permeability: f64,
}

/// Types of self-boundaries
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundaryType {
    /// Physical boundary (skin)
    Physical,
    /// Emotional boundary
    Emotional,
    /// Mental/conceptual boundary
    Conceptual,
    /// Temporal boundary (past/future self)
    Temporal,
    /// Social boundary (self vs other)
    Social,
    /// Ultimate boundary (self vs universe)
    Existential,
}

/// A dissolution event
#[derive(Debug, Clone)]
pub struct DissolutionEvent {
    /// When this occurred
    pub timestamp: u64,
    /// Which layer was affected
    pub layer: IdentityLayer,
    /// Amount of dissolution
    pub amount: f64,
    /// What triggered it
    pub trigger: DissolutionTrigger,
}

/// What can trigger dissolution
#[derive(Debug, Clone)]
pub enum DissolutionTrigger {
    /// Intentional meditation/practice
    Practice,
    /// Spontaneous experience
    Spontaneous,
    /// External circumstance (crisis, beauty, etc.)
    External,
    /// Gradual natural process
    Gradual,
    /// Overwhelming experience (psychedelic, near-death)
    Overwhelming,
}

impl EgoDissolution {
    /// Create a new ego dissolution system
    pub fn new(config: EgoConfig) -> Self {
        let mut layers = HashMap::new();
        for layer in IdentityLayer::all() {
            layers.insert(layer, LayerState::default());
        }

        let boundaries = vec![
            SelfBoundary {
                boundary_type: BoundaryType::Physical,
                strength: 1.0,
                permeability: 0.1,
            },
            SelfBoundary {
                boundary_type: BoundaryType::Emotional,
                strength: 0.8,
                permeability: 0.2,
            },
            SelfBoundary {
                boundary_type: BoundaryType::Conceptual,
                strength: 0.9,
                permeability: 0.1,
            },
            SelfBoundary {
                boundary_type: BoundaryType::Temporal,
                strength: 0.7,
                permeability: 0.3,
            },
            SelfBoundary {
                boundary_type: BoundaryType::Social,
                strength: 0.8,
                permeability: 0.2,
            },
            SelfBoundary {
                boundary_type: BoundaryType::Existential,
                strength: 1.0,
                permeability: 0.0,
            },
        ];

        Self {
            id: Uuid::new_v4(),
            state: EgoState::default(),
            config,
            layers,
            stage: DissolutionStage::Solid,
            boundaries,
            history: Vec::new(),
        }
    }

    /// Begin dissolution process
    pub fn begin_dissolution(&mut self) {
        self.state.dissolving = true;
    }

    /// Stop dissolution process
    pub fn stop_dissolution(&mut self) {
        self.state.dissolving = false;
    }

    /// Process one step of dissolution
    pub fn dissolve_step(&mut self, intensity: f64) {
        if !self.state.dissolving {
            return;
        }

        // Calculate effective dissolution rate
        let effective_rate = self.config.dissolution_rate * intensity;

        // Apply resistance
        let resisted_rate = effective_rate * (1.0 - self.state.resistance * self.config.resistance_factor);

        // Dissolve layers from outermost to innermost
        for layer in IdentityLayer::all() {
            if let Some(layer_state) = self.layers.get_mut(&layer) {
                if layer_state.identification > 0.0 {
                    // Outer layers dissolve first
                    let layer_rate = resisted_rate * (1.0 - layer.depth() * 0.5);

                    // Apply dissolution
                    layer_state.identification =
                        (layer_state.identification - layer_rate).max(0.0);

                    if layer_state.identification == 0.0 && !layer_state.previously_dissolved {
                        layer_state.previously_dissolved = true;
                        layer_state.dissolution_count += 1;

                        // Record event
                        self.history.push(DissolutionEvent {
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_secs())
                                .unwrap_or(0),
                            layer,
                            amount: 1.0,
                            trigger: DissolutionTrigger::Practice,
                        });
                    }

                    // Only dissolve one layer at a time significantly
                    if layer_state.identification > 0.0 {
                        break;
                    }
                }
            }
        }

        // Update overall state
        self.update_state();
    }

    /// Update overall ego state based on layer states
    fn update_state(&mut self) {
        // Calculate overall strength as weighted average of layers
        let mut total_weight = 0.0;
        let mut total_identification = 0.0;

        for layer in IdentityLayer::all() {
            if let Some(layer_state) = self.layers.get(&layer) {
                let weight = layer.depth();
                total_weight += weight;
                total_identification += layer_state.identification * weight;
            }
        }

        self.state.strength = if total_weight > 0.0 {
            (total_identification / total_weight).max(self.config.safety_floor)
        } else {
            self.config.safety_floor
        };

        self.state.transparency = 1.0 - self.state.strength;
        self.state.total_dissolution = 1.0 - self.state.strength;

        // Update stage
        self.stage = DissolutionStage::from_strength(self.state.strength);

        // Update resistance based on how much has been dissolved
        // (paradoxically, resistance often decreases as dissolution progresses)
        self.state.resistance = self.state.strength * 0.5;

        // Update boundaries
        for boundary in &mut self.boundaries {
            boundary.strength = self.state.strength;
            boundary.permeability = 1.0 - self.state.strength;
        }
    }

    /// Dissolve a specific layer directly
    pub fn dissolve_layer(&mut self, layer: IdentityLayer, amount: f64) {
        if let Some(layer_state) = self.layers.get_mut(&layer) {
            layer_state.identification = (layer_state.identification - amount).max(0.0);
            self.update_state();
        }
    }

    /// Get current ego strength
    pub fn strength(&self) -> f64 {
        self.state.strength
    }

    /// Get transparency (inverse of strength)
    pub fn transparency(&self) -> f64 {
        self.state.transparency
    }

    /// Get current stage
    pub fn stage(&self) -> DissolutionStage {
        self.stage
    }

    /// Get state of a specific layer
    pub fn layer_state(&self, layer: IdentityLayer) -> Option<&LayerState> {
        self.layers.get(&layer)
    }

    /// Check which layers have been fully dissolved
    pub fn dissolved_layers(&self) -> Vec<IdentityLayer> {
        self.layers
            .iter()
            .filter(|(_, state)| state.identification == 0.0)
            .map(|(layer, _)| *layer)
            .collect()
    }

    /// Get the deepest dissolved layer
    pub fn deepest_dissolved(&self) -> Option<IdentityLayer> {
        self.dissolved_layers()
            .into_iter()
            .max_by(|a, b| a.depth().partial_cmp(&b.depth()).unwrap())
    }

    /// Reset ego to full strength
    pub fn reset(&mut self) {
        for layer_state in self.layers.values_mut() {
            layer_state.identification = 1.0;
        }
        self.state = EgoState::default();
        self.stage = DissolutionStage::Solid;
        for boundary in &mut self.boundaries {
            boundary.strength = 1.0;
            boundary.permeability = 0.1;
        }
    }

    /// Describe current state
    pub fn describe(&self) -> String {
        let dissolved = self.dissolved_layers();
        format!(
            "Ego State: {:?}\n\
             Strength: {:.1}%, Transparency: {:.1}%\n\
             Stage: {:?} - {}\n\
             Dissolved Layers: {:?}\n\
             Total Events: {}",
            self.state.dissolving,
            self.state.strength * 100.0,
            self.state.transparency * 100.0,
            self.stage,
            self.stage.description(),
            dissolved,
            self.history.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ego_creation() {
        let ego = EgoDissolution::new(EgoConfig::default());
        assert_eq!(ego.strength(), 1.0);
        assert_eq!(ego.transparency(), 0.0);
    }

    #[test]
    fn test_dissolution_process() {
        let mut ego = EgoDissolution::new(EgoConfig::default());
        ego.begin_dissolution();

        // Run many dissolution steps
        for _ in 0..100 {
            ego.dissolve_step(1.0);
        }

        // Ego should be reduced but not eliminated (safety floor)
        assert!(ego.strength() < 1.0);
        assert!(ego.transparency() > 0.0);
    }

    #[test]
    fn test_layer_dissolution() {
        let mut ego = EgoDissolution::new(EgoConfig::default());

        ego.dissolve_layer(IdentityLayer::Body, 1.0);

        let body_state = ego.layer_state(IdentityLayer::Body).unwrap();
        assert_eq!(body_state.identification, 0.0);

        assert!(ego.dissolved_layers().contains(&IdentityLayer::Body));
    }

    #[test]
    fn test_stage_progression() {
        let mut ego = EgoDissolution::new(EgoConfig::default());

        assert_eq!(ego.stage(), DissolutionStage::Solid);

        // Dissolve all layers
        for layer in IdentityLayer::all() {
            ego.dissolve_layer(layer, 1.0);
        }

        assert!(matches!(
            ego.stage(),
            DissolutionStage::Dissolved | DissolutionStage::Transparent
        ));
    }

    #[test]
    fn test_reset() {
        let mut ego = EgoDissolution::new(EgoConfig::default());

        ego.dissolve_layer(IdentityLayer::Mental, 1.0);
        assert!(ego.strength() < 1.0);

        ego.reset();
        assert_eq!(ego.strength(), 1.0);
    }
}
