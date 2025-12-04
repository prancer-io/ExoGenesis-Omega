use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

pub type LoopId = String;
pub type CycleId = String;

/// 7 Temporal Loops - Multi-scale feedback and learning cycles
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LoopType {
    /// Loop 1: Reflexive (milliseconds) - Immediate sensory-motor feedback
    Reflexive = 1,
    /// Loop 2: Reactive (seconds) - Quick decision-making and responses
    Reactive = 2,
    /// Loop 3: Adaptive (minutes to hours) - Learning from recent experiences
    Adaptive = 3,
    /// Loop 4: Deliberative (days) - Strategic planning and reflection
    Deliberative = 4,
    /// Loop 5: Evolutionary (weeks to months) - Systematic improvement and evolution
    Evolutionary = 5,
    /// Loop 6: Transformative (years) - Fundamental capability changes
    Transformative = 6,
    /// Loop 7: Transcendent (decades+) - Paradigm shifts and emergence
    Transcendent = 7,
}

impl LoopType {
    pub fn cycle_duration(&self) -> Duration {
        match self {
            LoopType::Reflexive => Duration::milliseconds(100),
            LoopType::Reactive => Duration::seconds(5),
            LoopType::Adaptive => Duration::minutes(30),
            LoopType::Deliberative => Duration::hours(24),
            LoopType::Evolutionary => Duration::days(7),
            LoopType::Transformative => Duration::days(365),
            LoopType::Transcendent => Duration::days(3650), // 10 years
        }
    }

    pub fn all_loops() -> Vec<LoopType> {
        vec![
            LoopType::Reflexive,
            LoopType::Reactive,
            LoopType::Adaptive,
            LoopType::Deliberative,
            LoopType::Evolutionary,
            LoopType::Transformative,
            LoopType::Transcendent,
        ]
    }

    pub fn description(&self) -> &str {
        match self {
            LoopType::Reflexive => "Immediate sensory-motor feedback and reflexive responses",
            LoopType::Reactive => "Quick decision-making based on current context",
            LoopType::Adaptive => "Learning from recent experiences and adapting behavior",
            LoopType::Deliberative => "Strategic planning and reflective analysis",
            LoopType::Evolutionary => "Systematic improvement through variation and selection",
            LoopType::Transformative => "Fundamental capability changes and restructuring",
            LoopType::Transcendent => "Paradigm shifts and emergent properties",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoopStatus {
    Initializing,
    Running,
    Paused,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopMetrics {
    pub cycles_completed: u64,
    pub average_cycle_time: Duration,
    pub success_rate: f64,
    pub improvement_rate: f64,
    pub stability: f64,
    pub convergence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopCycle {
    pub id: CycleId,
    pub cycle_number: u64,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub input: CycleInput,
    pub output: Option<CycleOutput>,
    pub metrics: CycleMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleInput {
    pub data: HashMap<String, serde_json::Value>,
    pub context: String,
    pub objectives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleOutput {
    pub results: HashMap<String, serde_json::Value>,
    pub insights: Vec<String>,
    pub actions: Vec<Action>,
    pub next_objectives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub action_type: ActionType,
    pub description: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub priority: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Perceive,
    Reason,
    Learn,
    Create,
    Communicate,
    Execute,
    Reflect,
    Evolve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleMetrics {
    pub duration: Duration,
    pub success: bool,
    pub quality: f64,
    pub efficiency: f64,
    pub novelty: f64,
    pub alignment: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalLoop {
    pub id: LoopId,
    pub loop_type: LoopType,
    pub name: String,
    pub description: String,
    pub status: LoopStatus,
    pub current_cycle: Option<LoopCycle>,
    pub history: Vec<CycleId>,
    pub metrics: LoopMetrics,
    pub created_at: DateTime<Utc>,
    pub last_cycle_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopCoordination {
    pub inter_loop_connections: Vec<LoopConnection>,
    pub synchronization_points: Vec<SyncPoint>,
    pub emergent_patterns: Vec<EmergentPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopConnection {
    pub from_loop: LoopType,
    pub to_loop: LoopType,
    pub connection_type: ConnectionType,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    FeedForward,  // Faster loop influences slower loop
    FeedBack,     // Slower loop influences faster loop
    Resonance,    // Mutual reinforcement
    Inhibition,   // One loop suppresses another
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPoint {
    pub loops: Vec<LoopType>,
    pub trigger_condition: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub participating_loops: Vec<LoopType>,
    pub discovered_at: DateTime<Utc>,
    pub strength: f64,
}

impl TemporalLoop {
    pub fn new(loop_type: LoopType, name: String, description: String) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            loop_type,
            name,
            description,
            status: LoopStatus::Initializing,
            current_cycle: None,
            history: Vec::new(),
            metrics: LoopMetrics {
                cycles_completed: 0,
                average_cycle_time: loop_type.cycle_duration(),
                success_rate: 0.0,
                improvement_rate: 0.0,
                stability: 0.0,
                convergence: 0.0,
            },
            created_at: Utc::now(),
            last_cycle_at: None,
        }
    }

    pub fn start_cycle(&mut self, input: CycleInput) -> CycleId {
        let cycle_id = Uuid::now_v7().to_string();
        let cycle = LoopCycle {
            id: cycle_id.clone(),
            cycle_number: self.metrics.cycles_completed + 1,
            started_at: Utc::now(),
            completed_at: None,
            input,
            output: None,
            metrics: CycleMetrics {
                duration: Duration::zero(),
                success: false,
                quality: 0.0,
                efficiency: 0.0,
                novelty: 0.0,
                alignment: 0.0,
            },
        };

        self.current_cycle = Some(cycle);
        self.status = LoopStatus::Running;
        cycle_id
    }

    pub fn complete_cycle(&mut self, output: CycleOutput, metrics: CycleMetrics) {
        if let Some(mut cycle) = self.current_cycle.take() {
            cycle.completed_at = Some(Utc::now());
            cycle.output = Some(output);

            // Check success before moving metrics
            let success = metrics.success;
            cycle.metrics = metrics;

            self.history.push(cycle.id.clone());
            self.metrics.cycles_completed += 1;
            self.last_cycle_at = cycle.completed_at;

            // Update loop metrics
            if success {
                let prev_rate = self.metrics.success_rate;
                self.metrics.success_rate =
                    (prev_rate * (self.metrics.cycles_completed - 1) as f64 + 1.0)
                        / self.metrics.cycles_completed as f64;
            }
        }
    }
}
