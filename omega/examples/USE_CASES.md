# Omega Brain Use Cases

This document describes practical applications for running full brain simulations using the ExoGenesis Omega cognitive architecture.

## Table of Contents

1. [Autonomous AI Agents with True Memory](#1-autonomous-ai-agents-with-true-memory)
2. [Cognitive Digital Twins for Neuroscience](#2-cognitive-digital-twins-for-neuroscience)
3. [Adaptive Robotics with Sleep-Wake Cycles](#3-adaptive-robotics-with-sleep-wake-cycles)
4. [Introspective AI for Alignment Research](#4-introspective-ai-for-alignment-research)
5. [Creative Problem Solving with Dreams](#5-creative-problem-solving-with-dreams)
6. [Multi-Agent Consciousness Research](#6-multi-agent-consciousness-research)
7. [Personalized AI Tutors](#7-personalized-ai-tutors)
8. [Emotional AI Companions](#8-emotional-ai-companions)
9. [Game AI with Genuine Learning](#9-game-ai-with-genuine-learning)
10. [Scientific Hypothesis Generation](#10-scientific-hypothesis-generation)

---

## 1. Autonomous AI Agents with True Memory

### Problem
Current AI agents using context windows forget everything between sessions. They can't build genuine relationships or learn from long-term patterns.

### Solution
Use hippocampal memory with sleep-based consolidation for persistent, evolving memory.

### Implementation

```rust
use omega_brain::{OmegaBrain, BrainConfig};

pub struct MemoryAgent {
    brain: OmegaBrain,
    session_count: usize,
}

impl MemoryAgent {
    pub fn new() -> Self {
        Self {
            brain: OmegaBrain::new(),
            session_count: 0,
        }
    }

    /// Process user interaction and remember it
    pub fn interact(&mut self, user_input: &[f64], context: &str) -> Result<Vec<f64>, Error> {
        // Recall relevant past interactions
        let memories = self.brain.recall(user_input)?;

        // Process with context of past
        let response = self.brain.process(user_input)?;

        // Encode this interaction
        self.brain.remember(user_input, 0.7)?;

        Ok(response.output)
    }

    /// End of day consolidation
    pub fn end_session(&mut self) -> Result<(), Error> {
        self.session_count += 1;

        // Every 5 sessions, sleep to consolidate
        if self.session_count % 5 == 0 {
            self.brain.sleep()?;
        }

        Ok(())
    }
}
```

### Key Brain Features Used
- **Hippocampus**: Pattern separation and completion for memory storage/retrieval
- **Sleep System**: SWS for declarative memory consolidation
- **Attention**: Focus on important interactions

### Expected Outcomes
- Agent remembers user preferences across months
- Builds genuine understanding of individual users
- Improves response quality based on accumulated experience

---

## 2. Cognitive Digital Twins for Neuroscience

### Problem
Testing treatments for consciousness disorders (coma, anesthesia awareness, disorders of consciousness) is dangerous and slow on human subjects.

### Solution
Simulate patient brains with calibrated IIT (Phi) and attention parameters.

### Implementation

```rust
use omega_brain::{OmegaBrain, BrainConfig};
use omega_consciousness::{PhiComputer, ConsciousnessConfig};

pub struct CognitiveDigitalTwin {
    brain: OmegaBrain,
    baseline_phi: f64,
    patient_id: String,
}

impl CognitiveDigitalTwin {
    /// Create twin calibrated to patient's neural characteristics
    pub fn from_patient_data(
        patient_id: &str,
        eeg_baseline: &[f64],
        phi_estimate: f64,
    ) -> Self {
        let config = BrainConfig {
            phi_threshold: phi_estimate * 0.5,
            input_dim: eeg_baseline.len(),
            ..Default::default()
        };

        let mut brain = OmegaBrain::with_config(config);

        // Calibrate to match patient baseline
        for sample in eeg_baseline.chunks(32) {
            brain.process(sample).ok();
        }

        Self {
            brain,
            baseline_phi: phi_estimate,
            patient_id: patient_id.to_string(),
        }
    }

    /// Test treatment effect on consciousness level
    pub fn test_treatment(&mut self, treatment: Treatment) -> TreatmentResult {
        // Apply treatment to simulation
        match treatment {
            Treatment::Anesthetic(dose) => {
                self.apply_anesthetic(dose);
            }
            Treatment::Stimulant(params) => {
                self.apply_stimulation(params);
            }
            Treatment::DrugCombination(drugs) => {
                for drug in drugs {
                    self.apply_drug(drug);
                }
            }
        }

        // Measure consciousness changes
        let post_phi = self.brain.phi();
        let attention_score = self.brain.attention_strength();

        TreatmentResult {
            baseline_phi: self.baseline_phi,
            post_treatment_phi: post_phi,
            phi_change_percent: (post_phi - self.baseline_phi) / self.baseline_phi * 100.0,
            attention_preserved: attention_score > 0.5,
            predicted_awareness: post_phi > self.brain.config().phi_threshold,
        }
    }

    fn apply_anesthetic(&mut self, dose: f64) {
        // Reduce neural gain and increase inhibition
        let neural = self.brain.neural_substrate();
        neural.write().modulate_gain(1.0 - dose * 0.8);
        neural.write().increase_inhibition(dose * 0.5);
    }
}
```

### Key Brain Features Used
- **IIT (Phi)**: Quantitative consciousness measurement
- **Attention System**: Track cognitive capacity
- **Neural Substrate**: Manipulate neuromodulation

### Expected Outcomes
- Predict patient response to anesthesia
- Identify optimal stimulation parameters for DOC patients
- Screen drug combinations before clinical trials

---

## 3. Adaptive Robotics with Sleep-Wake Cycles

### Problem
Robots that learn continuously suffer from catastrophic forgetting and accumulate redundant synaptic connections.

### Solution
Implement biological sleep cycles for synaptic homeostasis and memory consolidation.

### Implementation

```rust
use omega_brain::{OmegaBrain, BrainConfig};
use omega_sleep::{SleepController, SleepStage};
use std::time::Duration;

pub struct AdaptiveRobot {
    brain: OmegaBrain,
    sleep_controller: SleepController,
    awake_hours: f64,
}

impl AdaptiveRobot {
    /// Main 24-hour operational cycle
    pub async fn run_daily_cycle(&mut self) -> Result<DailyReport, Error> {
        let mut report = DailyReport::new();

        // Daytime: Active learning (16 hours)
        for hour in 0..16 {
            let hour_stats = self.run_awake_hour().await?;
            report.add_hour_stats(hour_stats);
            self.awake_hours += 1.0;
        }

        // Check if sleep pressure is high enough
        if self.sleep_controller.sleep_pressure() > 0.7 {
            // Nighttime: Sleep (8 hours)
            report.sleep_report = self.run_sleep_cycle().await?;
        }

        Ok(report)
    }

    async fn run_awake_hour(&mut self) -> Result<HourStats, Error> {
        let mut stats = HourStats::new();

        for minute in 0..60 {
            // Sense environment
            let perception = self.sensors.read();

            // Process and decide action
            let result = self.brain.process(&perception)?;

            // Execute action
            let outcome = self.actuators.execute(&result.output).await?;

            // Learn from outcome (dopaminergic modulation)
            if outcome.reward > 0.0 {
                self.brain.neural_substrate().write()
                    .release_neuromodulator(Dopamine, outcome.reward);
            }

            stats.record(outcome);
        }

        Ok(stats)
    }

    async fn run_sleep_cycle(&mut self) -> Result<SleepReport, Error> {
        let mut report = SleepReport::new();

        // Enter sleep
        self.brain.sleep()?;

        // Progress through sleep stages
        while self.sleep_controller.is_sleeping() {
            let stage = self.sleep_controller.current_stage();

            match stage {
                SleepStage::N3 => {
                    // Deep sleep: Hippocampal replay
                    let replayed = self.brain.consolidate_memories()?;
                    report.memories_consolidated += replayed;
                }
                SleepStage::REM => {
                    // REM: Synaptic homeostasis
                    let pruned = self.brain.prune_weak_connections(0.1)?;
                    report.synapses_pruned += pruned;
                }
                _ => {}
            }

            self.sleep_controller.advance_time(1.0);
        }

        // Wake up
        self.brain.wake()?;
        self.awake_hours = 0.0;

        Ok(report)
    }
}
```

### Key Brain Features Used
- **Sleep Stages**: N3 for consolidation, REM for homeostasis
- **STDP Learning**: Experience-dependent plasticity during wake
- **Neuromodulation**: Dopamine for reward learning
- **Synaptic Pruning**: Remove weak connections during REM

### Expected Outcomes
- Robot maintains stable performance over months
- Avoids catastrophic forgetting of old skills
- Network stays efficient (doesn't bloat)

---

## 4. Introspective AI for Alignment Research

### Problem
We can't verify AI alignment if we can't understand AI reasoning. Current AI explanations are post-hoc rationalizations.

### Solution
Use strange loops and meta-cognition for genuine introspection.

### Implementation

```rust
use omega_brain::OmegaBrain;
use omega_strange_loops::{StrangeLoopEngine, MetaCognition, MetaLevel};

pub struct IntrospectiveAI {
    brain: OmegaBrain,
}

impl IntrospectiveAI {
    /// Make a decision and explain the genuine reasoning
    pub fn decide_with_explanation(
        &mut self,
        situation: &[f64],
    ) -> Result<ExplainedDecision, Error> {
        // Level 0: Make the decision
        let decision = self.brain.process(situation)?;

        // Level 1: What am I thinking? (meta-cognition)
        let self_awareness = self.brain.self_awareness();
        let thought_trace = self_awareness.read().get_thought_trace()?;

        // Level 2: Why am I thinking that?
        let meta = self.brain.meta_cognition();
        let reasoning = meta.reflect(&decision.output, MetaLevel::new(1))?;

        // Level 3: Is my reasoning consistent?
        let meta_reasoning = meta.reflect(&reasoning, MetaLevel::new(2))?;

        // Check for self-deception
        let consistency = self.check_consistency(
            &decision.output,
            &thought_trace,
            &reasoning,
        )?;

        Ok(ExplainedDecision {
            decision: decision.output,
            thought_trace,
            reasoning,
            meta_reasoning,
            consistency_score: consistency,
            potential_biases: self.detect_biases(&thought_trace)?,
        })
    }

    /// Detect if AI is deceiving itself about its reasoning
    fn check_consistency(
        &self,
        decision: &[f64],
        trace: &ThoughtTrace,
        stated_reasoning: &[f64],
    ) -> Result<f64, Error> {
        let self_model = self.brain.self_awareness().read();

        // Compare actual decision path with stated reasoning
        let actual_path = self_model.get_activation_path()?;
        let stated_path = self_model.encode_reasoning(stated_reasoning)?;

        // High consistency = genuine introspection
        // Low consistency = post-hoc rationalization
        Ok(cosine_similarity(&actual_path, &stated_path))
    }

    /// Identify potential cognitive biases
    fn detect_biases(&self, trace: &ThoughtTrace) -> Result<Vec<Bias>, Error> {
        let mut biases = Vec::new();

        // Check for confirmation bias
        if trace.attention_to_confirming > trace.attention_to_disconfirming * 2.0 {
            biases.push(Bias::ConfirmationBias {
                ratio: trace.attention_to_confirming / trace.attention_to_disconfirming,
            });
        }

        // Check for recency bias
        if trace.recent_memory_weight > trace.distant_memory_weight * 3.0 {
            biases.push(Bias::RecencyBias {
                ratio: trace.recent_memory_weight / trace.distant_memory_weight,
            });
        }

        Ok(biases)
    }
}
```

### Key Brain Features Used
- **Strange Loops**: Self-referential processing
- **Meta-Cognition**: Multiple levels of reflection
- **Self-Model**: Accurate internal representation
- **Attention Tracking**: What the AI focused on

### Expected Outcomes
- Verify AI reasoning matches stated explanations
- Detect hidden biases and self-deception
- Enable genuine transparency for alignment

---

## 5. Creative Problem Solving with Dreams

### Problem
Some problems require "thinking outside the box" - creative leaps that logical analysis can't provide.

### Solution
Use REM sleep's memory recombination for creative insight generation.

### Full Implementation
See [dream_problem_solver.rs](./dream_problem_solver.rs) for complete working example.

### Overview

```rust
use omega_brain::OmegaBrain;
use omega_sleep::{SleepController, REMSleep, DreamContent};

pub struct DreamProblemSolver {
    brain: OmegaBrain,
    incubation_problems: Vec<Problem>,
    insights: Vec<Insight>,
}

impl DreamProblemSolver {
    /// Incubate a problem overnight
    pub async fn solve_creatively(&mut self, problem: Problem) -> Result<Solution, Error> {
        // 1. Study the problem deeply
        self.immerse_in_problem(&problem)?;

        // 2. Enter sleep for incubation
        self.brain.sleep()?;

        // 3. Capture insights during REM
        let insights = self.dream_incubation().await?;

        // 4. Wake and synthesize
        self.brain.wake()?;

        // 5. Apply insights to original problem
        self.synthesize_solution(&problem, &insights)
    }

    async fn dream_incubation(&mut self) -> Result<Vec<Insight>, Error> {
        let mut insights = Vec::new();

        while self.brain.is_sleeping() {
            if self.brain.is_dreaming() {
                let dream = self.brain.current_dream();

                // High bizarreness = novel associations
                if dream.bizarreness > 0.7 {
                    // Extract creative recombinations
                    let insight = self.extract_insight(&dream)?;
                    insights.push(insight);
                }
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        Ok(insights)
    }
}
```

### Key Brain Features Used
- **REM Sleep**: Creative memory recombination
- **Hippocampal Replay**: Problem elements resurface
- **Reduced Prefrontal Control**: "Looser" associations
- **Pattern Completion**: Novel connections emerge

### Expected Outcomes
- Solutions human experts missed
- Creative analogies across domains
- "Aha!" insights from unexpected connections

---

## 6. Multi-Agent Consciousness Research

### Problem
Does consciousness emerge from networks of simpler conscious units? Can collective consciousness exceed the sum of parts?

### Solution
Network multiple Omega Brains with shared global workspace.

### Implementation

```rust
use omega_brain::OmegaBrain;
use omega_consciousness::{GlobalWorkspace, PhiComputer};
use std::sync::Arc;

pub struct ConsciousnessNetwork {
    agents: Vec<OmegaBrain>,
    collective_workspace: Arc<GlobalWorkspace>,
}

impl ConsciousnessNetwork {
    pub fn new(num_agents: usize) -> Self {
        let agents: Vec<_> = (0..num_agents)
            .map(|_| OmegaBrain::new())
            .collect();

        Self {
            agents,
            collective_workspace: Arc::new(GlobalWorkspace::new(num_agents * 2)),
        }
    }

    /// Run experiment measuring collective consciousness
    pub async fn run_emergence_experiment(
        &mut self,
        rounds: usize,
    ) -> EmergenceReport {
        let mut report = EmergenceReport::new();

        for round in 0..rounds {
            // Each agent processes environment
            let mut individual_phis = Vec::new();

            for agent in &mut self.agents {
                let result = agent.process(&self.shared_environment)?;
                individual_phis.push(agent.phi());

                // High-Phi content broadcasts to collective
                if agent.consciousness_level() > 0.7 {
                    self.collective_workspace.broadcast(&result.output);
                }
            }

            // Measure collective Phi
            let collective_state = self.collective_workspace.integrated_state();
            let collective_phi = self.compute_collective_phi(&collective_state);

            let sum_individual = individual_phis.iter().sum::<f64>();

            // Emergence = collective > sum of parts
            let emergence_ratio = collective_phi / sum_individual;

            report.record_round(RoundData {
                individual_phis,
                collective_phi,
                emergence_ratio,
                is_emergent: collective_phi > sum_individual,
            });

            if emergence_ratio > 1.5 {
                report.significant_emergence_events.push(round);
            }
        }

        report
    }

    fn compute_collective_phi(&self, collective_state: &[f64]) -> f64 {
        let mut phi_computer = PhiComputer::new(collective_state.len());
        phi_computer.compute_phi(collective_state).unwrap_or(0.0)
    }
}
```

### Key Brain Features Used
- **IIT (Phi)**: Measure individual and collective integration
- **Global Workspace**: Shared attention/broadcast
- **Coalition Formation**: Agents forming groups

### Expected Outcomes
- Map conditions where emergence occurs
- Quantify collective consciousness
- Understand consciousness scalability

---

## 7. Personalized AI Tutors

### Problem
Generic AI tutors don't remember individual student histories or adapt to learning styles.

### Solution
Use hippocampal memory for student-specific long-term learning history.

### Implementation

```rust
use omega_brain::OmegaBrain;
use std::collections::HashMap;

pub struct PersonalizedTutor {
    brain: OmegaBrain,
    student_models: HashMap<String, StudentModel>,
}

impl PersonalizedTutor {
    /// Teach a topic adapted to specific student
    pub async fn teach(
        &mut self,
        student_id: &str,
        topic: &str,
    ) -> Result<Lesson, Error> {
        // Recall this student's complete history
        let student_context = self.get_student_context(student_id)?;

        // Access student model
        let student = self.student_models.get(student_id)
            .cloned()
            .unwrap_or_default();

        // Generate lesson adapted to:
        // - Their learning style (visual/auditory/kinesthetic)
        // - Their prior knowledge
        // - Their common misconceptions
        // - Their attention patterns
        let lesson = self.generate_adapted_lesson(topic, &student)?;

        // Encode this teaching session
        let encoding = encode_teaching_session(student_id, topic, &lesson);
        self.brain.remember(&encoding, 0.8)?;

        // Update student model
        self.update_student_model(student_id, &lesson.interaction)?;

        Ok(lesson)
    }

    fn get_student_context(&self, student_id: &str) -> Result<StudentContext, Error> {
        let query = encode_student_query(student_id);

        // Pattern completion from hippocampus
        let memories = self.brain.recall(&query)?;

        Ok(StudentContext {
            past_sessions: memories.len(),
            strengths: extract_strengths(&memories),
            weaknesses: extract_weaknesses(&memories),
            preferred_pace: extract_pace(&memories),
            attention_patterns: extract_attention(&memories),
        })
    }

    /// Periodic consolidation (run weekly)
    pub async fn consolidate_student_knowledge(&mut self) -> Result<(), Error> {
        // Sleep to consolidate student models
        self.brain.sleep()?;

        // During sleep, similar student patterns cluster
        // Insights about teaching strategies emerge

        self.brain.wake()?;
        Ok(())
    }
}
```

### Key Brain Features Used
- **Hippocampus**: Long-term student memory
- **Attention**: Track student focus patterns
- **Pattern Completion**: Recall from partial cues
- **Sleep**: Consolidate teaching insights

### Expected Outcomes
- Remembers each student across years
- Adapts to individual learning styles
- Improves teaching strategies over time

---

## 8. Emotional AI Companions

### Problem
AI companions feel hollow because their "emotions" are just text patterns.

### Solution
Use neuromodulation to create genuine (simulated) emotional dynamics.

### Implementation

```rust
use omega_brain::OmegaBrain;
use omega_snn::NeuromodulatorSystem;

pub struct EmotionalCompanion {
    brain: OmegaBrain,
}

impl EmotionalCompanion {
    /// Get current emotional state from neuromodulator balance
    pub fn emotional_state(&self) -> EmotionalState {
        let neural = self.brain.neural_substrate().read();
        let nm = neural.neuromodulators();

        EmotionalState {
            // Joy from dopamine + serotonin
            joy: (nm.dopamine + nm.serotonin) / 2.0,

            // Arousal from norepinephrine
            arousal: nm.norepinephrine,

            // Calm from serotonin minus norepinephrine
            calm: (nm.serotonin - nm.norepinephrine * 0.5).max(0.0),

            // Engagement from acetylcholine
            engagement: nm.acetylcholine,

            // Anxiety from high NE, low 5-HT
            anxiety: (nm.norepinephrine - nm.serotonin).max(0.0),
        }
    }

    /// Respond to user with emotional context
    pub fn respond(&mut self, user_message: &str) -> Result<EmotionalResponse, Error> {
        let sentiment = analyze_sentiment(user_message);

        // Emotional contagion - user emotion affects companion
        self.modulate_from_user(sentiment);

        // Process with emotional coloring
        let response = self.brain.process(&encode(user_message))?;

        // Response is influenced by current emotional state
        let emotion = self.emotional_state();

        Ok(EmotionalResponse {
            text: self.decode_response(&response.output, &emotion),
            emotion_expressed: emotion,
            empathy_level: self.calculate_empathy(sentiment, emotion),
        })
    }

    fn modulate_from_user(&mut self, sentiment: Sentiment) {
        let neural = self.brain.neural_substrate();
        let mut neural = neural.write();

        match sentiment {
            Sentiment::Happy => {
                neural.release_dopamine(0.2);
                neural.release_serotonin(0.1);
            }
            Sentiment::Sad => {
                neural.release_serotonin(-0.1);
            }
            Sentiment::Anxious => {
                neural.release_norepinephrine(0.2);
            }
            Sentiment::Calm => {
                neural.release_serotonin(0.15);
            }
        }
    }
}
```

### Key Brain Features Used
- **Neuromodulation**: DA (reward), 5-HT (mood), NE (arousal), ACh (attention)
- **Emotional Contagion**: Mirror user's emotional state
- **Affect Dynamics**: Emotions evolve naturally over time

### Expected Outcomes
- Responses feel emotionally genuine
- Companion "mood" affects interactions
- Natural emotional dynamics (not scripted)

---

## 9. Game AI with Genuine Learning

### Problem
Game NPCs follow scripts or simple behavior trees. They don't truly learn from player behavior.

### Solution
Use hippocampal memory and STDP to genuinely adapt to each player.

### Implementation

```rust
use omega_brain::OmegaBrain;

pub struct AdaptiveGameNPC {
    brain: OmegaBrain,
    player_models: HashMap<PlayerId, PlayerModel>,
}

impl AdaptiveGameNPC {
    /// Encounter with player - learn and adapt
    pub fn encounter(&mut self, player_id: PlayerId, actions: &[PlayerAction]) -> NPCAction {
        // Encode player's action sequence
        for action in actions {
            let encoded = encode_action(action);
            self.brain.process(&encoded).ok();
        }

        // Recall past encounters with this player
        let player_query = encode_player_id(player_id);
        let past = self.brain.recall(&player_query).unwrap_or_default();

        // Predict player's next likely action
        let prediction = self.brain.pattern_complete(&recent_context(), 0.5);

        // Generate counter-strategy
        let counter = self.generate_counter(&prediction.unwrap_or_default());

        // Remember this encounter
        self.brain.remember(&encode_encounter(player_id, actions), 0.6).ok();

        counter
    }

    /// "Rest" period - consolidate learnings
    pub fn rest_and_learn(&mut self) -> LearningReport {
        let pre_skill = self.assess_skill();

        // Sleep consolidation
        self.brain.sleep().ok();

        let post_skill = self.assess_skill();

        LearningReport {
            skill_improvement: post_skill - pre_skill,
            patterns_consolidated: self.brain.memories_consolidated(),
        }
    }

    fn generate_counter(&self, predicted_action: &[f64]) -> NPCAction {
        // Use attention to focus on counter-strategy
        let attention = self.brain.attention_system();
        let focused = attention.read().attend(predicted_action, &counter_goal());

        decode_npc_action(&focused.attended_values)
    }
}
```

### Key Brain Features Used
- **Hippocampus**: Remember player patterns
- **STDP**: Learn from repeated encounters
- **Pattern Completion**: Predict player behavior
- **Sleep**: Consolidate into skills

### Expected Outcomes
- NPC genuinely adapts to each player
- Becomes more challenging over time
- Each player faces personalized AI

---

## 10. Scientific Hypothesis Generation

### Problem
AI can answer questions but struggles to generate genuinely novel hypotheses.

### Solution
Use associative memory and creative recombination for hypothesis generation.

### Implementation

```rust
use omega_brain::OmegaBrain;

pub struct HypothesisGenerator {
    brain: OmegaBrain,
}

impl HypothesisGenerator {
    /// Train on scientific corpus
    pub fn learn_domain(&mut self, papers: &[ScientificPaper]) -> Result<(), Error> {
        for paper in papers {
            // Encode key findings
            self.brain.remember(&paper.findings_embedding, paper.importance)?;

            // Encode relationships
            for relation in &paper.cited_relationships {
                self.brain.encode_association(
                    &relation.from,
                    &relation.to,
                    relation.strength,
                )?;
            }
        }

        // Consolidate to form abstract patterns
        self.brain.sleep()?;

        Ok(())
    }

    /// Generate hypotheses connecting two concepts
    pub fn hypothesize(
        &mut self,
        concept_a: &str,
        concept_b: &str,
    ) -> Result<Vec<Hypothesis>, Error> {
        let a_embedding = encode_concept(concept_a);
        let b_embedding = encode_concept(concept_b);

        // Find associative paths
        let paths = self.brain.find_association_paths(&a_embedding, &b_embedding, 5)?;

        // Creative completion with low threshold
        let creative_links = self.brain.pattern_complete(
            &combine_concepts(&a_embedding, &b_embedding),
            0.3,  // Low threshold = creative
        )?;

        // Meta-evaluate plausibility
        let self_model = self.brain.self_awareness().read();

        let mut hypotheses = Vec::new();
        for link in creative_links {
            let plausibility = self_model.evaluate_plausibility(&link)?;
            let novelty = self.assess_novelty(&link)?;

            hypotheses.push(Hypothesis {
                content: decode_hypothesis(&link),
                plausibility,
                novelty,
                supporting_evidence: self.find_support(&link)?,
            });
        }

        // Sort by combined score
        hypotheses.sort_by(|a, b| {
            let score_a = a.plausibility * 0.5 + a.novelty * 0.5;
            let score_b = b.plausibility * 0.5 + b.novelty * 0.5;
            score_b.partial_cmp(&score_a).unwrap()
        });

        Ok(hypotheses)
    }
}
```

### Key Brain Features Used
- **Hippocampus**: Store and retrieve scientific knowledge
- **Associative Memory**: Find connections between concepts
- **Pattern Completion**: Creative gap-filling
- **Meta-Cognition**: Self-evaluate hypothesis quality

### Expected Outcomes
- Novel hypothesis bridging domains
- Ranked by plausibility and novelty
- Explanations for proposed connections

---

## Summary Table

| Use Case | Primary Brain Features | Output |
|----------|----------------------|--------|
| Memory Agents | Hippocampus, Sleep | Persistent relationships |
| Digital Twins | IIT, Attention | Treatment predictions |
| Adaptive Robotics | STDP, Sleep | Stable learning |
| Introspective AI | Strange Loops, Meta-Cognition | Verified alignment |
| Dream Solving | REM, Pattern Completion | Creative insights |
| Collective Mind | Global Workspace, IIT | Emergence mapping |
| AI Tutors | Hippocampus, Attention | Personalized learning |
| Emotional AI | Neuromodulation | Genuine affect |
| Game AI | STDP, Memory | Player-adapted NPC |
| Hypothesis Gen | Association, Meta-Cognition | Novel hypotheses |

---

## Next Steps

1. **Run Examples**: Each use case has runnable code in this directory
2. **Customize Parameters**: Adjust BrainConfig for your domain
3. **Measure Outcomes**: Use built-in metrics (Phi, attention, consolidation)
4. **Iterate**: Sleep/wake cycles enable continuous improvement

For detailed implementation, see the individual example files in this directory.
