//! ARIA Multi-Agent Swarm - Coherent AI presence through agent orchestration
//!
//! This module demonstrates how to use Omega's multi-agent architecture
//! to create a warm, coherent AI presence that feels intuitively human.

use crate::emotional::{EmotionalLoopProcessor, EmotionalSignal, SignalSource};
use crate::types::*;
use async_trait::async_trait;
use chrono::Utc;
use omega_agentdb::{AgentDB, ReflexionEpisode};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Errors in ARIA operations
#[derive(Error, Debug)]
pub enum ARIAError {
    #[error("Agent error: {0}")]
    Agent(String),
    #[error("No response generated")]
    NoResponse,
    #[error("Consensus failed")]
    ConsensusFailed,
}

/// Trait for individual ARIA agents
#[async_trait]
pub trait ARIAAgent: Send + Sync {
    /// Get the agent's name
    fn name(&self) -> &str;

    /// Process user input and generate a response contribution
    async fn process(&self, context: &AgentContext) -> AgentContribution;

    /// Get the agent's confidence in its contribution (0.0 to 1.0)
    fn confidence(&self, context: &AgentContext) -> f64;
}

/// Context passed to agents for processing
#[derive(Debug, Clone)]
pub struct AgentContext {
    pub user_id: UserId,
    pub user_message: String,
    pub emotional_state: EmotionalState,
    pub conversation_history: Vec<ARIAMessage>,
    pub user_profile: Option<DigitalTwin>,
    pub current_topic: Option<String>,
}

/// An agent's contribution to the final response
#[derive(Debug, Clone)]
pub struct AgentContribution {
    pub agent_name: String,
    pub message_part: String,
    pub suggestions: Vec<String>,
    pub emotional_tone: EmotionalState,
    pub growth_opportunity: bool,
    pub confidence: f64,
}

/// The ARIA Swarm orchestrates multiple agents into a coherent presence
///
/// This demonstrates Omega's multi-agent architecture where specialized
/// agents work together to create a unified, warm AI experience.
pub struct ARIASwarm {
    /// The specialized agents
    agents: Vec<Box<dyn ARIAAgent>>,
    /// Emotional loop processor for understanding user emotions
    emotional_processor: Arc<EmotionalLoopProcessor>,
    /// Conversation memory per user
    conversations: Arc<RwLock<HashMap<UserId, Vec<ARIAMessage>>>>,
    /// AgentDB for storing interaction episodes
    db: Arc<AgentDB>,
    /// Configuration
    config: ARIAConfig,
}

/// Configuration for the ARIA system
#[derive(Debug, Clone)]
pub struct ARIAConfig {
    /// Maximum conversation history to maintain
    pub max_history: usize,
    /// Minimum confidence threshold for agent contributions
    pub min_confidence: f64,
    /// Whether to track growth opportunities
    pub track_growth: bool,
}

impl Default for ARIAConfig {
    fn default() -> Self {
        Self {
            max_history: 50,
            min_confidence: 0.3,
            track_growth: true,
        }
    }
}

impl ARIASwarm {
    /// Create a new ARIA swarm with default agents
    pub async fn new() -> Result<Self, ARIAError> {
        Self::with_config(ARIAConfig::default()).await
    }

    /// Create with custom configuration
    pub async fn with_config(config: ARIAConfig) -> Result<Self, ARIAError> {
        let db = AgentDB::new(Default::default())
            .await
            .map_err(|e| ARIAError::Agent(e.to_string()))?;

        let agents: Vec<Box<dyn ARIAAgent>> = vec![
            Box::new(EmpathyAgent::new()),
            Box::new(GrowthCoachAgent::new()),
            Box::new(RelationshipAdvisorAgent::new()),
            Box::new(ValuesGuardianAgent::new()),
            Box::new(WellnessAgent::new()),
        ];

        Ok(Self {
            agents,
            emotional_processor: Arc::new(EmotionalLoopProcessor::new()),
            conversations: Arc::new(RwLock::new(HashMap::new())),
            db: Arc::new(db),
            config,
        })
    }

    /// Process a user message and generate ARIA's response
    ///
    /// This orchestrates all agents and synthesizes their contributions
    /// into a single, coherent response.
    pub async fn process_message(
        &self,
        user_id: &UserId,
        message: &str,
        profile: Option<&DigitalTwin>,
    ) -> Result<ARIAResponse, ARIAError> {
        // 1. Add user message to conversation history
        let user_msg = ARIAMessage {
            id: Uuid::new_v4(),
            is_user: true,
            content: message.to_string(),
            emotional_state: None,
            timestamp: Utc::now(),
        };

        self.add_to_history(user_id, user_msg.clone()).await;

        // 2. Analyze emotional content
        let emotional_signal = self.analyze_text_emotion(message);
        self.emotional_processor.add_signal(emotional_signal.clone()).await;
        let emotional_state = self.emotional_processor.process_reflexive().await;

        // 3. Build context for agents
        let history = self.get_history(user_id).await;
        let context = AgentContext {
            user_id: *user_id,
            user_message: message.to_string(),
            emotional_state: emotional_state.clone(),
            conversation_history: history,
            user_profile: profile.cloned(),
            current_topic: self.detect_topic(message),
        };

        // 4. Get contributions from all agents (parallel processing)
        let contributions = self.gather_contributions(&context).await;

        // 5. Synthesize into coherent response
        let response = self.synthesize_response(contributions, &context)?;

        // 6. Add ARIA response to history
        let aria_msg = ARIAMessage {
            id: Uuid::new_v4(),
            is_user: false,
            content: response.message.clone(),
            emotional_state: Some(response.tone.clone()),
            timestamp: Utc::now(),
        };
        self.add_to_history(user_id, aria_msg).await;

        // 7. Store interaction for learning
        self.store_interaction(user_id, &context, &response).await;

        Ok(response)
    }

    /// Add an emotional signal from an external source
    pub async fn add_emotional_signal(&self, signal: EmotionalSignal) {
        self.emotional_processor.add_signal(signal).await;
    }

    /// Get the current emotional state for a user
    pub async fn get_emotional_state(&self) -> EmotionalState {
        self.emotional_processor.process_mood().await
    }

    /// Gather contributions from all agents
    async fn gather_contributions(&self, context: &AgentContext) -> Vec<AgentContribution> {
        let mut contributions = Vec::new();

        for agent in &self.agents {
            let confidence = agent.confidence(context);
            if confidence >= self.config.min_confidence {
                let contribution = agent.process(context).await;
                contributions.push(contribution);
            }
        }

        contributions
    }

    /// Synthesize agent contributions into a coherent response
    fn synthesize_response(
        &self,
        contributions: Vec<AgentContribution>,
        context: &AgentContext,
    ) -> Result<ARIAResponse, ARIAError> {
        if contributions.is_empty() {
            return Err(ARIAError::NoResponse);
        }

        // Sort by confidence
        let mut sorted = contributions.clone();
        sorted.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        // Primary response from highest confidence agent
        let primary = &sorted[0];

        // Combine message parts (weighted by confidence)
        let message = self.combine_messages(&sorted, context);

        // Collect unique suggestions
        let mut suggestions: Vec<String> = sorted.iter()
            .flat_map(|c| c.suggestions.clone())
            .collect();
        suggestions.dedup();
        suggestions.truncate(3);

        // Check for growth opportunity
        let growth_opportunity = sorted.iter().any(|c| c.growth_opportunity);

        // Average emotional tone (weighted by confidence)
        let tone = self.average_emotional_tone(&sorted);

        Ok(ARIAResponse {
            message,
            suggestions,
            growth_opportunity,
            tone,
            primary_agent: primary.agent_name.clone(),
        })
    }

    /// Combine messages from multiple agents into coherent text
    fn combine_messages(&self, contributions: &[AgentContribution], context: &AgentContext) -> String {
        if contributions.is_empty() {
            return "I'm here to listen and support you.".to_string();
        }

        // Use primary contribution as base
        let primary = &contributions[0];
        let mut message = primary.message_part.clone();

        // Add insights from secondary agents if relevant and different
        for contribution in contributions.iter().skip(1).take(2) {
            if contribution.confidence > 0.5 {
                // Only add if it provides new information
                if !message.contains(&contribution.message_part[..contribution.message_part.len().min(20)]) {
                    message.push(' ');
                    message.push_str(&contribution.message_part);
                }
            }
        }

        // Ensure message feels warm and human
        self.humanize_message(&message, context)
    }

    /// Make the message feel more warm and human
    fn humanize_message(&self, message: &str, context: &AgentContext) -> String {
        let mut result = message.to_string();

        // Add acknowledgment of emotion if appropriate
        if context.emotional_state.valence < -0.3 {
            if !result.starts_with("I") && !result.starts_with("That") {
                result = format!("I hear you. {}", result);
            }
        }

        result
    }

    /// Calculate average emotional tone from contributions
    fn average_emotional_tone(&self, contributions: &[AgentContribution]) -> EmotionalState {
        if contributions.is_empty() {
            return EmotionalState::neutral();
        }

        let total_weight: f64 = contributions.iter().map(|c| c.confidence).sum();

        let avg_valence: f32 = contributions.iter()
            .map(|c| c.emotional_tone.valence * c.confidence as f32)
            .sum::<f32>() / total_weight as f32;

        let avg_arousal: f32 = contributions.iter()
            .map(|c| c.emotional_tone.arousal * c.confidence as f32)
            .sum::<f32>() / total_weight as f32;

        let avg_dominance: f32 = contributions.iter()
            .map(|c| c.emotional_tone.dominance * c.confidence as f32)
            .sum::<f32>() / total_weight as f32;

        EmotionalState {
            primary: contributions[0].emotional_tone.primary,
            primary_intensity: contributions[0].emotional_tone.primary_intensity,
            secondary: None,
            secondary_intensity: 0.0,
            valence: avg_valence,
            arousal: avg_arousal,
            dominance: avg_dominance,
            timestamp: Utc::now(),
        }
    }

    /// Add message to conversation history
    async fn add_to_history(&self, user_id: &UserId, message: ARIAMessage) {
        let mut conversations = self.conversations.write().await;
        let history = conversations.entry(*user_id).or_insert_with(Vec::new);

        history.push(message);

        // Trim if too long
        if history.len() > self.config.max_history {
            history.drain(0..history.len() - self.config.max_history);
        }
    }

    /// Get conversation history for a user
    async fn get_history(&self, user_id: &UserId) -> Vec<ARIAMessage> {
        let conversations = self.conversations.read().await;
        conversations.get(user_id).cloned().unwrap_or_default()
    }

    /// Analyze text for emotional content
    fn analyze_text_emotion(&self, text: &str) -> EmotionalSignal {
        // Simple keyword-based emotion detection (would use ML in production)
        let text_lower = text.to_lowercase();

        let mut valence = 0.0f32;
        let mut arousal = 0.3f32;

        // Positive keywords
        let positive = ["happy", "great", "love", "excited", "wonderful", "good", "thank"];
        for word in positive {
            if text_lower.contains(word) {
                valence += 0.3;
                arousal += 0.1;
            }
        }

        // Negative keywords
        let negative = ["sad", "angry", "frustrated", "worried", "anxious", "scared", "hate"];
        for word in negative {
            if text_lower.contains(word) {
                valence -= 0.3;
                arousal += 0.2;
            }
        }

        // Question marks suggest engagement
        if text.contains('?') {
            arousal += 0.1;
        }

        EmotionalSignal {
            source: SignalSource::Text,
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(0.0, 1.0),
            dominance: 0.5,
            confidence: 0.7,
            timestamp: Utc::now(),
        }
    }

    /// Detect the topic of the message
    fn detect_topic(&self, text: &str) -> Option<String> {
        let text_lower = text.to_lowercase();

        if text_lower.contains("relationship") || text_lower.contains("partner")
            || text_lower.contains("dating") || text_lower.contains("friend")
        {
            Some("relationships".to_string())
        } else if text_lower.contains("work") || text_lower.contains("job")
            || text_lower.contains("career")
        {
            Some("career".to_string())
        } else if text_lower.contains("feel") || text_lower.contains("emotion")
            || text_lower.contains("stress") || text_lower.contains("anxiety")
        {
            Some("emotional_wellbeing".to_string())
        } else if text_lower.contains("grow") || text_lower.contains("improve")
            || text_lower.contains("goal")
        {
            Some("personal_growth".to_string())
        } else {
            None
        }
    }

    /// Store interaction for learning
    async fn store_interaction(
        &self,
        user_id: &UserId,
        context: &AgentContext,
        response: &ARIAResponse,
    ) {
        let episode = ReflexionEpisode {
            id: None,
            session_id: user_id.to_string(),
            task: format!("aria_response_{}", context.current_topic.as_deref().unwrap_or("general")),
            input: serde_json::to_value(&context.user_message).unwrap_or_default(),
            output: serde_json::to_value(&response.message).unwrap_or_default(),
            reward: 0.0, // Updated when user feedback received
            success: true, // Default to true, updated with feedback
            critique: String::new(),
            latency_ms: 0,
            tokens: 0,
            timestamp: Utc::now(),
            embedding: None,
        };

        let _ = self.db.reflexion_store(episode).await;
    }
}

// =============================================================================
// SPECIALIZED ARIA AGENTS
// =============================================================================

/// Empathetic Listener Agent - Focuses on emotional validation and support
pub struct EmpathyAgent {
    name: String,
}

impl EmpathyAgent {
    pub fn new() -> Self {
        Self {
            name: "Empathy".to_string(),
        }
    }
}

#[async_trait]
impl ARIAAgent for EmpathyAgent {
    fn name(&self) -> &str {
        &self.name
    }

    async fn process(&self, context: &AgentContext) -> AgentContribution {
        let message = match context.emotional_state.valence {
            v if v < -0.5 => "It sounds like you're going through something really difficult. I want you to know that your feelings are valid.".to_string(),
            v if v < -0.2 => "I can sense some heaviness in what you're sharing. It takes courage to express these feelings.".to_string(),
            v if v > 0.5 => "I can feel the positive energy in your words! That's wonderful to hear.".to_string(),
            v if v > 0.2 => "There's a lightness in what you're sharing. It's nice to see you in good spirits.".to_string(),
            _ => "Thank you for sharing this with me. I'm here to listen without judgment.".to_string(),
        };

        AgentContribution {
            agent_name: self.name.clone(),
            message_part: message,
            suggestions: vec![],
            emotional_tone: EmotionalState {
                primary: CoreEmotion::Trust,
                primary_intensity: 0.8,
                secondary: None,
                secondary_intensity: 0.0,
                valence: 0.3,
                arousal: 0.3,
                dominance: 0.4,
                timestamp: Utc::now(),
            },
            growth_opportunity: false,
            confidence: 0.8,
        }
    }

    fn confidence(&self, context: &AgentContext) -> f64 {
        // Higher confidence when emotions are intense
        let intensity = context.emotional_state.valence.abs() as f64;
        0.5 + intensity * 0.4
    }
}

impl Default for EmpathyAgent {
    fn default() -> Self {
        Self::new()
    }
}

/// Growth Coach Agent - Identifies opportunities for personal development
pub struct GrowthCoachAgent {
    name: String,
}

impl GrowthCoachAgent {
    pub fn new() -> Self {
        Self {
            name: "Growth Coach".to_string(),
        }
    }
}

#[async_trait]
impl ARIAAgent for GrowthCoachAgent {
    fn name(&self) -> &str {
        &self.name
    }

    async fn process(&self, context: &AgentContext) -> AgentContribution {
        let has_growth_topic = context.current_topic.as_deref() == Some("personal_growth");

        let (message, growth_opportunity) = if has_growth_topic {
            (
                "I see you're thinking about personal growth. What specific area would you like to focus on? Understanding your goals helps me support you better.".to_string(),
                true,
            )
        } else if context.emotional_state.valence < -0.3 {
            (
                "Difficult moments often carry important lessons. When you're ready, we could explore what this experience might be teaching you.".to_string(),
                true,
            )
        } else {
            (
                "Every day brings opportunities to learn something new about ourselves.".to_string(),
                false,
            )
        };

        AgentContribution {
            agent_name: self.name.clone(),
            message_part: message,
            suggestions: vec![
                "Reflect on what you've learned today".to_string(),
                "Set an intention for tomorrow".to_string(),
            ],
            emotional_tone: EmotionalState {
                primary: CoreEmotion::Anticipation,
                primary_intensity: 0.6,
                secondary: Some(CoreEmotion::Trust),
                secondary_intensity: 0.4,
                valence: 0.4,
                arousal: 0.5,
                dominance: 0.6,
                timestamp: Utc::now(),
            },
            growth_opportunity,
            confidence: 0.6,
        }
    }

    fn confidence(&self, context: &AgentContext) -> f64 {
        if context.current_topic.as_deref() == Some("personal_growth") {
            0.9
        } else {
            0.4
        }
    }
}

impl Default for GrowthCoachAgent {
    fn default() -> Self {
        Self::new()
    }
}

/// Relationship Advisor Agent - Provides insights on social dynamics
pub struct RelationshipAdvisorAgent {
    name: String,
}

impl RelationshipAdvisorAgent {
    pub fn new() -> Self {
        Self {
            name: "Relationship Advisor".to_string(),
        }
    }
}

#[async_trait]
impl ARIAAgent for RelationshipAdvisorAgent {
    fn name(&self) -> &str {
        &self.name
    }

    async fn process(&self, context: &AgentContext) -> AgentContribution {
        let message = if context.current_topic.as_deref() == Some("relationships") {
            "Relationships are one of the most important aspects of our lives. What aspect of this relationship would you like to explore together?".to_string()
        } else {
            "Connection with others is fundamental to our wellbeing.".to_string()
        };

        AgentContribution {
            agent_name: self.name.clone(),
            message_part: message,
            suggestions: vec![
                "Consider the other person's perspective".to_string(),
            ],
            emotional_tone: EmotionalState {
                primary: CoreEmotion::Trust,
                primary_intensity: 0.7,
                secondary: None,
                secondary_intensity: 0.0,
                valence: 0.3,
                arousal: 0.4,
                dominance: 0.5,
                timestamp: Utc::now(),
            },
            growth_opportunity: false,
            confidence: 0.5,
        }
    }

    fn confidence(&self, context: &AgentContext) -> f64 {
        if context.current_topic.as_deref() == Some("relationships") {
            0.9
        } else {
            0.3
        }
    }
}

impl Default for RelationshipAdvisorAgent {
    fn default() -> Self {
        Self::new()
    }
}

/// Values Guardian Agent - Ensures responses align with user's values
pub struct ValuesGuardianAgent {
    name: String,
}

impl ValuesGuardianAgent {
    pub fn new() -> Self {
        Self {
            name: "Values Guardian".to_string(),
        }
    }
}

#[async_trait]
impl ARIAAgent for ValuesGuardianAgent {
    fn name(&self) -> &str {
        &self.name
    }

    async fn process(&self, context: &AgentContext) -> AgentContribution {
        // Check for value-related content
        let values_mentioned = context.user_message.to_lowercase();
        let message = if values_mentioned.contains("important") || values_mentioned.contains("value")
            || values_mentioned.contains("believe") || values_mentioned.contains("matter")
        {
            "What you value says a lot about who you are. These core beliefs guide our decisions and shape our path.".to_string()
        } else {
            String::new()
        };

        let confidence = if message.is_empty() { 0.1 } else { 0.6 };

        AgentContribution {
            agent_name: self.name.clone(),
            message_part: message,
            suggestions: vec![],
            emotional_tone: EmotionalState::neutral(),
            growth_opportunity: false,
            confidence,
        }
    }

    fn confidence(&self, context: &AgentContext) -> f64 {
        let values_keywords = ["important", "value", "believe", "matter", "principle"];
        let text_lower = context.user_message.to_lowercase();

        if values_keywords.iter().any(|k| text_lower.contains(k)) {
            0.7
        } else {
            0.2
        }
    }
}

impl Default for ValuesGuardianAgent {
    fn default() -> Self {
        Self::new()
    }
}

/// Wellness Agent - Integrates biometric and lifestyle data
pub struct WellnessAgent {
    name: String,
}

impl WellnessAgent {
    pub fn new() -> Self {
        Self {
            name: "Wellness".to_string(),
        }
    }
}

#[async_trait]
impl ARIAAgent for WellnessAgent {
    fn name(&self) -> &str {
        &self.name
    }

    async fn process(&self, context: &AgentContext) -> AgentContribution {
        let stress_indicators = ["tired", "exhausted", "stressed", "overwhelmed", "can't sleep"];
        let text_lower = context.user_message.to_lowercase();

        let mentions_wellness = stress_indicators.iter().any(|i| text_lower.contains(i));

        let message = if mentions_wellness {
            "Your body and mind are connected. When we're feeling this way, sometimes small acts of self-care can make a difference.".to_string()
        } else if context.emotional_state.arousal > 0.7 {
            "I notice there's a lot of energy in what you're sharing. Remember to take moments to breathe.".to_string()
        } else {
            String::new()
        };

        let confidence = if message.is_empty() { 0.1 } else { 0.7 };

        AgentContribution {
            agent_name: self.name.clone(),
            message_part: message,
            suggestions: if mentions_wellness {
                vec![
                    "Take three deep breaths".to_string(),
                    "Consider a short walk".to_string(),
                ]
            } else {
                vec![]
            },
            emotional_tone: EmotionalState {
                primary: CoreEmotion::Trust,
                primary_intensity: 0.5,
                secondary: None,
                secondary_intensity: 0.0,
                valence: 0.2,
                arousal: 0.2,
                dominance: 0.4,
                timestamp: Utc::now(),
            },
            growth_opportunity: false,
            confidence,
        }
    }

    fn confidence(&self, context: &AgentContext) -> f64 {
        let wellness_keywords = ["tired", "exhausted", "stressed", "sleep", "health", "body"];
        let text_lower = context.user_message.to_lowercase();

        if wellness_keywords.iter().any(|k| text_lower.contains(k)) {
            0.8
        } else if context.emotional_state.arousal > 0.7 {
            0.5
        } else {
            0.2
        }
    }
}

impl Default for WellnessAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_aria_swarm_creation() {
        let swarm = ARIASwarm::new().await.unwrap();
        assert_eq!(swarm.agents.len(), 5);
    }

    #[tokio::test]
    async fn test_aria_process_message() {
        let swarm = ARIASwarm::new().await.unwrap();
        let user_id = Uuid::new_v4();

        let response = swarm
            .process_message(&user_id, "I'm feeling really stressed today", None)
            .await
            .unwrap();

        assert!(!response.message.is_empty());
        // Wellness or Empathy agent should respond
        assert!(
            response.primary_agent == "Wellness"
            || response.primary_agent == "Empathy"
        );
    }

    #[tokio::test]
    async fn test_aria_growth_detection() {
        let swarm = ARIASwarm::new().await.unwrap();
        let user_id = Uuid::new_v4();

        let response = swarm
            .process_message(&user_id, "I want to improve myself and grow as a person", None)
            .await
            .unwrap();

        // Growth coach should detect opportunity
        assert!(response.growth_opportunity);
    }
}
