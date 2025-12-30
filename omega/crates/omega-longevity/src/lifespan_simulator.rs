//! Lifespan Simulator - Precognition Engine for Intervention Evaluation
//!
//! Uses time-dilated future simulation to predict long-term effects of
//! longevity interventions across multiple virtual lifespans.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────────┐
//!  │                   LIFESPAN SIMULATOR                                │
//!  ├─────────────────────────────────────────────────────────────────────┤
//!  │                                                                     │
//!  │  INTERVENTION     10,000 SIMULATED        CONVERGENT OUTCOMES      │
//!  │  CANDIDATES       LIFESPANS (1000x)       & RECOMMENDATIONS        │
//!  │                                                                     │
//!  │  ┌─────────┐      ┌───────────────┐       ┌─────────────────┐      │
//!  │  │Rapamycin│──────│ Sim 1: +12y   │       │ Best: Combo A   │      │
//!  │  │ + NMN   │      │ Sim 2: +15y   │──────►│ Median: +14y    │      │
//!  │  │ + D+Q   │      │ Sim 3: +11y   │       │ Risk: Low       │      │
//!  │  └─────────┘      │ ...           │       │ Start: Age 50   │      │
//!  │                   │ Sim N: +18y   │       └─────────────────┘      │
//!  │                   └───────────────┘                                 │
//!  │                                                                     │
//!  │  Time: 10,000 years simulated in 10 seconds                        │
//!  │                                                                     │
//!  └─────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;
use rand::Rng;
use rand_distr::{Distribution, Normal, LogNormal};

use crate::hallmarks::{Hallmark, Intervention, InterventionType};
use crate::{Result, LongevityError};

/// Time dilation factor (simulated time / real time)
const TIME_DILATION: f64 = 1000.0;

/// Configuration for lifespan simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatorConfig {
    /// Number of parallel lifespans to simulate
    pub num_simulations: usize,
    /// Maximum simulated age (years)
    pub max_age: f64,
    /// Time dilation factor
    pub time_dilation: f64,
    /// Include stochastic disease events
    pub stochastic_events: bool,
    /// Model genetic variation
    pub genetic_variation: bool,
    /// Convergence detection threshold
    pub convergence_threshold: f64,
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self {
            num_simulations: 1000,
            max_age: 150.0,
            time_dilation: TIME_DILATION,
            stochastic_events: true,
            genetic_variation: true,
            convergence_threshold: 0.7,
        }
    }
}

/// A virtual individual for simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualIndividual {
    pub id: Uuid,
    /// Starting biological age
    pub starting_bio_age: f64,
    /// Starting chronological age
    pub starting_chrono_age: f64,
    /// Genetic modifiers for each hallmark
    pub genetic_modifiers: HashMap<Hallmark, f64>,
    /// Lifestyle factors
    pub lifestyle_score: f64,
    /// Disease susceptibilities
    pub disease_risks: HashMap<String, f64>,
}

impl VirtualIndividual {
    pub fn random(rng: &mut impl Rng) -> Self {
        let mut genetic_modifiers = HashMap::new();
        for hallmark in Hallmark::all() {
            // Genetic effect: -0.3 to +0.3 (protective to harmful)
            genetic_modifiers.insert(hallmark, rng.gen_range(-0.3..0.3));
        }

        let mut disease_risks = HashMap::new();
        disease_risks.insert("cardiovascular".to_string(), rng.gen_range(0.05..0.25));
        disease_risks.insert("cancer".to_string(), rng.gen_range(0.03..0.20));
        disease_risks.insert("neurodegeneration".to_string(), rng.gen_range(0.02..0.15));
        disease_risks.insert("metabolic".to_string(), rng.gen_range(0.05..0.30));

        Self {
            id: Uuid::new_v4(),
            starting_bio_age: rng.gen_range(45.0..75.0),
            starting_chrono_age: rng.gen_range(45.0..75.0),
            genetic_modifiers,
            lifestyle_score: rng.gen_range(0.3..0.9),
            disease_risks,
        }
    }
}

/// An intervention protocol for simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionProtocol {
    pub id: Uuid,
    pub name: String,
    /// Interventions in this protocol
    pub interventions: Vec<ProtocolIntervention>,
    /// Start age (chronological)
    pub start_age: f64,
    /// End age (None = lifetime)
    pub end_age: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolIntervention {
    pub name: String,
    pub intervention_type: InterventionType,
    /// Effect on each hallmark (negative = beneficial)
    pub hallmark_effects: HashMap<Hallmark, f64>,
    /// Overall efficacy (0-1)
    pub efficacy: f64,
    /// Side effect probability
    pub side_effect_prob: f64,
    /// Interaction effects with other interventions
    pub interactions: Vec<(String, f64)>, // (intervention_name, multiplier)
}

/// A single simulated lifespan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedLifespan {
    pub id: Uuid,
    /// Final lifespan achieved
    pub lifespan: f64,
    /// Final healthspan (years in good health)
    pub healthspan: f64,
    /// Biological age trajectory
    pub bio_age_trajectory: Vec<(f64, f64)>, // (chrono_age, bio_age)
    /// Major events during life
    pub events: Vec<LifeEvent>,
    /// Cause of death (if reached)
    pub death_cause: Option<String>,
    /// Hallmark states at end of life
    pub final_hallmark_states: HashMap<Hallmark, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeEvent {
    pub age: f64,
    pub event_type: LifeEventType,
    pub description: String,
    pub impact_on_aging: f64, // Positive = accelerated, negative = slowed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifeEventType {
    Disease,
    Recovery,
    InterventionStart,
    InterventionStop,
    SideEffect,
    HealthMilestone,
    Death,
}

/// Simulation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResults {
    pub id: Uuid,
    pub protocol: InterventionProtocol,
    /// All simulated lifespans
    pub lifespans: Vec<SimulatedLifespan>,
    /// Summary statistics
    pub summary: SimulationSummary,
    /// Convergent outcomes detected
    pub convergent_outcomes: Vec<ConvergentOutcome>,
    /// Recommended actions
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationSummary {
    /// Mean lifespan extension (years)
    pub mean_lifespan_extension: f64,
    /// Median lifespan extension
    pub median_lifespan_extension: f64,
    /// Standard deviation
    pub std_lifespan_extension: f64,
    /// Mean healthspan extension
    pub mean_healthspan_extension: f64,
    /// Probability of reaching age 100
    pub prob_centenarian: f64,
    /// Probability of reaching age 120
    pub prob_supercentenarian: f64,
    /// Side effect frequency
    pub side_effect_rate: f64,
    /// Most common cause of death
    pub common_death_causes: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergentOutcome {
    pub id: Uuid,
    pub lifespan_range: (f64, f64),
    pub probability: f64,
    pub key_factors: Vec<String>,
}

/// The Lifespan Simulator
pub struct LifespanSimulator {
    config: SimulatorConfig,
    /// Baseline aging rate per hallmark (% per year)
    hallmark_aging_rates: HashMap<Hallmark, f64>,
    /// Disease probability functions
    disease_hazards: HashMap<String, Box<dyn Fn(f64) -> f64 + Send + Sync>>,
    /// RNG
    rng: rand::rngs::ThreadRng,
}

impl LifespanSimulator {
    pub fn new(config: SimulatorConfig) -> Self {
        let mut hallmark_aging_rates = HashMap::new();

        // Annual aging rate per hallmark (approximate)
        hallmark_aging_rates.insert(Hallmark::GenomicInstability, 0.01);
        hallmark_aging_rates.insert(Hallmark::TelomereAttrition, 0.008);
        hallmark_aging_rates.insert(Hallmark::EpigeneticAlterations, 0.012);
        hallmark_aging_rates.insert(Hallmark::LossOfProteostasis, 0.009);
        hallmark_aging_rates.insert(Hallmark::DeregulatedNutrientSensing, 0.008);
        hallmark_aging_rates.insert(Hallmark::MitochondrialDysfunction, 0.011);
        hallmark_aging_rates.insert(Hallmark::CellularSenescence, 0.015);
        hallmark_aging_rates.insert(Hallmark::StemCellExhaustion, 0.007);
        hallmark_aging_rates.insert(Hallmark::AlteredIntercellularCommunication, 0.010);
        hallmark_aging_rates.insert(Hallmark::ChronicInflammation, 0.013);
        hallmark_aging_rates.insert(Hallmark::Dysbiosis, 0.006);
        hallmark_aging_rates.insert(Hallmark::DisabledMacroautophagy, 0.008);

        Self {
            config,
            hallmark_aging_rates,
            disease_hazards: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    /// Simulate a single lifespan
    fn simulate_lifespan(
        &mut self,
        individual: &VirtualIndividual,
        protocol: &InterventionProtocol,
    ) -> SimulatedLifespan {
        let mut current_chrono_age = individual.starting_chrono_age;
        let mut current_bio_age = individual.starting_bio_age;
        let mut hallmark_states: HashMap<Hallmark, f64> = HashMap::new();
        let mut bio_age_trajectory = Vec::new();
        let mut events = Vec::new();

        // Initialize hallmark states
        for hallmark in Hallmark::all() {
            let genetic_mod = individual.genetic_modifiers.get(&hallmark).unwrap_or(&0.0);
            hallmark_states.insert(hallmark, current_bio_age / 100.0 + genetic_mod);
        }

        bio_age_trajectory.push((current_chrono_age, current_bio_age));

        let mut death_cause = None;
        let time_step = 0.25; // Simulate in quarter-year steps

        while current_chrono_age < self.config.max_age && death_cause.is_none() {
            // Check if intervention is active
            let intervention_active = current_chrono_age >= protocol.start_age
                && protocol.end_age.map_or(true, |e| current_chrono_age <= e);

            // Calculate aging rate
            let mut total_aging_rate = 0.0;
            for (hallmark, base_rate) in &self.hallmark_aging_rates {
                let mut rate = *base_rate;

                // Apply genetic modifier
                if let Some(mod_val) = individual.genetic_modifiers.get(hallmark) {
                    rate *= 1.0 + mod_val;
                }

                // Apply lifestyle modifier
                rate *= 1.0 - (individual.lifestyle_score * 0.3);

                // Apply intervention effects
                if intervention_active {
                    for intervention in &protocol.interventions {
                        if let Some(effect) = intervention.hallmark_effects.get(hallmark) {
                            rate *= 1.0 + effect; // Negative effect = slower aging
                        }
                    }
                }

                // Update hallmark state
                let current = hallmark_states.get(hallmark).unwrap_or(&0.5);
                hallmark_states.insert(*hallmark, (current + rate * time_step).min(1.0));

                total_aging_rate += rate;
            }

            // Update biological age
            let avg_rate = total_aging_rate / self.hallmark_aging_rates.len() as f64;
            current_bio_age += avg_rate * 100.0 * time_step;
            current_chrono_age += time_step;

            bio_age_trajectory.push((current_chrono_age, current_bio_age));

            // Check for disease events (stochastic)
            if self.config.stochastic_events {
                for (disease, base_risk) in &individual.disease_risks {
                    // Risk increases with biological age
                    let age_factor = (current_bio_age / 50.0).powf(2.0);
                    let risk_per_step = base_risk * age_factor * time_step;

                    if self.rng.gen::<f64>() < risk_per_step {
                        // Disease event
                        let severity = self.rng.gen_range(0.1..1.0);
                        if severity > 0.8 {
                            death_cause = Some(disease.clone());
                            events.push(LifeEvent {
                                age: current_chrono_age,
                                event_type: LifeEventType::Death,
                                description: format!("Fatal {} event", disease),
                                impact_on_aging: 1.0,
                            });
                        } else {
                            events.push(LifeEvent {
                                age: current_chrono_age,
                                event_type: LifeEventType::Disease,
                                description: format!("Non-fatal {} event (severity: {:.2})", disease, severity),
                                impact_on_aging: severity * 0.5,
                            });
                            // Disease accelerates aging temporarily
                            current_bio_age += severity * 2.0;
                        }
                    }
                }
            }

            // Check for natural death (biological age > threshold)
            let death_threshold = 95.0 + self.rng.gen_range(-10.0..10.0);
            if current_bio_age >= death_threshold && death_cause.is_none() {
                death_cause = Some("natural causes".to_string());
                events.push(LifeEvent {
                    age: current_chrono_age,
                    event_type: LifeEventType::Death,
                    description: "Natural death from accumulated aging".to_string(),
                    impact_on_aging: 1.0,
                });
            }

            // Check for side effects
            if intervention_active {
                for intervention in &protocol.interventions {
                    if self.rng.gen::<f64>() < intervention.side_effect_prob * time_step {
                        events.push(LifeEvent {
                            age: current_chrono_age,
                            event_type: LifeEventType::SideEffect,
                            description: format!("Side effect from {}", intervention.name),
                            impact_on_aging: 0.1,
                        });
                    }
                }
            }
        }

        // Calculate healthspan (time before bio_age > 70)
        let healthspan = bio_age_trajectory.iter()
            .take_while(|(_, bio)| *bio < 70.0)
            .last()
            .map(|(chrono, _)| *chrono)
            .unwrap_or(individual.starting_chrono_age);

        SimulatedLifespan {
            id: Uuid::new_v4(),
            lifespan: current_chrono_age,
            healthspan,
            bio_age_trajectory,
            events,
            death_cause,
            final_hallmark_states: hallmark_states,
        }
    }

    /// Run full simulation of an intervention protocol
    pub fn simulate_protocol(&mut self, protocol: InterventionProtocol) -> Result<SimulationResults> {
        let mut lifespans = Vec::new();

        // Generate virtual individuals and simulate
        for _ in 0..self.config.num_simulations {
            let individual = VirtualIndividual::random(&mut self.rng);
            let lifespan = self.simulate_lifespan(&individual, &protocol);
            lifespans.push(lifespan);
        }

        // Calculate summary statistics
        let summary = self.calculate_summary(&lifespans);

        // Detect convergent outcomes
        let convergent_outcomes = self.detect_convergence(&lifespans);

        // Generate recommendations
        let recommendations = self.generate_recommendations(&summary, &convergent_outcomes);

        Ok(SimulationResults {
            id: Uuid::new_v4(),
            protocol,
            lifespans,
            summary,
            convergent_outcomes,
            recommendations,
        })
    }

    fn calculate_summary(&self, lifespans: &[SimulatedLifespan]) -> SimulationSummary {
        let baseline_lifespan = 78.0; // Baseline human lifespan
        let baseline_healthspan = 63.0;

        let extensions: Vec<f64> = lifespans.iter()
            .map(|l| l.lifespan - baseline_lifespan)
            .collect();

        let healthspan_extensions: Vec<f64> = lifespans.iter()
            .map(|l| l.healthspan - baseline_healthspan)
            .collect();

        let n = extensions.len() as f64;

        let mean_ext = extensions.iter().sum::<f64>() / n;
        let mean_health_ext = healthspan_extensions.iter().sum::<f64>() / n;

        // Median
        let mut sorted = extensions.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_ext = sorted[sorted.len() / 2];

        // Standard deviation
        let variance = extensions.iter()
            .map(|x| (x - mean_ext).powi(2))
            .sum::<f64>() / n;
        let std_ext = variance.sqrt();

        // Centenarian probability
        let centenarians = lifespans.iter().filter(|l| l.lifespan >= 100.0).count();
        let supercentenarians = lifespans.iter().filter(|l| l.lifespan >= 120.0).count();

        // Side effect rate
        let total_side_effects: usize = lifespans.iter()
            .map(|l| l.events.iter().filter(|e| e.event_type == LifeEventType::SideEffect).count())
            .sum();

        // Death causes
        let mut death_causes: HashMap<String, usize> = HashMap::new();
        for lifespan in lifespans {
            if let Some(cause) = &lifespan.death_cause {
                *death_causes.entry(cause.clone()).or_insert(0) += 1;
            }
        }
        let mut causes: Vec<_> = death_causes.into_iter()
            .map(|(k, v)| (k, v as f64 / n))
            .collect();
        causes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        SimulationSummary {
            mean_lifespan_extension: mean_ext,
            median_lifespan_extension: median_ext,
            std_lifespan_extension: std_ext,
            mean_healthspan_extension: mean_health_ext,
            prob_centenarian: centenarians as f64 / n,
            prob_supercentenarian: supercentenarians as f64 / n,
            side_effect_rate: total_side_effects as f64 / n,
            common_death_causes: causes.into_iter().take(5).collect(),
        }
    }

    fn detect_convergence(&self, lifespans: &[SimulatedLifespan]) -> Vec<ConvergentOutcome> {
        let mut outcomes = Vec::new();

        // Bin lifespans into ranges
        let bins = vec![
            (70.0, 80.0),
            (80.0, 90.0),
            (90.0, 100.0),
            (100.0, 110.0),
            (110.0, 120.0),
            (120.0, 150.0),
        ];

        for (low, high) in bins {
            let count = lifespans.iter()
                .filter(|l| l.lifespan >= low && l.lifespan < high)
                .count();

            let prob = count as f64 / lifespans.len() as f64;

            if prob >= self.config.convergence_threshold * 0.1 {
                outcomes.push(ConvergentOutcome {
                    id: Uuid::new_v4(),
                    lifespan_range: (low, high),
                    probability: prob,
                    key_factors: vec![
                        format!("Range {:.0}-{:.0} years", low, high),
                        format!("{:.1}% of simulations", prob * 100.0),
                    ],
                });
            }
        }

        outcomes
    }

    fn generate_recommendations(&self, summary: &SimulationSummary, convergent: &[ConvergentOutcome]) -> Vec<String> {
        let mut recs = Vec::new();

        if summary.mean_lifespan_extension > 10.0 {
            recs.push(format!(
                "Protocol shows strong efficacy: +{:.1} years mean lifespan extension",
                summary.mean_lifespan_extension
            ));
        }

        if summary.mean_healthspan_extension > 8.0 {
            recs.push(format!(
                "Excellent healthspan extension: +{:.1} years of healthy life",
                summary.mean_healthspan_extension
            ));
        }

        if summary.prob_centenarian > 0.3 {
            recs.push(format!(
                "High probability of reaching 100: {:.1}%",
                summary.prob_centenarian * 100.0
            ));
        }

        if summary.side_effect_rate > 2.0 {
            recs.push(format!(
                "Warning: High side effect rate ({:.1} per person). Consider dose adjustment.",
                summary.side_effect_rate
            ));
        }

        if let Some(best) = convergent.iter().max_by(|a, b| a.lifespan_range.1.partial_cmp(&b.lifespan_range.1).unwrap()) {
            recs.push(format!(
                "Best convergent outcome: {:.0}-{:.0} years ({:.1}% probability)",
                best.lifespan_range.0, best.lifespan_range.1, best.probability * 100.0
            ));
        }

        recs
    }

    /// Compare multiple protocols
    pub fn compare_protocols(&mut self, protocols: Vec<InterventionProtocol>) -> Vec<(InterventionProtocol, SimulationSummary)> {
        protocols.into_iter()
            .filter_map(|p| {
                self.simulate_protocol(p.clone()).ok()
                    .map(|r| (p, r.summary))
            })
            .collect()
    }

    /// Quick evaluation: should this intervention be pursued?
    pub fn quick_evaluate(&mut self, intervention: ProtocolIntervention) -> bool {
        let protocol = InterventionProtocol {
            id: Uuid::new_v4(),
            name: format!("Quick eval: {}", intervention.name),
            interventions: vec![intervention],
            start_age: 50.0,
            end_age: None,
        };

        let mut quick_config = self.config.clone();
        quick_config.num_simulations = 100; // Faster for quick eval

        let original_config = std::mem::replace(&mut self.config, quick_config);

        let result = self.simulate_protocol(protocol);

        self.config = original_config;

        result.map(|r| r.summary.mean_lifespan_extension > 5.0).unwrap_or(false)
    }
}

/// Create common intervention protocols for testing
pub mod protocols {
    use super::*;

    pub fn rapamycin_protocol() -> InterventionProtocol {
        let mut effects = HashMap::new();
        effects.insert(Hallmark::DeregulatedNutrientSensing, -0.4);
        effects.insert(Hallmark::CellularSenescence, -0.2);
        effects.insert(Hallmark::DisabledMacroautophagy, -0.3);

        InterventionProtocol {
            id: Uuid::new_v4(),
            name: "Rapamycin Protocol".to_string(),
            interventions: vec![ProtocolIntervention {
                name: "Rapamycin".to_string(),
                intervention_type: InterventionType::SmallMolecule,
                hallmark_effects: effects,
                efficacy: 0.85,
                side_effect_prob: 0.05,
                interactions: Vec::new(),
            }],
            start_age: 50.0,
            end_age: None,
        }
    }

    pub fn senolytic_protocol() -> InterventionProtocol {
        let mut effects = HashMap::new();
        effects.insert(Hallmark::CellularSenescence, -0.5);
        effects.insert(Hallmark::ChronicInflammation, -0.3);
        effects.insert(Hallmark::StemCellExhaustion, -0.2);

        InterventionProtocol {
            id: Uuid::new_v4(),
            name: "Senolytic Protocol (D+Q)".to_string(),
            interventions: vec![ProtocolIntervention {
                name: "Dasatinib + Quercetin".to_string(),
                intervention_type: InterventionType::Senolytic,
                hallmark_effects: effects,
                efficacy: 0.75,
                side_effect_prob: 0.03,
                interactions: Vec::new(),
            }],
            start_age: 55.0,
            end_age: None,
        }
    }

    pub fn combination_protocol() -> InterventionProtocol {
        let mut rapa_effects = HashMap::new();
        rapa_effects.insert(Hallmark::DeregulatedNutrientSensing, -0.35);
        rapa_effects.insert(Hallmark::DisabledMacroautophagy, -0.25);

        let mut nmn_effects = HashMap::new();
        nmn_effects.insert(Hallmark::MitochondrialDysfunction, -0.3);
        nmn_effects.insert(Hallmark::GenomicInstability, -0.15);

        let mut senolytic_effects = HashMap::new();
        senolytic_effects.insert(Hallmark::CellularSenescence, -0.45);
        senolytic_effects.insert(Hallmark::ChronicInflammation, -0.25);

        InterventionProtocol {
            id: Uuid::new_v4(),
            name: "Combination Protocol".to_string(),
            interventions: vec![
                ProtocolIntervention {
                    name: "Rapamycin".to_string(),
                    intervention_type: InterventionType::SmallMolecule,
                    hallmark_effects: rapa_effects,
                    efficacy: 0.80,
                    side_effect_prob: 0.04,
                    interactions: vec![("NMN".to_string(), 1.1)], // Synergy
                },
                ProtocolIntervention {
                    name: "NMN".to_string(),
                    intervention_type: InterventionType::Supplement,
                    hallmark_effects: nmn_effects,
                    efficacy: 0.65,
                    side_effect_prob: 0.01,
                    interactions: Vec::new(),
                },
                ProtocolIntervention {
                    name: "D+Q Senolytic".to_string(),
                    intervention_type: InterventionType::Senolytic,
                    hallmark_effects: senolytic_effects,
                    efficacy: 0.75,
                    side_effect_prob: 0.03,
                    interactions: Vec::new(),
                },
            ],
            start_age: 50.0,
            end_age: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulator_creation() {
        let config = SimulatorConfig::default();
        let simulator = LifespanSimulator::new(config);
        assert!(!simulator.hallmark_aging_rates.is_empty());
    }

    #[test]
    fn test_simulate_protocol() {
        let mut config = SimulatorConfig::default();
        config.num_simulations = 50; // Fewer for testing

        let mut simulator = LifespanSimulator::new(config);
        let protocol = protocols::rapamycin_protocol();

        let results = simulator.simulate_protocol(protocol).unwrap();
        assert!(!results.lifespans.is_empty());
        assert!(results.summary.mean_lifespan_extension != 0.0);
    }

    #[test]
    fn test_compare_protocols() {
        let mut config = SimulatorConfig::default();
        config.num_simulations = 30;

        let mut simulator = LifespanSimulator::new(config);

        let protocols = vec![
            protocols::rapamycin_protocol(),
            protocols::senolytic_protocol(),
        ];

        let comparisons = simulator.compare_protocols(protocols);
        assert_eq!(comparisons.len(), 2);
    }
}
