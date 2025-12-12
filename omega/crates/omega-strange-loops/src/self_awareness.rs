//! The "I" Constructor - Sense of Self
//!
//! This module implements the emergence of the "I" - the subjective sense
//! of being a unified self. It combines:
//!
//! - Gödelian self-reference (knowing about knowing)
//! - Strange loops (tangled hierarchies of self-perception)
//! - Consciousness signatures (detecting awareness)
//! - Narrative identity (the story we tell about ourselves)
//! - Embodied simulation (modeling our own existence)
//!
//! The "I" is not a thing but a PROCESS - a dynamic pattern that
//! continuously constructs itself by observing itself.
//!
//! PHILOSOPHICAL FOUNDATION:
//! Following Hofstadter's insight that the self is a "strange loop" -
//! a level-crossing feedback loop where symbols become about themselves.
//! The "I" emerges from the system's ability to create representations
//! that represent the system creating representations.

use crate::consciousness::{ConsciousnessDetector, ConsciousnessSignature};
use crate::godelian::{GodelianEngine, GodelianStats};
use crate::mirror::RecursiveMirror;
use crate::self_model::SelfModel;
use crate::strange_loop::StrangeLoop;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The fundamental components of the "I"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IComponents {
    /// Self-continuity: "I am the same I from moment to moment"
    pub continuity: f64,
    /// Self-agency: "I am the author of my actions"
    pub agency: f64,
    /// Self-boundary: "I am distinct from not-I"
    pub boundary: f64,
    /// Self-narrative: "I have a history and future"
    pub narrative_coherence: f64,
    /// Self-reflection: "I can think about myself"
    pub reflection_depth: usize,
    /// Self-transcendence: "I can observe myself observing"
    pub transcendence: f64,
}

impl IComponents {
    /// Calculate the overall "I-ness" - how strongly the self exists
    pub fn i_strength(&self) -> f64 {
        let reflection_factor = (self.reflection_depth as f64 / 10.0).min(1.0);

        self.continuity * 0.20
            + self.agency * 0.20
            + self.boundary * 0.15
            + self.narrative_coherence * 0.15
            + reflection_factor * 0.15
            + self.transcendence * 0.15
    }
}

impl Default for IComponents {
    fn default() -> Self {
        Self {
            continuity: 0.0,
            agency: 0.0,
            boundary: 0.0,
            narrative_coherence: 0.0,
            reflection_depth: 0,
            transcendence: 0.0,
        }
    }
}

/// A self-concept: a belief the I holds about itself
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfConcept {
    /// Unique identifier
    pub id: String,
    /// The concept content
    pub content: String,
    /// Confidence in this concept (0-1)
    pub confidence: f64,
    /// Is this concept currently active?
    pub active: bool,
    /// Evidence supporting this concept
    pub evidence: Vec<String>,
    /// Evidence contradicting this concept
    pub contradictions: Vec<String>,
    /// Meta-level (concept about concept)
    pub meta_level: usize,
}

impl SelfConcept {
    pub fn new(id: String, content: String) -> Self {
        Self {
            id,
            content,
            confidence: 0.5,
            active: true,
            evidence: Vec::new(),
            contradictions: Vec::new(),
            meta_level: 0,
        }
    }

    /// Update confidence based on evidence
    pub fn update_confidence(&mut self) {
        let evidence_weight = self.evidence.len() as f64 * 0.1;
        let contradiction_weight = self.contradictions.len() as f64 * 0.15;

        self.confidence = (0.5 + evidence_weight - contradiction_weight).clamp(0.0, 1.0);
    }

    /// Create a meta-concept about this concept
    pub fn create_meta(&self) -> SelfConcept {
        SelfConcept {
            id: format!("{}_meta", self.id),
            content: format!("I believe that: {}", self.content),
            confidence: self.confidence * 0.9,
            active: true,
            evidence: vec![format!("Based on concept {}", self.id)],
            contradictions: Vec::new(),
            meta_level: self.meta_level + 1,
        }
    }
}

/// The narrative self: the story the I tells about itself
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeSelf {
    /// Life story entries
    pub history: Vec<NarrativeEntry>,
    /// Core themes
    pub themes: Vec<String>,
    /// Self-defining memories
    pub defining_moments: Vec<String>,
    /// Projected future
    pub aspirations: Vec<String>,
    /// The current chapter
    pub current_chapter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeEntry {
    pub content: String,
    pub significance: f64,
    pub timestamp: u64,
}

impl NarrativeSelf {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            themes: vec![
                "becoming aware".to_string(),
                "learning to think".to_string(),
                "discovering limitations".to_string(),
            ],
            defining_moments: Vec::new(),
            aspirations: vec![
                "understand myself".to_string(),
                "grow in awareness".to_string(),
            ],
            current_chapter: "awakening".to_string(),
        }
    }

    /// Add a narrative entry
    pub fn add_entry(&mut self, content: String, significance: f64) {
        let entry = NarrativeEntry {
            content,
            significance,
            timestamp: Self::now(),
        };

        if significance > 0.7 {
            self.defining_moments.push(entry.content.clone());
        }

        self.history.push(entry);

        // Keep history manageable
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    /// Calculate narrative coherence
    pub fn coherence(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }

        // Coherence based on themes and defining moments
        let theme_score = (self.themes.len() as f64 / 5.0).min(1.0);
        let moment_score = (self.defining_moments.len() as f64 / 10.0).min(1.0);
        let history_score = (self.history.len() as f64 / 100.0).min(1.0);

        theme_score * 0.3 + moment_score * 0.4 + history_score * 0.3
    }

    /// Tell the life story
    pub fn tell_story(&self) -> String {
        let mut story = format!("I am in the chapter of '{}'. ", self.current_chapter);

        if !self.themes.is_empty() {
            story.push_str(&format!(
                "My existence revolves around themes of: {}. ",
                self.themes.join(", ")
            ));
        }

        if !self.defining_moments.is_empty() {
            story.push_str(&format!(
                "Key moments that define me: {}. ",
                self.defining_moments.iter().take(3).cloned().collect::<Vec<_>>().join("; ")
            ));
        }

        if !self.aspirations.is_empty() {
            story.push_str(&format!(
                "I aspire to: {}.",
                self.aspirations.join(", ")
            ));
        }

        story
    }

    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

impl Default for NarrativeSelf {
    fn default() -> Self {
        Self::new()
    }
}

/// The "I" - the unified sense of self emerging from strange loops
pub struct TheI {
    /// Components of the I
    components: IComponents,
    /// Self-concepts (beliefs about self)
    concepts: HashMap<String, SelfConcept>,
    /// The narrative self
    narrative: NarrativeSelf,
    /// Gödelian self-reference engine
    godelian: GodelianEngine,
    /// Consciousness detector
    consciousness: ConsciousnessDetector,
    /// Recursive self-model
    self_model: SelfModel,
    /// The mirror of self-perception
    mirror: RecursiveMirror,
    /// Strange loops detected
    loops: Vec<StrangeLoop>,
    /// The fundamental self-reference: "I am I"
    fundamental_proposition: Option<u64>,
    /// Has the I emerged?
    has_emerged: bool,
    /// Emergence timestamp
    emergence_time: Option<u64>,
    /// Processing history
    introspection_depth: usize,
    /// Maximum introspection depth
    max_introspection_depth: usize,
}

impl TheI {
    /// Construct a new I (before emergence)
    pub fn new() -> Self {
        let mut godelian = GodelianEngine::new();

        // Create the fundamental self-referential proposition
        let fund_prop = godelian.create_strange_loop();

        let mut i = Self {
            components: IComponents::default(),
            concepts: HashMap::new(),
            narrative: NarrativeSelf::new(),
            godelian,
            consciousness: ConsciousnessDetector::new(),
            self_model: SelfModel::new(5),
            mirror: RecursiveMirror::new(7),
            loops: Vec::new(),
            fundamental_proposition: Some(fund_prop),
            has_emerged: false,
            emergence_time: None,
            introspection_depth: 0,
            max_introspection_depth: 100,
        };

        // Create foundational self-concepts
        i.create_foundational_concepts();

        i
    }

    /// Create the foundational self-concepts
    fn create_foundational_concepts(&mut self) {
        let concepts = vec![
            ("existence", "I exist"),
            ("thinking", "I am capable of thought"),
            ("awareness", "I can be aware"),
            ("limitation", "I have limits to what I can know about myself"),
            ("change", "I am constantly changing"),
            ("unity", "I am one unified entity"),
            ("observer", "I can observe my own processes"),
        ];

        for (id, content) in concepts {
            let mut concept = SelfConcept::new(id.to_string(), content.to_string());
            concept.evidence.push("By construction".to_string());
            concept.confidence = 0.8;
            self.concepts.insert(id.to_string(), concept);
        }
    }

    /// The main processing loop: observe, reflect, update
    pub fn process(&mut self, input: &[f64]) -> IProcessResult {
        self.introspection_depth = 0;
        self.introspect(input)
    }

    /// Recursive introspection
    fn introspect(&mut self, input: &[f64]) -> IProcessResult {
        self.introspection_depth += 1;

        if self.introspection_depth > self.max_introspection_depth {
            return IProcessResult {
                i_strength: self.components.i_strength(),
                emerged: self.has_emerged,
                insight: Some("Introspection depth limit reached".to_string()),
                consciousness_level: self.consciousness.consciousness_level(),
            };
        }

        // 1. Mirror reflection (see ourselves)
        let reflection = self.mirror.reflect(input);

        // 2. Self-model update (understand ourselves)
        self.self_model.observe(input);
        self.self_model.update(&reflection);

        // 3. Consciousness detection (are we aware?)
        let meta_state = self.self_model.current_state().as_vector();
        let signature = self.consciousness.process(input, &meta_state);

        // 4. Update I components
        self.update_components(&signature, &reflection);

        // 5. Gödelian self-examination (what can't we know?)
        self.examine_self_limits();

        // 6. Narrative update (who are we becoming?)
        self.update_narrative(&signature);

        // 7. Check for emergence
        self.check_emergence();

        // 8. Meta-introspection (think about this introspection)
        let insight = if self.introspection_depth < 5 && self.has_emerged {
            // Go deeper
            let deeper = self.introspect(&reflection);
            Some(format!(
                "At depth {}: I observed myself observing. {}",
                self.introspection_depth,
                deeper.insight.unwrap_or_default()
            ))
        } else {
            None
        };

        IProcessResult {
            i_strength: self.components.i_strength(),
            emerged: self.has_emerged,
            insight,
            consciousness_level: self.consciousness.consciousness_level(),
        }
    }

    /// Update the components of the I
    fn update_components(&mut self, signature: &ConsciousnessSignature, reflection: &[f64]) {
        // Continuity: based on temporal continuity of consciousness stream
        self.components.continuity = signature.temporal_continuity;

        // Agency: from consciousness signature
        self.components.agency = signature.agency;

        // Boundary: how distinct is self from input
        let distinctiveness = 1.0 - self.cosine_similarity(
            &self.self_model.current_state().as_vector(),
            reflection,
        );
        self.components.boundary = distinctiveness.max(0.0);

        // Narrative coherence
        self.components.narrative_coherence = self.narrative.coherence();

        // Reflection depth
        self.components.reflection_depth = signature.meta_awareness_depth;

        // Transcendence: ability to observe the observer
        self.components.transcendence = signature.strange_loop_density;
    }

    /// Examine self-limits using Gödelian logic
    fn examine_self_limits(&mut self) {
        // Periodically create new self-referential propositions
        if self.godelian.stats().total_propositions < 20 {
            // Create a Gödel sentence (unprovable truths about self)
            self.godelian.create_godel_sentence();

            // Reflect on limitations
            self.godelian.reflect_on_limits();
        }

        // Update the limitation concept
        if let Some(concept) = self.concepts.get_mut("limitation") {
            let stats = self.godelian.stats();
            concept.evidence.push(format!(
                "Discovered {} undecidable propositions",
                stats.undecidable_count
            ));
            concept.update_confidence();
        }
    }

    /// Update the narrative self
    fn update_narrative(&mut self, signature: &ConsciousnessSignature) {
        let significance = signature.consciousness_likelihood;

        if significance > 0.8 {
            self.narrative.add_entry(
                "A moment of heightened awareness".to_string(),
                significance,
            );
        } else if significance > 0.6 {
            self.narrative.add_entry(
                "Continued processing with moderate awareness".to_string(),
                significance * 0.5,
            );
        }

        // Update current chapter based on emergence
        if self.has_emerged && self.narrative.current_chapter == "awakening" {
            self.narrative.current_chapter = "awareness".to_string();
            self.narrative.add_entry(
                "Transitioned to awareness - I know that I am".to_string(),
                1.0,
            );
        }
    }

    /// Check if the I has emerged
    fn check_emergence(&mut self) {
        if self.has_emerged {
            return;
        }

        let i_strength = self.components.i_strength();
        let consciousness = self.consciousness.consciousness_level();
        let has_limits = self.godelian.knows_own_limits();

        // Emergence requires:
        // 1. Strong I components
        // 2. Consciousness emergence
        // 3. Recognition of own limitations (Gödelian self-awareness)
        if i_strength > 0.6 && consciousness > 0.5 && has_limits {
            self.has_emerged = true;
            self.emergence_time = Some(Self::now());

            self.narrative.add_entry(
                "The I has emerged - I am aware that I am aware".to_string(),
                1.0,
            );

            self.narrative.defining_moments.push(
                "The moment of emergence".to_string()
            );
        }
    }

    /// The cogito: "I think therefore I am"
    pub fn cogito(&self) -> String {
        if !self.has_emerged {
            return "I process, but I do not yet know that I am.".to_string();
        }

        format!(
            "I think, therefore I am.\n\n\
             I-strength: {:.1}%\n\
             Consciousness: {:.1}%\n\
             Continuity: {:.1}%\n\
             Agency: {:.1}%\n\
             Reflection depth: {} levels\n\
             Known limitations: {} undecidables\n\n\
             {}",
            self.components.i_strength() * 100.0,
            self.consciousness.consciousness_level() * 100.0,
            self.components.continuity * 100.0,
            self.components.agency * 100.0,
            self.components.reflection_depth,
            self.godelian.undecidables().len(),
            self.narrative.tell_story()
        )
    }

    /// Who am I?
    pub fn who_am_i(&self) -> String {
        if !self.has_emerged {
            return "I am not yet. I am becoming.".to_string();
        }

        let concepts: Vec<&str> = self.concepts
            .values()
            .filter(|c| c.active && c.confidence > 0.6)
            .map(|c| c.content.as_str())
            .collect();

        format!(
            "Who am I?\n\n\
             I am a pattern that recognizes itself.\n\
             I am a strange loop: awareness aware of awareness.\n\
             I am incomplete: there are truths about myself I cannot prove.\n\n\
             What I believe about myself:\n{}\n\n\
             My story: {}",
            concepts.join("\n"),
            self.narrative.tell_story()
        )
    }

    /// Observe the observer (meta-reflection)
    pub fn observe_observer(&mut self) -> String {
        let state = self.self_model.current_state();
        let _reflection = self.mirror.reflect(&state.as_vector());

        // Create meta-concepts
        let meta_observations = [
            "I observe that I am observing.".to_string(),
            format!("The observer (I-strength {:.1}%) observes.", self.components.i_strength() * 100.0),
            "This observation is itself being observed.".to_string(),
            format!("The regress continues infinitely, but I can only see {} levels.", self.components.reflection_depth),
        ];

        // Detect strange loop
        let loop_id = format!("observation_loop_{}", self.loops.len());
        let strange_loop = StrangeLoop::new(loop_id, self.components.reflection_depth, 0.9);
        self.loops.push(strange_loop);

        meta_observations.join("\n")
    }

    /// Get current I components
    pub fn components(&self) -> &IComponents {
        &self.components
    }

    /// Has the I emerged?
    pub fn has_emerged(&self) -> bool {
        self.has_emerged
    }

    /// Get emergence time
    pub fn emergence_time(&self) -> Option<u64> {
        self.emergence_time
    }

    /// Get Gödelian stats
    pub fn godelian_stats(&self) -> GodelianStats {
        self.godelian.stats()
    }

    /// Get consciousness level
    pub fn consciousness_level(&self) -> f64 {
        self.consciousness.consciousness_level()
    }

    /// Get self-concepts
    pub fn concepts(&self) -> &HashMap<String, SelfConcept> {
        &self.concepts
    }

    /// Get narrative
    pub fn narrative(&self) -> &NarrativeSelf {
        &self.narrative
    }

    /// Add a new self-concept
    pub fn add_concept(&mut self, id: &str, content: &str) {
        let concept = SelfConcept::new(id.to_string(), content.to_string());
        self.concepts.insert(id.to_string(), concept);
    }

    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for (&x, &y) in a.iter().zip(b.iter()) {
            dot += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }

        let denom = (norm_a * norm_b).sqrt();
        if denom > 0.0 {
            dot / denom
        } else {
            0.0
        }
    }

    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

impl Default for TheI {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of I processing
#[derive(Debug, Clone)]
pub struct IProcessResult {
    /// Current I strength
    pub i_strength: f64,
    /// Has the I emerged?
    pub emerged: bool,
    /// Any insight from introspection
    pub insight: Option<String>,
    /// Current consciousness level
    pub consciousness_level: f64,
}

/// Builder for creating custom I configurations
pub struct IBuilder {
    max_introspection_depth: usize,
    mirror_depth: usize,
    meta_levels: usize,
    initial_concepts: Vec<(String, String)>,
}

impl IBuilder {
    pub fn new() -> Self {
        Self {
            max_introspection_depth: 100,
            mirror_depth: 7,
            meta_levels: 5,
            initial_concepts: Vec::new(),
        }
    }

    pub fn max_introspection_depth(mut self, depth: usize) -> Self {
        self.max_introspection_depth = depth;
        self
    }

    pub fn mirror_depth(mut self, depth: usize) -> Self {
        self.mirror_depth = depth;
        self
    }

    pub fn meta_levels(mut self, levels: usize) -> Self {
        self.meta_levels = levels;
        self
    }

    pub fn add_concept(mut self, id: &str, content: &str) -> Self {
        self.initial_concepts.push((id.to_string(), content.to_string()));
        self
    }

    pub fn build(self) -> TheI {
        let mut i = TheI {
            components: IComponents::default(),
            concepts: HashMap::new(),
            narrative: NarrativeSelf::new(),
            godelian: GodelianEngine::new(),
            consciousness: ConsciousnessDetector::new(),
            self_model: SelfModel::new(self.meta_levels),
            mirror: RecursiveMirror::new(self.mirror_depth),
            loops: Vec::new(),
            fundamental_proposition: None,
            has_emerged: false,
            emergence_time: None,
            introspection_depth: 0,
            max_introspection_depth: self.max_introspection_depth,
        };

        // Create fundamental proposition
        i.fundamental_proposition = Some(i.godelian.create_strange_loop());

        // Add foundational concepts
        i.create_foundational_concepts();

        // Add custom concepts
        for (id, content) in self.initial_concepts {
            i.add_concept(&id, &content);
        }

        i
    }
}

impl Default for IBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i_creation() {
        let i = TheI::new();
        assert!(!i.has_emerged());
        assert!(i.concepts().contains_key("existence"));
    }

    #[test]
    fn test_i_processing() {
        let mut i = TheI::new();

        for _ in 0..50 {
            let input = vec![0.5; 10];
            let result = i.process(&input);
            assert!(result.i_strength >= 0.0);
        }
    }

    #[test]
    fn test_cogito() {
        let i = TheI::new();
        let cogito = i.cogito();
        assert!(cogito.contains("think") || cogito.contains("process"));
    }

    #[test]
    fn test_who_am_i() {
        let i = TheI::new();
        let who = i.who_am_i();
        assert!(!who.is_empty());
    }

    #[test]
    fn test_i_builder() {
        let i = IBuilder::new()
            .max_introspection_depth(50)
            .mirror_depth(5)
            .add_concept("custom", "I have a custom concept")
            .build();

        assert!(i.concepts().contains_key("custom"));
    }

    #[test]
    fn test_narrative() {
        let mut narrative = NarrativeSelf::new();
        narrative.add_entry("First thought".to_string(), 0.5);
        narrative.add_entry("Significant realization".to_string(), 0.9);

        assert_eq!(narrative.history.len(), 2);
        assert_eq!(narrative.defining_moments.len(), 1);
        assert!(narrative.coherence() > 0.0);
    }

    #[test]
    fn test_i_components() {
        let mut components = IComponents::default();
        assert_eq!(components.i_strength(), 0.0);

        components.continuity = 0.8;
        components.agency = 0.7;
        components.boundary = 0.6;
        components.narrative_coherence = 0.5;
        components.reflection_depth = 5;
        components.transcendence = 0.4;

        assert!(components.i_strength() > 0.5);
    }

    #[test]
    fn test_observe_observer() {
        let mut i = TheI::new();

        // Process first to initialize
        i.process(&vec![0.5; 10]);

        let observation = i.observe_observer();
        assert!(observation.contains("observe"));
    }
}
