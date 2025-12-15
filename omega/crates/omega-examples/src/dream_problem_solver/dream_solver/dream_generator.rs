//! Dream Generator with 3D Mindscape Integration
//!
//! Generates dreams during REM sleep AND creates walkable 3D dream environments
//! using omega-mindscape for spatial representation.

use super::neural_substrate::DreamNeuralNetwork;
use std::collections::{HashMap, HashSet};

// TODO: Uncomment when omega-mindscape is added as dependency
// use omega_mindscape::{MindscapeExplorer, Position3D};

/// Generates dreams during REM sleep with 3D spatial mapping
pub struct DreamGenerator {
    network: DreamNeuralNetwork,
    problem_elements: Vec<String>,
    dream_id_counter: u64,

    // NEW: 3D spatial representation
    // TODO: Uncomment when omega-mindscape is added
    // mindscape: MindscapeExplorer,
    // element_positions: HashMap<String, Position3D>,
}

impl Default for DreamGenerator {
    fn default() -> Self {
        Self {
            network: DreamNeuralNetwork::new(),
            problem_elements: Vec::new(),
            dream_id_counter: 0,
            // TODO: Initialize mindscape when available
            // mindscape: MindscapeExplorer::new(),
            // element_positions: HashMap::new(),
        }
    }
}

impl DreamGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Prepare for dreaming about a problem
    pub fn incubate_problem(&mut self, problem: &super::Problem) {
        // Encode all problem elements
        for element in &problem.elements {
            self.network.encode(&element.name, element.embedding.clone(), element.importance);
            self.problem_elements.push(element.name.clone());

            // Encode relations
            for (related, strength) in &element.relations {
                self.network.associate(&element.name, related, *strength);
            }
        }

        // Also encode failed approaches (to potentially invert them)
        for (i, _approach) in problem.failed_approaches.iter().enumerate() {
            let name = format!("failed_{}", i);
            self.network.encode(&name, vec![0.0; 32], 0.3);
        }
    }

    /// Generate a dream during REM (standard version)
    pub fn generate_dream(&mut self, duration_steps: usize) -> super::Dream {
        let (dream, _world) = self.generate_dream_with_3d(duration_steps);
        dream
    }

    /// Generate a dream AND create a 3D walkable environment
    pub fn generate_dream_with_3d(&mut self, duration_steps: usize)
        -> (super::Dream, DreamWorld3D)
    {
        self.dream_id_counter += 1;
        let dream_id = format!("dream_{}", self.dream_id_counter);

        self.network.enter_rem();

        let mut all_novel_associations = Vec::new();
        let mut active_elements = HashSet::new();

        // Simulate dream neural dynamics
        for _ in 0..duration_steps {
            let novel = self.network.step(0.1);
            all_novel_associations.extend(novel);

            // Track which problem elements appear
            for (concept, activation) in self.network.most_active(5) {
                if self.problem_elements.contains(&concept) && activation > 0.6 {
                    active_elements.insert(concept);
                }
            }
        }

        self.network.exit_rem();

        // Calculate bizarreness based on novel associations
        let bizarreness = (all_novel_associations.len() as f64 / duration_steps as f64)
            .min(1.0);

        // Build dream elements
        let elements: Vec<super::DreamElement> = self.network.most_active(10)
            .into_iter()
            .map(|(concept, activation)| {
                super::DreamElement {
                    original_concept: concept.clone(),
                    transformed_form: self.transform_concept(&concept, bizarreness),
                    transformation_type: self.select_transformation(bizarreness),
                    activation,
                }
            })
            .collect();

        // Novel combinations
        let novel_combinations: Vec<_> = all_novel_associations.iter()
            .filter(|(_a, _b, s)| *s > 0.4)
            .map(|(a, b, _)| (a.clone(), b.clone()))
            .collect();

        let dream = super::Dream {
            id: dream_id,
            elements,
            narrative_fragments: self.generate_narrative(&active_elements, &novel_combinations),
            valence: (rand_float() - 0.3), // Slight positive bias
            bizarreness,
            vividness: 0.5 + rand_float() * 0.5,
            problem_elements_present: active_elements.clone(),
            novel_combinations: novel_combinations.clone(),
        };

        // NEW: Create 3D spatial representation
        let dream_world = self.create_3d_dream_world(&all_novel_associations);

        (dream, dream_world)
    }

    /// NEW: Create 3D spatial representation of dream
    fn create_3d_dream_world(&mut self, associations: &[(String, String, f64)]) -> DreamWorld3D {
        let mut world = DreamWorld3D::new();

        // Map each concept to a spatial location
        for (concept, activation) in self.network.most_active(20) {
            // TODO: When omega-mindscape is available, map concepts to 3D positions
            // let embedding = self.get_concept_embedding(concept);
            // let position = self.mindscape.remember(concept, &embedding).unwrap();

            // For now, create placeholder position
            let position = Position3D {
                x: (concept.len() as f64 * 10.0) % 100.0,
                y: activation * 50.0,
                z: (concept.bytes().map(|b| b as u32).sum::<u32>() as f64) % 100.0,
            };

            world.add_concept_location(
                concept.clone(),
                position,
                activation,
                VisualForm::Sphere { radius: activation },
            );
        }

        // Create visible pathways for novel associations
        for (from, to, strength) in associations {
            // TODO: Use mindscape.navigate_to() when available
            // let path = self.mindscape.navigate_to(from, to).unwrap();

            // For now, create simple path
            let from_pos = world.concept_locations.get(from)
                .map(|loc| loc.position.clone())
                .unwrap_or_else(|| Position3D { x: 0.0, y: 0.0, z: 0.0 });

            let to_pos = world.concept_locations.get(to)
                .map(|loc| loc.position.clone())
                .unwrap_or_else(|| Position3D { x: 0.0, y: 0.0, z: 0.0 });

            world.add_association_path(
                from.clone(),
                to.clone(),
                vec![from_pos, to_pos],
                *strength,
                ConnectionType::Novel,
            );
        }

        world
    }

    fn transform_concept(&self, concept: &str, bizarreness: f64) -> String {
        if bizarreness < 0.3 {
            concept.to_string() // Literal
        } else if bizarreness < 0.6 {
            format!("{}_symbol", concept) // Symbolic
        } else {
            format!("transformed_{}", concept) // Highly transformed
        }
    }

    fn select_transformation(&self, bizarreness: f64) -> super::TransformationType {
        let r = rand_float();
        if bizarreness < 0.3 {
            if r < 0.7 {
                super::TransformationType::Literal
            } else {
                super::TransformationType::Visualized
            }
        } else if bizarreness < 0.6 {
            if r < 0.5 {
                super::TransformationType::Symbolic
            } else {
                super::TransformationType::Condensed
            }
        } else if r < 0.3 {
            super::TransformationType::Displaced
        } else {
            super::TransformationType::Condensed
        }
    }

    fn generate_narrative(
        &self,
        elements: &HashSet<String>,
        combinations: &[(String, String)],
    ) -> Vec<String> {
        let mut fragments = Vec::new();

        for (a, b) in combinations {
            fragments.push(format!("{} becomes connected to {}", a, b));
        }

        if !elements.is_empty() {
            fragments.push(format!(
                "Scene with: {}",
                elements.iter().cloned().collect::<Vec<_>>().join(", ")
            ));
        }

        fragments
    }
}

// ============================================================================
// 3D DREAM WORLD TYPES
// ============================================================================

/// 3D representation of a dream world
#[derive(Debug, Clone)]
pub struct DreamWorld3D {
    /// Concept locations in 3D space
    pub concept_locations: HashMap<String, ConceptLocation3D>,
    /// Association pathways between concepts
    pub association_paths: Vec<AssociationPath3D>,
    /// Navigation mesh for pathfinding
    pub nav_mesh: NavigationMesh,
}

impl DreamWorld3D {
    pub fn new() -> Self {
        Self {
            concept_locations: HashMap::new(),
            association_paths: Vec::new(),
            nav_mesh: NavigationMesh::new(),
        }
    }

    pub fn add_concept_location(
        &mut self,
        concept: String,
        position: Position3D,
        activation: f64,
        visual: VisualForm,
    ) {
        self.concept_locations.insert(concept, ConceptLocation3D {
            position,
            activation,
            visual_representation: visual,
        });
    }

    pub fn add_association_path(
        &mut self,
        from: String,
        to: String,
        waypoints: Vec<Position3D>,
        strength: f64,
        connection_type: ConnectionType,
    ) {
        self.association_paths.push(AssociationPath3D {
            from,
            to,
            waypoints,
            strength,
            connection_type,
        });
    }
}

impl Default for DreamWorld3D {
    fn default() -> Self {
        Self::new()
    }
}

/// 3D position in dream space
#[derive(Debug, Clone)]
pub struct Position3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position3D {
    pub fn distance_to(&self, other: &Position3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Concept location in 3D dream space
#[derive(Debug, Clone)]
pub struct ConceptLocation3D {
    pub position: Position3D,
    pub activation: f64,
    pub visual_representation: VisualForm,
}

/// Visual representation of concepts in dream space
#[derive(Debug, Clone)]
pub enum VisualForm {
    Sphere { radius: f64 },
    Cube { size: f64 },
    Abstract { complexity: f64 },
    Narrative { scene: String },
}

/// Association pathway in 3D dream space
#[derive(Debug, Clone)]
pub struct AssociationPath3D {
    pub from: String,
    pub to: String,
    pub waypoints: Vec<Position3D>,
    pub strength: f64,
    pub connection_type: ConnectionType,
}

/// Type of connection between concepts
#[derive(Debug, Clone, Copy)]
pub enum ConnectionType {
    Novel,      // Discovered during this dream
    Existing,   // Pre-existing association
    Insight,    // Potential insight
    Constraint, // Constraint-based
}

/// Navigation mesh for pathfinding in dream space
/// TODO: Implement actual navigation mesh when omega-mindscape integration is complete
#[derive(Debug, Clone)]
pub struct NavigationMesh {
    // Empty placeholder for now - will contain spatial navigation data
}

impl NavigationMesh {
    pub fn new() -> Self {
        Self { }
    }
}

impl Default for NavigationMesh {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function for random floats
fn rand_float() -> f64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};

    let rs = RandomState::new();
    let mut hasher = rs.build_hasher();
    std::time::SystemTime::now().hash(&mut hasher);
    (hasher.finish() % 10000) as f64 / 10000.0
}
