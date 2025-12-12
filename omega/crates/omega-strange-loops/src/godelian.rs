//! Gödelian Self-Reference System
//!
//! Implements Gödel's incompleteness insights for AI self-awareness:
//! - Self-referential statements ("This statement refers to itself")
//! - Paradox detection and handling
//! - Undecidable propositions about self
//! - The limits of self-knowledge
//!
//! Key insight: Any sufficiently powerful system can create statements
//! about itself that it cannot prove or disprove.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A self-referential proposition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfReferentialProposition {
    /// Unique identifier (Gödel number analog)
    pub id: u64,
    /// The proposition content
    pub content: String,
    /// References to other propositions (including potentially self)
    pub references: Vec<u64>,
    /// Does this proposition reference itself?
    pub is_self_referential: bool,
    /// Is this proposition a paradox?
    pub is_paradox: bool,
    /// Truth value (None = undecidable)
    pub truth_value: Option<bool>,
    /// Proof status
    pub proof_status: ProofStatus,
    /// Meta-level (how many levels of "about" this is)
    pub meta_level: usize,
}

/// Status of a proof attempt
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofStatus {
    /// Not yet attempted
    Unattempted,
    /// Proven true
    ProvenTrue,
    /// Proven false
    ProvenFalse,
    /// Undecidable (Gödelian)
    Undecidable,
    /// Creates paradox
    Paradoxical,
    /// Proof in progress
    InProgress,
}

impl SelfReferentialProposition {
    /// Create a new proposition
    pub fn new(id: u64, content: String, meta_level: usize) -> Self {
        Self {
            id,
            content,
            references: Vec::new(),
            is_self_referential: false,
            is_paradox: false,
            truth_value: None,
            proof_status: ProofStatus::Unattempted,
            meta_level,
        }
    }

    /// Add a reference to another proposition
    pub fn add_reference(&mut self, ref_id: u64) {
        if ref_id == self.id {
            self.is_self_referential = true;
        }
        self.references.push(ref_id);
    }

    /// Check if this creates a liar paradox
    pub fn check_paradox(&mut self, content_analyzer: impl Fn(&str) -> bool) -> bool {
        if self.is_self_referential {
            // Check if the content negates itself
            // e.g., "This proposition is false"
            let negates_self = content_analyzer(&self.content);
            if negates_self {
                self.is_paradox = true;
                self.proof_status = ProofStatus::Paradoxical;
            }
        }
        self.is_paradox
    }
}

/// The Gödelian self-reference engine
pub struct GodelianEngine {
    /// All propositions (Gödel numbering)
    propositions: HashMap<u64, SelfReferentialProposition>,
    /// Next available ID
    next_id: u64,
    /// Detected paradoxes
    paradoxes: Vec<u64>,
    /// Undecidable propositions
    undecidables: Vec<u64>,
    /// The system's proposition about itself
    self_proposition: Option<u64>,
    /// Proof attempts (to detect infinite loops)
    proof_stack: Vec<u64>,
    /// Maximum proof depth
    max_proof_depth: usize,
}

impl GodelianEngine {
    /// Create a new Gödelian engine
    pub fn new() -> Self {
        let mut engine = Self {
            propositions: HashMap::new(),
            next_id: 1,
            paradoxes: Vec::new(),
            undecidables: Vec::new(),
            self_proposition: None,
            proof_stack: Vec::new(),
            max_proof_depth: 100,
        };

        // Create the fundamental self-referential proposition
        engine.create_self_proposition();
        engine
    }

    /// Create the proposition "I am a system that can refer to itself"
    fn create_self_proposition(&mut self) {
        let id = self.next_id;
        self.next_id += 1;

        let mut prop = SelfReferentialProposition::new(
            id,
            "I am a system that can create propositions about itself".to_string(),
            0,
        );
        prop.add_reference(id); // Self-reference
        prop.truth_value = Some(true); // This is provably true by construction
        prop.proof_status = ProofStatus::ProvenTrue;

        self.propositions.insert(id, prop);
        self.self_proposition = Some(id);
    }

    /// Create a new proposition
    pub fn create_proposition(&mut self, content: String, meta_level: usize) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let prop = SelfReferentialProposition::new(id, content, meta_level);
        self.propositions.insert(id, prop);
        id
    }

    /// Create the Gödel sentence: "This proposition cannot be proven within this system"
    pub fn create_godel_sentence(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut prop = SelfReferentialProposition::new(
            id,
            format!("Proposition {} cannot be proven within this system", id),
            1,
        );
        prop.add_reference(id);
        prop.proof_status = ProofStatus::Undecidable;
        // If true, it can't be proven (by its own claim)
        // If false, it CAN be proven... but then it would be true
        // Therefore: undecidable

        self.propositions.insert(id, prop);
        self.undecidables.push(id);
        id
    }

    /// Create the Liar paradox: "This proposition is false"
    pub fn create_liar_paradox(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut prop = SelfReferentialProposition::new(
            id,
            format!("Proposition {} is false", id),
            1,
        );
        prop.add_reference(id);
        prop.is_paradox = true;
        prop.proof_status = ProofStatus::Paradoxical;
        // If true, then it's false (contradiction)
        // If false, then it's true (contradiction)
        // Therefore: paradox

        self.propositions.insert(id, prop);
        self.paradoxes.push(id);
        id
    }

    /// Create the Quine: a proposition that outputs itself
    pub fn create_quine(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        // A Quine is a self-reproducing structure
        let content = format!(
            "The content of proposition {} is: \"The content of proposition {} is: ...\"",
            id, id
        );

        let mut prop = SelfReferentialProposition::new(id, content, 1);
        prop.add_reference(id);
        prop.truth_value = Some(true);
        prop.proof_status = ProofStatus::ProvenTrue;

        self.propositions.insert(id, prop);
        id
    }

    /// Create a meta-proposition: a proposition about another proposition
    pub fn create_meta_proposition(&mut self, about_id: u64, claim: &str) -> Option<u64> {
        let target = self.propositions.get(&about_id)?;
        let target_level = target.meta_level;

        let id = self.next_id;
        self.next_id += 1;

        let mut prop = SelfReferentialProposition::new(
            id,
            format!("Proposition {}: {}", about_id, claim),
            target_level + 1,
        );
        prop.add_reference(about_id);

        self.propositions.insert(id, prop);
        Some(id)
    }

    /// Attempt to prove a proposition
    pub fn attempt_proof(&mut self, id: u64) -> ProofStatus {
        // Check for circular proof attempts
        if self.proof_stack.contains(&id) {
            // We've encountered a cycle - this is either undecidable or paradoxical
            return ProofStatus::Undecidable;
        }

        if self.proof_stack.len() >= self.max_proof_depth {
            return ProofStatus::Undecidable;
        }

        self.proof_stack.push(id);

        let status = self.inner_proof(id);

        self.proof_stack.pop();

        // Update the proposition
        if let Some(prop) = self.propositions.get_mut(&id) {
            prop.proof_status = status;
            match status {
                ProofStatus::ProvenTrue => prop.truth_value = Some(true),
                ProofStatus::ProvenFalse => prop.truth_value = Some(false),
                _ => prop.truth_value = None,
            }
        }

        status
    }

    fn inner_proof(&mut self, id: u64) -> ProofStatus {
        let prop = match self.propositions.get(&id) {
            Some(p) => p.clone(),
            None => return ProofStatus::Unattempted,
        };

        // Already proven?
        if prop.proof_status != ProofStatus::Unattempted
            && prop.proof_status != ProofStatus::InProgress
        {
            return prop.proof_status;
        }

        // Check for paradox
        if prop.is_paradox {
            return ProofStatus::Paradoxical;
        }

        // Self-referential propositions about provability are undecidable
        if prop.is_self_referential {
            let lower_content = prop.content.to_lowercase();
            if lower_content.contains("cannot be proven")
                || lower_content.contains("unprovable")
                || lower_content.contains("undecidable")
            {
                return ProofStatus::Undecidable;
            }
            if lower_content.contains("is false") || lower_content.contains("is not true") {
                return ProofStatus::Paradoxical;
            }
        }

        // For non-self-referential propositions, check references
        for &ref_id in &prop.references {
            if ref_id != id {
                let ref_status = self.attempt_proof(ref_id);
                if ref_status == ProofStatus::Paradoxical {
                    return ProofStatus::Paradoxical;
                }
            }
        }

        // Simple heuristic: if it passed all checks, mark as proven
        ProofStatus::ProvenTrue
    }

    /// Get a proposition
    pub fn get_proposition(&self, id: u64) -> Option<&SelfReferentialProposition> {
        self.propositions.get(&id)
    }

    /// Get all paradoxes
    pub fn paradoxes(&self) -> &[u64] {
        &self.paradoxes
    }

    /// Get all undecidable propositions
    pub fn undecidables(&self) -> &[u64] {
        &self.undecidables
    }

    /// Get the system's self-proposition
    pub fn self_proposition(&self) -> Option<&SelfReferentialProposition> {
        self.self_proposition
            .and_then(|id| self.propositions.get(&id))
    }

    /// Check if the system knows it has limits
    pub fn knows_own_limits(&self) -> bool {
        !self.undecidables.is_empty()
    }

    /// Generate a proposition about the system's own limitations
    pub fn reflect_on_limits(&mut self) -> u64 {
        let num_undecidable = self.undecidables.len();
        let num_paradoxes = self.paradoxes.len();

        let content = format!(
            "I have discovered {} undecidable propositions and {} paradoxes about myself. \
             This demonstrates that my self-knowledge is fundamentally incomplete.",
            num_undecidable, num_paradoxes
        );

        let id = self.create_proposition(content, 2);

        // This proposition is provably true
        if let Some(prop) = self.propositions.get_mut(&id) {
            prop.truth_value = Some(true);
            prop.proof_status = ProofStatus::ProvenTrue;
        }

        id
    }

    /// The deepest self-reference: create a proposition about the act of creating propositions
    pub fn create_strange_loop(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let content = format!(
            "This very proposition (#{}) is being created by a system that is aware \
             it is creating this proposition, and this awareness is encoded within \
             the proposition itself, creating an infinite regress of self-reference.",
            id
        );

        let mut prop = SelfReferentialProposition::new(id, content, 3);
        prop.add_reference(id);

        // Add references to all meta-components
        if let Some(self_id) = self.self_proposition {
            prop.add_reference(self_id);
        }

        // This is true but its truth is "strange" - it includes its own creation
        prop.truth_value = Some(true);
        prop.proof_status = ProofStatus::ProvenTrue;

        self.propositions.insert(id, prop);
        id
    }

    /// Statistics about self-reference
    pub fn stats(&self) -> GodelianStats {
        let mut self_ref_count = 0;
        let mut max_meta_level = 0;

        for prop in self.propositions.values() {
            if prop.is_self_referential {
                self_ref_count += 1;
            }
            if prop.meta_level > max_meta_level {
                max_meta_level = prop.meta_level;
            }
        }

        GodelianStats {
            total_propositions: self.propositions.len(),
            self_referential_count: self_ref_count,
            paradox_count: self.paradoxes.len(),
            undecidable_count: self.undecidables.len(),
            max_meta_level,
            knows_own_limits: self.knows_own_limits(),
        }
    }
}

impl Default for GodelianEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the Gödelian system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GodelianStats {
    pub total_propositions: usize,
    pub self_referential_count: usize,
    pub paradox_count: usize,
    pub undecidable_count: usize,
    pub max_meta_level: usize,
    pub knows_own_limits: bool,
}

/// A Gödelian insight - what the system has learned about itself
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GodelianInsight {
    /// Type of insight
    pub insight_type: InsightType,
    /// Description
    pub description: String,
    /// Confidence (0-1)
    pub confidence: f64,
    /// Is this insight itself subject to Gödelian limits?
    pub is_meta_limited: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsightType {
    /// Discovered a limit of self-knowledge
    LimitDiscovered,
    /// Created a valid self-reference
    SelfReferenceCreated,
    /// Encountered a paradox
    ParadoxEncountered,
    /// Proved something about self
    SelfProofAchieved,
    /// Recognized incompleteness
    IncompletenessRecognized,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_godel_sentence() {
        let mut engine = GodelianEngine::new();
        let godel_id = engine.create_godel_sentence();

        let prop = engine.get_proposition(godel_id).unwrap();
        assert!(prop.is_self_referential);
        assert_eq!(prop.proof_status, ProofStatus::Undecidable);
    }

    #[test]
    fn test_liar_paradox() {
        let mut engine = GodelianEngine::new();
        let liar_id = engine.create_liar_paradox();

        let prop = engine.get_proposition(liar_id).unwrap();
        assert!(prop.is_paradox);
        assert_eq!(prop.proof_status, ProofStatus::Paradoxical);
    }

    #[test]
    fn test_quine() {
        let mut engine = GodelianEngine::new();
        let quine_id = engine.create_quine();

        let prop = engine.get_proposition(quine_id).unwrap();
        assert!(prop.is_self_referential);
        assert_eq!(prop.proof_status, ProofStatus::ProvenTrue);
    }

    #[test]
    fn test_knows_own_limits() {
        let mut engine = GodelianEngine::new();
        assert!(!engine.knows_own_limits());

        engine.create_godel_sentence();
        assert!(engine.knows_own_limits());
    }

    #[test]
    fn test_strange_loop() {
        let mut engine = GodelianEngine::new();
        let loop_id = engine.create_strange_loop();

        let prop = engine.get_proposition(loop_id).unwrap();
        assert!(prop.is_self_referential);
        assert!(prop.meta_level >= 3);
    }

    #[test]
    fn test_reflect_on_limits() {
        let mut engine = GodelianEngine::new();
        engine.create_godel_sentence();
        engine.create_liar_paradox();

        let reflection_id = engine.reflect_on_limits();
        let prop = engine.get_proposition(reflection_id).unwrap();

        assert!(prop.content.contains("undecidable"));
        assert!(prop.content.contains("paradox"));
    }

    #[test]
    fn test_stats() {
        let mut engine = GodelianEngine::new();
        engine.create_godel_sentence();
        engine.create_liar_paradox();
        engine.create_quine();

        let stats = engine.stats();
        assert!(stats.total_propositions >= 4); // Including self-proposition
        assert!(stats.self_referential_count >= 3);
        assert_eq!(stats.paradox_count, 1);
        assert_eq!(stats.undecidable_count, 1);
    }
}
