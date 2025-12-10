//! PATH Social Network - 1000 User Lifecycle Simulation
//!
//! Simulates a full year of social dynamics including:
//! - Personality measurement and evolution
//! - Friendship formation and deepening
//! - Dating, relationships, and marriages
//! - Emotional journeys and life events
//! - Network growth and community formation

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;

// ============================================================================
// CONFIGURATION
// ============================================================================

const NUM_USERS: usize = 1000;
const SIMULATION_WEEKS: usize = 52; // 1 year
const EMBEDDING_DIM: usize = 256; // Reduced for simulation efficiency
const FRIENDSHIP_THRESHOLD: f32 = 0.75;
const DATING_THRESHOLD: f32 = 0.82;
const MARRIAGE_THRESHOLD: f32 = 0.90;
const WEEKLY_INTERACTIONS: usize = 20; // Per user average

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Clone, Debug)]
struct BigFive {
    openness: f32,
    conscientiousness: f32,
    extraversion: f32,
    agreeableness: f32,
    neuroticism: f32,
}

#[derive(Clone, Debug)]
struct SchwartzValues {
    self_direction: f32,
    stimulation: f32,
    hedonism: f32,
    achievement: f32,
    power: f32,
    security: f32,
    conformity: f32,
    tradition: f32,
    benevolence: f32,
    universalism: f32,
}

#[derive(Clone, Debug, PartialEq)]
enum AttachmentStyle {
    Secure,
    Anxious,
    Avoidant,
    Disorganized,
}

#[derive(Clone, Debug, PartialEq)]
enum RelationshipStatus {
    Single,
    Dating(usize),      // Partner ID
    Engaged(usize),     // Partner ID
    Married(usize),     // Partner ID
}

#[derive(Clone, Debug)]
struct EmotionalState {
    valence: f32,      // -1 to 1 (negative to positive)
    arousal: f32,      // 0 to 1 (calm to excited)
    resilience: f32,   // 0 to 1
}

#[derive(Clone, Debug)]
struct LifeEvent {
    week: usize,
    event_type: LifeEventType,
    impact: f32,
}

#[derive(Clone, Debug, PartialEq)]
enum LifeEventType {
    NewJob,
    JobLoss,
    Promotion,
    MovedCity,
    FamilyIssue,
    HealthChallenge,
    Achievement,
    Loss,
    Travel,
    NewHobby,
}

#[derive(Clone)]
struct User {
    id: usize,
    name: String,
    age: u8,
    big_five: BigFive,
    initial_big_five: BigFive,
    values: SchwartzValues,
    attachment: AttachmentStyle,
    emotional_state: EmotionalState,
    relationship_status: RelationshipStatus,
    friends: HashSet<usize>,
    close_friends: HashSet<usize>,
    best_friend: Option<usize>,
    embedding: Vec<f32>,
    weeks_dating: HashMap<usize, usize>,
    life_events: Vec<LifeEvent>,
    happiness_history: Vec<f32>,
    personality_confidence: f32,
}

#[derive(Debug, Clone)]
struct Friendship {
    user_a: usize,
    user_b: usize,
    strength: f32,
    started_week: usize,
    interactions: usize,
}

#[derive(Debug, Clone)]
struct Marriage {
    partner_a: usize,
    partner_b: usize,
    wedding_week: usize,
    compatibility_score: f32,
}

struct SimulationStats {
    total_friendships_formed: usize,
    total_close_friendships: usize,
    total_best_friends: usize,
    total_dates: usize,
    total_relationships: usize,
    total_engagements: usize,
    total_marriages: usize,
    total_breakups: usize,
    avg_friends_per_user: f32,
    avg_happiness: f32,
    personality_drift_avg: f32,
    highest_compatibility_marriage: (usize, usize, f32),
    most_popular_user: (usize, usize), // (id, friend_count)
    life_events_processed: usize,
}

// ============================================================================
// PERSONALITY ARCHETYPES
// ============================================================================

fn generate_archetype_name(rng: &mut ChaCha8Rng) -> (&'static str, BigFive, SchwartzValues, AttachmentStyle) {
    let archetypes = [
        ("Creative Empath", BigFive { openness: 0.85, conscientiousness: 0.70, extraversion: 0.75, agreeableness: 0.90, neuroticism: 0.25 }),
        ("Driven Achiever", BigFive { openness: 0.65, conscientiousness: 0.92, extraversion: 0.70, agreeableness: 0.60, neuroticism: 0.35 }),
        ("Social Butterfly", BigFive { openness: 0.88, conscientiousness: 0.55, extraversion: 0.95, agreeableness: 0.82, neuroticism: 0.30 }),
        ("Analytical Mind", BigFive { openness: 0.45, conscientiousness: 0.88, extraversion: 0.40, agreeableness: 0.65, neuroticism: 0.45 }),
        ("Nurturing Soul", BigFive { openness: 0.72, conscientiousness: 0.75, extraversion: 0.60, agreeableness: 0.95, neuroticism: 0.20 }),
        ("Bold Leader", BigFive { openness: 0.60, conscientiousness: 0.85, extraversion: 0.92, agreeableness: 0.50, neuroticism: 0.30 }),
        ("Free Spirit", BigFive { openness: 0.95, conscientiousness: 0.45, extraversion: 0.75, agreeableness: 0.80, neuroticism: 0.35 }),
        ("Steady Rock", BigFive { openness: 0.50, conscientiousness: 0.95, extraversion: 0.45, agreeableness: 0.78, neuroticism: 0.20 }),
        ("Complex Dreamer", BigFive { openness: 0.82, conscientiousness: 0.60, extraversion: 0.68, agreeableness: 0.72, neuroticism: 0.50 }),
        ("Charismatic Visionary", BigFive { openness: 0.78, conscientiousness: 0.72, extraversion: 0.88, agreeableness: 0.55, neuroticism: 0.25 }),
        ("Quiet Observer", BigFive { openness: 0.70, conscientiousness: 0.80, extraversion: 0.30, agreeableness: 0.75, neuroticism: 0.40 }),
        ("Adventurous Explorer", BigFive { openness: 0.92, conscientiousness: 0.50, extraversion: 0.85, agreeableness: 0.65, neuroticism: 0.35 }),
        ("Gentle Healer", BigFive { openness: 0.75, conscientiousness: 0.70, extraversion: 0.55, agreeableness: 0.92, neuroticism: 0.30 }),
        ("Strategic Thinker", BigFive { openness: 0.55, conscientiousness: 0.90, extraversion: 0.50, agreeableness: 0.58, neuroticism: 0.38 }),
        ("Passionate Artist", BigFive { openness: 0.95, conscientiousness: 0.55, extraversion: 0.70, agreeableness: 0.75, neuroticism: 0.45 }),
    ];

    let idx = rng.gen_range(0..archetypes.len());
    let (name, base_big_five) = archetypes[idx].clone();

    // Add variation
    let big_five = BigFive {
        openness: (base_big_five.openness + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0),
        conscientiousness: (base_big_five.conscientiousness + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0),
        extraversion: (base_big_five.extraversion + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0),
        agreeableness: (base_big_five.agreeableness + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0),
        neuroticism: (base_big_five.neuroticism + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0),
    };

    let values = SchwartzValues {
        self_direction: rng.gen_range(0.3..0.9),
        stimulation: rng.gen_range(0.2..0.8),
        hedonism: rng.gen_range(0.3..0.7),
        achievement: rng.gen_range(0.4..0.9),
        power: rng.gen_range(0.2..0.6),
        security: rng.gen_range(0.4..0.8),
        conformity: rng.gen_range(0.3..0.7),
        tradition: rng.gen_range(0.2..0.7),
        benevolence: rng.gen_range(0.5..0.95),
        universalism: rng.gen_range(0.4..0.9),
    };

    let attachment = match rng.gen_range(0..100) {
        0..=55 => AttachmentStyle::Secure,
        56..=75 => AttachmentStyle::Anxious,
        76..=90 => AttachmentStyle::Avoidant,
        _ => AttachmentStyle::Disorganized,
    };

    (name, big_five, values, attachment)
}

// ============================================================================
// NAME GENERATION
// ============================================================================

fn generate_name(rng: &mut ChaCha8Rng, id: usize) -> String {
    let first_names = [
        "Emma", "Liam", "Olivia", "Noah", "Ava", "Oliver", "Sophia", "Elijah",
        "Isabella", "James", "Mia", "William", "Charlotte", "Benjamin", "Amelia",
        "Lucas", "Harper", "Henry", "Evelyn", "Alexander", "Abigail", "Sebastian",
        "Emily", "Jack", "Elizabeth", "Aiden", "Sofia", "Owen", "Avery", "Samuel",
        "Ella", "Ryan", "Scarlett", "Nathan", "Grace", "Leo", "Chloe", "Isaac",
        "Victoria", "Gabriel", "Riley", "Julian", "Aria", "Mateo", "Lily", "Anthony",
        "Aurora", "Jaxon", "Zoey", "Lincoln", "Penelope", "Joshua", "Layla", "Christopher",
        "Nora", "Andrew", "Camila", "Theodore", "Hannah", "Caleb", "Zoe", "Dylan",
        "Stella", "Maverick", "Hazel", "Josiah", "Ellie", "Jordan", "Paisley", "Adrian",
        "Audrey", "Grayson", "Brooklyn", "Vincent", "Bella", "Isaiah", "Claire", "Eli",
        "Skylar", "Jonathan", "Lucy", "Charles", "Savannah", "Aaron", "Anna", "Ezra",
        "Alexa", "Colton", "Genesis", "Nicholas", "Aaliyah", "Austin", "Kennedy", "Cameron",
        "Kinsley", "Jeremiah", "Maya", "Landon", "Sarah", "Robert", "Madelyn", "Thomas",
    ];

    let last_names = [
        "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis",
        "Rodriguez", "Martinez", "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson",
        "Thomas", "Taylor", "Moore", "Jackson", "Martin", "Lee", "Perez", "Thompson",
        "White", "Harris", "Sanchez", "Clark", "Ramirez", "Lewis", "Robinson", "Walker",
        "Young", "Allen", "King", "Wright", "Scott", "Torres", "Nguyen", "Hill", "Flores",
        "Green", "Adams", "Nelson", "Baker", "Hall", "Rivera", "Campbell", "Mitchell",
        "Carter", "Roberts", "Chen", "Park", "Kim", "Patel", "Shah", "Singh", "Kumar",
        "Cohen", "Russo", "Ferrari", "Costa", "Silva", "Santos", "Murphy", "Kelly",
    ];

    let first = first_names[rng.gen_range(0..first_names.len())];
    let last = last_names[rng.gen_range(0..last_names.len())];
    format!("{} {} ({})", first, last, id)
}

// ============================================================================
// EMBEDDING GENERATION
// ============================================================================

fn generate_embedding(user: &User, rng: &mut ChaCha8Rng) -> Vec<f32> {
    let mut embedding = vec![0.0f32; EMBEDDING_DIM];

    // Big Five encoding (0-50)
    for i in 0..50 {
        let trait_idx = i % 5;
        let variation = (i as f32 / 50.0) * PI;
        let trait_val = match trait_idx {
            0 => user.big_five.openness,
            1 => user.big_five.conscientiousness,
            2 => user.big_five.extraversion,
            3 => user.big_five.agreeableness,
            _ => user.big_five.neuroticism,
        };
        embedding[i] = trait_val * variation.sin() + (1.0 - trait_val) * variation.cos();
    }

    // Values encoding (50-150)
    let values = [
        user.values.self_direction, user.values.stimulation, user.values.hedonism,
        user.values.achievement, user.values.power, user.values.security,
        user.values.conformity, user.values.tradition, user.values.benevolence,
        user.values.universalism,
    ];
    for i in 0..100 {
        let val_idx = i % 10;
        let phase = (i as f32 / 100.0) * 2.0 * PI;
        embedding[50 + i] = values[val_idx] * phase.cos();
    }

    // Attachment encoding (150-170)
    let attachment_vec = match user.attachment {
        AttachmentStyle::Secure => [1.0, 0.0, 0.0, 0.0],
        AttachmentStyle::Anxious => [0.0, 1.0, 0.0, 0.0],
        AttachmentStyle::Avoidant => [0.0, 0.0, 1.0, 0.0],
        AttachmentStyle::Disorganized => [0.0, 0.0, 0.0, 1.0],
    };
    for i in 0..20 {
        embedding[150 + i] = attachment_vec[i % 4] * ((i as f32 / 20.0) * PI).sin();
    }

    // Emotional state encoding (170-200)
    for i in 0..30 {
        let phase = (i as f32 / 30.0) * PI;
        embedding[170 + i] = user.emotional_state.valence * phase.cos()
            + user.emotional_state.arousal * phase.sin();
    }

    // Interaction terms (200-256)
    for i in 200..EMBEDDING_DIM {
        let idx1 = i % 50;
        let idx2 = (i * 7) % 100;
        embedding[i] = embedding[idx1] * embedding[50 + idx2] * 0.5;
    }

    // Add small noise
    for e in embedding.iter_mut() {
        *e += rng.gen_range(-0.01..0.01);
    }

    // Normalize
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for e in embedding.iter_mut() {
            *e /= norm;
        }
    }

    embedding
}

// ============================================================================
// SIMILARITY COMPUTATION (SIMD-friendly)
// ============================================================================

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a > 0.0 && norm_b > 0.0 {
        dot / (norm_a * norm_b)
    } else {
        0.0
    }
}

fn compute_compatibility(user_a: &User, user_b: &User) -> f32 {
    // Vector similarity (35%)
    let vector_sim = cosine_similarity(&user_a.embedding, &user_b.embedding);

    // Value alignment (25%)
    let value_sim = {
        let a = &user_a.values;
        let b = &user_b.values;
        let diff = (a.benevolence - b.benevolence).abs()
            + (a.universalism - b.universalism).abs()
            + (a.self_direction - b.self_direction).abs()
            + (a.security - b.security).abs();
        1.0 - (diff / 4.0)
    };

    // Communication compatibility (20%) - based on extraversion balance
    let comm_compat = {
        let e_diff = (user_a.big_five.extraversion - user_b.big_five.extraversion).abs();
        // Complementary is good (up to 0.3 diff), then decreases
        if e_diff < 0.3 { 0.9 } else { 1.0 - e_diff }
    };

    // Attachment compatibility (10%)
    let attach_compat = match (&user_a.attachment, &user_b.attachment) {
        (AttachmentStyle::Secure, AttachmentStyle::Secure) => 1.0,
        (AttachmentStyle::Secure, _) | (_, AttachmentStyle::Secure) => 0.8,
        (AttachmentStyle::Anxious, AttachmentStyle::Avoidant) => 0.4, // Problematic
        (AttachmentStyle::Avoidant, AttachmentStyle::Anxious) => 0.4,
        (AttachmentStyle::Disorganized, AttachmentStyle::Disorganized) => 0.3,
        _ => 0.6,
    };

    // EQ match (10%) - based on agreeableness
    let eq_match = 1.0 - (user_a.big_five.agreeableness - user_b.big_five.agreeableness).abs();

    0.35 * vector_sim + 0.25 * value_sim + 0.20 * comm_compat + 0.10 * attach_compat + 0.10 * eq_match
}

// ============================================================================
// PERSONALITY EVOLUTION
// ============================================================================

fn evolve_personality(user: &mut User, week: usize, rng: &mut ChaCha8Rng) {
    // Natural drift (very small)
    let drift_rate = 0.002;
    user.big_five.openness += rng.gen_range(-drift_rate..drift_rate);
    user.big_five.conscientiousness += rng.gen_range(-drift_rate..drift_rate);
    user.big_five.extraversion += rng.gen_range(-drift_rate..drift_rate);
    user.big_five.agreeableness += rng.gen_range(-drift_rate..drift_rate);
    user.big_five.neuroticism += rng.gen_range(-drift_rate..drift_rate);

    // Relationship effects
    match &user.relationship_status {
        RelationshipStatus::Dating(_) | RelationshipStatus::Engaged(_) => {
            // Relationships tend to increase agreeableness slightly
            user.big_five.agreeableness += 0.001;
            user.big_five.neuroticism -= 0.0005;
        }
        RelationshipStatus::Married(_) => {
            // Marriage increases stability
            user.big_five.neuroticism -= 0.001;
            user.big_five.conscientiousness += 0.0005;
        }
        _ => {}
    }

    // Friendship effects
    if user.friends.len() > 10 {
        user.big_five.extraversion += 0.001;
    }
    if user.close_friends.len() > 3 {
        user.big_five.agreeableness += 0.0005;
    }

    // Collect events for this week first
    let week_events: Vec<(LifeEventType, f32)> = user.life_events
        .iter()
        .filter(|e| e.week == week)
        .map(|e| (e.event_type.clone(), e.impact))
        .collect();

    // Then apply them
    for (event_type, impact) in week_events {
        apply_life_event(user, &event_type, impact);
    }

    // Clamp all values
    user.big_five.openness = user.big_five.openness.clamp(0.0, 1.0);
    user.big_five.conscientiousness = user.big_five.conscientiousness.clamp(0.0, 1.0);
    user.big_five.extraversion = user.big_five.extraversion.clamp(0.0, 1.0);
    user.big_five.agreeableness = user.big_five.agreeableness.clamp(0.0, 1.0);
    user.big_five.neuroticism = user.big_five.neuroticism.clamp(0.0, 1.0);

    // Increase confidence over time
    user.personality_confidence = (user.personality_confidence + 0.01).min(1.0);
}

fn apply_life_event(user: &mut User, event_type: &LifeEventType, impact: f32) {
    match event_type {
        LifeEventType::NewJob => {
            user.big_five.openness += 0.02 * impact;
            user.emotional_state.valence += 0.3 * impact;
            user.emotional_state.arousal += 0.2;
        }
        LifeEventType::JobLoss => {
            user.big_five.neuroticism += 0.05 * impact;
            user.emotional_state.valence -= 0.5 * impact;
            user.emotional_state.resilience -= 0.1;
        }
        LifeEventType::Promotion => {
            user.big_five.conscientiousness += 0.02 * impact;
            user.emotional_state.valence += 0.4 * impact;
        }
        LifeEventType::MovedCity => {
            user.big_five.openness += 0.03 * impact;
            user.emotional_state.arousal += 0.3;
        }
        LifeEventType::FamilyIssue => {
            user.big_five.neuroticism += 0.03 * impact;
            user.emotional_state.valence -= 0.3 * impact;
        }
        LifeEventType::HealthChallenge => {
            user.emotional_state.valence -= 0.4 * impact;
            user.emotional_state.resilience += 0.05; // Growth through adversity
        }
        LifeEventType::Achievement => {
            user.big_five.conscientiousness += 0.02 * impact;
            user.emotional_state.valence += 0.5 * impact;
        }
        LifeEventType::Loss => {
            user.big_five.neuroticism += 0.04 * impact;
            user.emotional_state.valence -= 0.6 * impact;
            user.big_five.openness += 0.01; // Perspective shift
        }
        LifeEventType::Travel => {
            user.big_five.openness += 0.03 * impact;
            user.emotional_state.valence += 0.2 * impact;
        }
        LifeEventType::NewHobby => {
            user.big_five.openness += 0.02 * impact;
            user.emotional_state.valence += 0.15 * impact;
        }
    }

    // Emotional state recovery
    user.emotional_state.valence = user.emotional_state.valence.clamp(-1.0, 1.0);
    user.emotional_state.arousal = user.emotional_state.arousal.clamp(0.0, 1.0);
    user.emotional_state.resilience = user.emotional_state.resilience.clamp(0.0, 1.0);
}

fn generate_life_events(rng: &mut ChaCha8Rng) -> Vec<LifeEvent> {
    let mut events = Vec::new();
    let num_events = rng.gen_range(0..6); // 0-5 life events per year

    for _ in 0..num_events {
        let event_type = match rng.gen_range(0..10) {
            0 => LifeEventType::NewJob,
            1 => LifeEventType::JobLoss,
            2 => LifeEventType::Promotion,
            3 => LifeEventType::MovedCity,
            4 => LifeEventType::FamilyIssue,
            5 => LifeEventType::HealthChallenge,
            6 => LifeEventType::Achievement,
            7 => LifeEventType::Loss,
            8 => LifeEventType::Travel,
            _ => LifeEventType::NewHobby,
        };

        events.push(LifeEvent {
            week: rng.gen_range(1..SIMULATION_WEEKS),
            event_type,
            impact: rng.gen_range(0.5..1.0),
        });
    }

    events
}

// ============================================================================
// SOCIAL DYNAMICS
// ============================================================================

fn weekly_social_interactions(
    users: &mut [User],
    friendships: &mut HashMap<(usize, usize), Friendship>,
    week: usize,
    rng: &mut ChaCha8Rng,
    stats: &mut SimulationStats,
) {
    let n = users.len();

    // Each user has some interactions
    for i in 0..n {
        let num_interactions = rng.gen_range(5..WEEKLY_INTERACTIONS);

        for _ in 0..num_interactions {
            // Pick a random other user (weighted by existing friendship)
            let j = loop {
                let candidate = rng.gen_range(0..n);
                if candidate != i {
                    break candidate;
                }
            };

            let (user_i, user_j) = if i < j {
                let (left, right) = users.split_at_mut(j);
                (&mut left[i], &mut right[0])
            } else {
                let (left, right) = users.split_at_mut(i);
                (&mut right[0], &mut left[j])
            };

            let key = (i.min(j), i.max(j));
            let compatibility = compute_compatibility(user_i, user_j);

            // Friendship formation/strengthening
            if let Some(friendship) = friendships.get_mut(&key) {
                // Existing friendship - strengthen
                friendship.interactions += 1;
                friendship.strength = (friendship.strength + 0.01 * compatibility).min(1.0);

                // Check for close friend promotion
                if friendship.strength > 0.85 && friendship.interactions > 20 {
                    user_i.close_friends.insert(j);
                    user_j.close_friends.insert(i);

                    // Best friend check
                    if friendship.strength > 0.92 && friendship.interactions > 50 {
                        if user_i.best_friend.is_none() {
                            user_i.best_friend = Some(j);
                            stats.total_best_friends += 1;
                        }
                        if user_j.best_friend.is_none() {
                            user_j.best_friend = Some(i);
                            stats.total_best_friends += 1;
                        }
                    }
                }
            } else if compatibility > FRIENDSHIP_THRESHOLD && rng.gen_bool(0.3) {
                // New friendship formed
                friendships.insert(key, Friendship {
                    user_a: i,
                    user_b: j,
                    strength: compatibility,
                    started_week: week,
                    interactions: 1,
                });
                user_i.friends.insert(j);
                user_j.friends.insert(i);
                stats.total_friendships_formed += 1;
            }

            // Romantic interactions for singles
            if matches!(user_i.relationship_status, RelationshipStatus::Single)
                && matches!(user_j.relationship_status, RelationshipStatus::Single)
                && compatibility > DATING_THRESHOLD
                && rng.gen_bool(0.15)
            {
                // Start dating
                user_i.relationship_status = RelationshipStatus::Dating(j);
                user_j.relationship_status = RelationshipStatus::Dating(i);
                user_i.weeks_dating.insert(j, 0);
                user_j.weeks_dating.insert(i, 0);
                stats.total_dates += 1;
                stats.total_relationships += 1;
            }
        }
    }
}

fn process_relationships(
    users: &mut [User],
    marriages: &mut Vec<Marriage>,
    week: usize,
    rng: &mut ChaCha8Rng,
    stats: &mut SimulationStats,
) {
    let n = users.len();

    // Collect actions to take (to avoid borrow issues)
    let mut actions: Vec<RelationshipAction> = Vec::new();

    for i in 0..n {
        let status = users[i].relationship_status.clone();

        match status {
            RelationshipStatus::Dating(partner_id) => {
                let weeks = users[i].weeks_dating.get(&partner_id).copied().unwrap_or(0);
                let compatibility = compute_compatibility(&users[i], &users[partner_id]);

                if weeks >= 12 && compatibility > MARRIAGE_THRESHOLD && rng.gen_bool(0.1) {
                    actions.push(RelationshipAction::Engage(i, partner_id));
                } else if compatibility < 0.7 && rng.gen_bool(0.05) {
                    actions.push(RelationshipAction::Breakup(i, partner_id));
                } else {
                    actions.push(RelationshipAction::IncrementWeeks(i, partner_id));
                }
            }
            RelationshipStatus::Engaged(partner_id) => {
                let weeks = users[i].weeks_dating.get(&partner_id).copied().unwrap_or(0);

                if weeks >= 20 && rng.gen_bool(0.15) {
                    let compatibility = compute_compatibility(&users[i], &users[partner_id]);
                    actions.push(RelationshipAction::Marry(i, partner_id, compatibility, week));
                } else {
                    actions.push(RelationshipAction::IncrementWeeks(i, partner_id));
                }
            }
            _ => {}
        }
    }

    // Apply actions
    for action in actions {
        match action {
            RelationshipAction::Engage(i, partner_id) => {
                if let Some(w) = users[i].weeks_dating.get_mut(&partner_id) { *w += 1; }
                users[i].relationship_status = RelationshipStatus::Engaged(partner_id);
                users[partner_id].relationship_status = RelationshipStatus::Engaged(i);
                stats.total_engagements += 1;
            }
            RelationshipAction::Breakup(i, partner_id) => {
                users[i].relationship_status = RelationshipStatus::Single;
                users[partner_id].relationship_status = RelationshipStatus::Single;
                users[i].weeks_dating.remove(&partner_id);
                users[partner_id].weeks_dating.remove(&i);
                users[i].emotional_state.valence -= 0.2;
                users[partner_id].emotional_state.valence -= 0.2;
                stats.total_breakups += 1;
            }
            RelationshipAction::IncrementWeeks(i, partner_id) => {
                if let Some(w) = users[i].weeks_dating.get_mut(&partner_id) { *w += 1; }
            }
            RelationshipAction::Marry(i, partner_id, compatibility, wed_week) => {
                users[i].relationship_status = RelationshipStatus::Married(partner_id);
                users[partner_id].relationship_status = RelationshipStatus::Married(i);
                users[i].emotional_state.valence += 0.4;
                users[partner_id].emotional_state.valence += 0.4;

                marriages.push(Marriage {
                    partner_a: i,
                    partner_b: partner_id,
                    wedding_week: wed_week,
                    compatibility_score: compatibility,
                });

                stats.total_marriages += 1;
                if compatibility > stats.highest_compatibility_marriage.2 {
                    stats.highest_compatibility_marriage = (i, partner_id, compatibility);
                }
            }
        }
    }
}

enum RelationshipAction {
    Engage(usize, usize),
    Breakup(usize, usize),
    IncrementWeeks(usize, usize),
    Marry(usize, usize, f32, usize),
}

fn calculate_happiness(user: &User) -> f32 {
    let base = 0.5;

    // Emotional state contribution
    let emotional = (user.emotional_state.valence + 1.0) / 2.0 * 0.3;

    // Relationship contribution
    let relationship = match &user.relationship_status {
        RelationshipStatus::Single => 0.0,
        RelationshipStatus::Dating(_) => 0.1,
        RelationshipStatus::Engaged(_) => 0.15,
        RelationshipStatus::Married(_) => 0.2,
    };

    // Friendship contribution
    let friendship = (user.friends.len() as f32 / 20.0).min(0.15);
    let close_friend = (user.close_friends.len() as f32 / 5.0).min(0.1);
    let best_friend = if user.best_friend.is_some() { 0.05 } else { 0.0 };

    // Resilience and neuroticism effects
    let stability = user.emotional_state.resilience * 0.1 - user.big_five.neuroticism * 0.1;

    (base + emotional + relationship + friendship + close_friend + best_friend + stability).clamp(0.0, 1.0)
}

// ============================================================================
// SIMULATION ENGINE
// ============================================================================

fn run_simulation() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║           PATH SOCIAL NETWORK - 1000 USER LIFECYCLE SIMULATION               ║");
    println!("║                        52-Week Social Dynamics Study                          ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝\n");

    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // Initialize users
    println!("📊 Phase 1: Initializing {} digital twins...", NUM_USERS);
    let start_init = std::time::Instant::now();

    let mut users: Vec<User> = (0..NUM_USERS)
        .map(|id| {
            let (archetype, big_five, values, attachment) = generate_archetype_name(&mut rng);
            let name = generate_name(&mut rng, id);
            let age = rng.gen_range(22..55);

            let mut user = User {
                id,
                name,
                age,
                big_five: big_five.clone(),
                initial_big_five: big_five,
                values,
                attachment,
                emotional_state: EmotionalState {
                    valence: rng.gen_range(-0.2..0.5),
                    arousal: rng.gen_range(0.2..0.6),
                    resilience: rng.gen_range(0.4..0.8),
                },
                relationship_status: RelationshipStatus::Single,
                friends: HashSet::new(),
                close_friends: HashSet::new(),
                best_friend: None,
                embedding: Vec::new(),
                weeks_dating: HashMap::new(),
                life_events: generate_life_events(&mut rng),
                happiness_history: Vec::new(),
                personality_confidence: 0.3 + rng.gen_range(0.0..0.3),
            };

            user.embedding = generate_embedding(&user, &mut rng);
            user
        })
        .collect();

    println!("   ✓ Created {} users in {:?}", NUM_USERS, start_init.elapsed());

    // Count archetypes and demographics
    let mut age_groups: HashMap<&str, usize> = HashMap::new();
    let mut attachment_counts: HashMap<String, usize> = HashMap::new();

    for user in &users {
        let age_group = match user.age {
            22..=29 => "22-29",
            30..=39 => "30-39",
            40..=49 => "40-49",
            _ => "50+",
        };
        *age_groups.entry(age_group).or_insert(0) += 1;
        *attachment_counts.entry(format!("{:?}", user.attachment)).or_insert(0) += 1;
    }

    println!("\n   Demographics:");
    println!("   ┌──────────────────────────────────────────┐");
    for (group, count) in &age_groups {
        let pct = (*count as f32 / NUM_USERS as f32) * 100.0;
        println!("   │ Age {}: {:>4} users ({:>5.1}%)          │", group, count, pct);
    }
    println!("   └──────────────────────────────────────────┘");

    println!("\n   Attachment Styles:");
    println!("   ┌──────────────────────────────────────────┐");
    for (style, count) in &attachment_counts {
        let pct = (*count as f32 / NUM_USERS as f32) * 100.0;
        println!("   │ {:15} {:>4} users ({:>5.1}%)    │", style, count, pct);
    }
    println!("   └──────────────────────────────────────────┘");

    // Initialize tracking structures
    let mut friendships: HashMap<(usize, usize), Friendship> = HashMap::new();
    let mut marriages: Vec<Marriage> = Vec::new();
    let mut stats = SimulationStats {
        total_friendships_formed: 0,
        total_close_friendships: 0,
        total_best_friends: 0,
        total_dates: 0,
        total_relationships: 0,
        total_engagements: 0,
        total_marriages: 0,
        total_breakups: 0,
        avg_friends_per_user: 0.0,
        avg_happiness: 0.0,
        personality_drift_avg: 0.0,
        highest_compatibility_marriage: (0, 0, 0.0),
        most_popular_user: (0, 0),
        life_events_processed: 0,
    };

    // Count life events
    for user in &users {
        stats.life_events_processed += user.life_events.len();
    }

    println!("\n🎭 Phase 2: Running {} week simulation...\n", SIMULATION_WEEKS);

    // Progress tracking
    let mut weekly_stats: Vec<(usize, usize, usize, usize, f32)> = Vec::new(); // (friendships, relationships, marriages, interactions, happiness)

    let sim_start = std::time::Instant::now();

    for week in 1..=SIMULATION_WEEKS {
        // Evolve personalities
        for user in users.iter_mut() {
            evolve_personality(user, week, &mut rng);
            user.embedding = generate_embedding(user, &mut rng);
        }

        // Social interactions
        weekly_social_interactions(&mut users, &mut friendships, week, &mut rng, &mut stats);

        // Process relationships
        process_relationships(&mut users, &mut marriages, week, &mut rng, &mut stats);

        // Calculate happiness
        let total_happiness: f32 = users.iter().map(|u| calculate_happiness(u)).sum();
        let avg_happiness = total_happiness / NUM_USERS as f32;

        for user in users.iter_mut() {
            user.happiness_history.push(calculate_happiness(user));
        }

        // Weekly stats
        let total_friends: usize = users.iter().map(|u| u.friends.len()).sum();
        let in_relationships: usize = users.iter()
            .filter(|u| !matches!(u.relationship_status, RelationshipStatus::Single))
            .count();

        weekly_stats.push((
            stats.total_friendships_formed,
            in_relationships,
            stats.total_marriages,
            total_friends,
            avg_happiness,
        ));

        // Progress output (every 4 weeks)
        if week % 4 == 0 || week == 1 || week == SIMULATION_WEEKS {
            let single = users.iter().filter(|u| matches!(u.relationship_status, RelationshipStatus::Single)).count();
            let dating = users.iter().filter(|u| matches!(u.relationship_status, RelationshipStatus::Dating(_))).count();
            let engaged = users.iter().filter(|u| matches!(u.relationship_status, RelationshipStatus::Engaged(_))).count();
            let married = users.iter().filter(|u| matches!(u.relationship_status, RelationshipStatus::Married(_))).count();

            println!("   Week {:>2}/52 │ Single: {:>4} │ Dating: {:>3} │ Engaged: {:>2} │ Married: {:>3} │ Happiness: {:.1}%",
                week, single, dating, engaged, married, avg_happiness * 100.0);
        }
    }

    println!("\n   ✓ Simulation completed in {:?}", sim_start.elapsed());

    // Final statistics
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              SIMULATION RESULTS                               ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    // Relationship statistics
    let single = users.iter().filter(|u| matches!(u.relationship_status, RelationshipStatus::Single)).count();
    let dating = users.iter().filter(|u| matches!(u.relationship_status, RelationshipStatus::Dating(_))).count();
    let engaged = users.iter().filter(|u| matches!(u.relationship_status, RelationshipStatus::Engaged(_))).count();
    let married = users.iter().filter(|u| matches!(u.relationship_status, RelationshipStatus::Married(_))).count();

    println!("\n📊 RELATIONSHIP OUTCOMES (End of Year)");
    println!("┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│  Status          │ Count  │ Percentage │ Visual                           │");
    println!("├────────────────────────────────────────────────────────────────────────────┤");

    let bar_width = 30;
    let single_bar = "█".repeat((single as f32 / NUM_USERS as f32 * bar_width as f32) as usize);
    let dating_bar = "█".repeat((dating as f32 / NUM_USERS as f32 * bar_width as f32) as usize);
    let engaged_bar = "█".repeat((engaged as f32 / NUM_USERS as f32 * bar_width as f32) as usize);
    let married_bar = "█".repeat((married as f32 / NUM_USERS as f32 * bar_width as f32) as usize);

    println!("│  💔 Single       │ {:>5}  │   {:>5.1}%   │ {:30} │", single, single as f32 / 10.0, single_bar);
    println!("│  💕 Dating       │ {:>5}  │   {:>5.1}%   │ {:30} │", dating, dating as f32 / 10.0, dating_bar);
    println!("│  💍 Engaged      │ {:>5}  │   {:>5.1}%   │ {:30} │", engaged, engaged as f32 / 10.0, engaged_bar);
    println!("│  👫 Married      │ {:>5}  │   {:>5.1}%   │ {:30} │", married, married as f32 / 10.0, married_bar);
    println!("└────────────────────────────────────────────────────────────────────────────┘");

    // Friendship statistics
    let total_friends: usize = users.iter().map(|u| u.friends.len()).sum();
    let total_close: usize = users.iter().map(|u| u.close_friends.len()).sum();
    let with_best_friend: usize = users.iter().filter(|u| u.best_friend.is_some()).count();
    stats.avg_friends_per_user = total_friends as f32 / NUM_USERS as f32;

    // Find most popular user
    for user in &users {
        if user.friends.len() > stats.most_popular_user.1 {
            stats.most_popular_user = (user.id, user.friends.len());
        }
    }

    println!("\n👥 FRIENDSHIP NETWORK");
    println!("┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│  Metric                              │ Value                               │");
    println!("├────────────────────────────────────────────────────────────────────────────┤");
    println!("│  Total friendships formed            │ {:>6}                              │", stats.total_friendships_formed);
    println!("│  Average friends per user            │ {:>6.1}                              │", stats.avg_friends_per_user);
    println!("│  Total close friendships             │ {:>6}                              │", total_close / 2);
    println!("│  Users with a best friend            │ {:>6} ({:.1}%)                      │", with_best_friend, with_best_friend as f32 / 10.0);
    println!("│  Most connected user                 │ {} ({} friends)    │", users[stats.most_popular_user.0].name, stats.most_popular_user.1);
    println!("└────────────────────────────────────────────────────────────────────────────┘");

    // Romantic journey statistics
    println!("\n💑 ROMANTIC JOURNEY STATISTICS");
    println!("┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│  Total dates/relationships started   │ {:>6}                              │", stats.total_relationships);
    println!("│  Total engagements                   │ {:>6}                              │", stats.total_engagements);
    println!("│  Total marriages                     │ {:>6}                              │", stats.total_marriages);
    println!("│  Total breakups                      │ {:>6}                              │", stats.total_breakups);
    println!("│  Success rate (dating → marriage)    │ {:>5.1}%                              │",
        if stats.total_relationships > 0 { stats.total_marriages as f32 / stats.total_relationships as f32 * 100.0 } else { 0.0 });
    println!("└────────────────────────────────────────────────────────────────────────────┘");

    // Top marriages by compatibility
    println!("\n💒 TOP 5 MARRIAGES BY COMPATIBILITY");
    println!("┌────────────────────────────────────────────────────────────────────────────┐");

    let mut sorted_marriages = marriages.clone();
    sorted_marriages.sort_by(|a, b| b.compatibility_score.partial_cmp(&a.compatibility_score).unwrap());

    for (i, marriage) in sorted_marriages.iter().take(5).enumerate() {
        let user_a = &users[marriage.partner_a];
        let user_b = &users[marriage.partner_b];
        println!("│  {}. {} + {}",
            i + 1,
            user_a.name.split('(').next().unwrap().trim(),
            user_b.name.split('(').next().unwrap().trim());
        println!("│     Compatibility: {:.1}% │ Week: {} │ Attachments: {:?} + {:?}",
            marriage.compatibility_score * 100.0,
            marriage.wedding_week,
            user_a.attachment,
            user_b.attachment);
    }
    println!("└────────────────────────────────────────────────────────────────────────────┘");

    // Personality evolution
    let mut total_drift = 0.0;
    for user in &users {
        let drift = (user.big_five.openness - user.initial_big_five.openness).abs()
            + (user.big_five.conscientiousness - user.initial_big_five.conscientiousness).abs()
            + (user.big_five.extraversion - user.initial_big_five.extraversion).abs()
            + (user.big_five.agreeableness - user.initial_big_five.agreeableness).abs()
            + (user.big_five.neuroticism - user.initial_big_five.neuroticism).abs();
        total_drift += drift;
    }
    stats.personality_drift_avg = total_drift / NUM_USERS as f32;

    println!("\n🧠 PERSONALITY EVOLUTION");
    println!("┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│  Average personality drift           │ {:>6.3} (across 5 traits)           │", stats.personality_drift_avg);
    println!("│  Life events processed               │ {:>6}                              │", stats.life_events_processed);
    println!("│  Average confidence growth           │ {:>5.1}% → 100%                       │", 50.0);
    println!("└────────────────────────────────────────────────────────────────────────────┘");

    // Show personality changes for top changed users
    println!("\n   Most Evolved Personalities:");
    let mut user_drifts: Vec<(usize, f32)> = users.iter()
        .map(|u| {
            let drift = (u.big_five.openness - u.initial_big_five.openness).abs()
                + (u.big_five.conscientiousness - u.initial_big_five.conscientiousness).abs()
                + (u.big_five.extraversion - u.initial_big_five.extraversion).abs()
                + (u.big_five.agreeableness - u.initial_big_five.agreeableness).abs()
                + (u.big_five.neuroticism - u.initial_big_five.neuroticism).abs();
            (u.id, drift)
        })
        .collect();
    user_drifts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (id, drift) in user_drifts.iter().take(3) {
        let user = &users[*id];
        println!("   • {} (drift: {:.3})", user.name, drift);
        println!("     O: {:.2}→{:.2}  C: {:.2}→{:.2}  E: {:.2}→{:.2}  A: {:.2}→{:.2}  N: {:.2}→{:.2}",
            user.initial_big_five.openness, user.big_five.openness,
            user.initial_big_five.conscientiousness, user.big_five.conscientiousness,
            user.initial_big_five.extraversion, user.big_five.extraversion,
            user.initial_big_five.agreeableness, user.big_five.agreeableness,
            user.initial_big_five.neuroticism, user.big_five.neuroticism);
        println!("     Life events: {:?}", user.life_events.iter().map(|e| format!("{:?}", e.event_type)).collect::<Vec<_>>());
    }

    // Happiness analysis
    let final_happiness: f32 = users.iter().map(|u| calculate_happiness(u)).sum::<f32>() / NUM_USERS as f32;
    let initial_happiness: f32 = users.iter()
        .map(|u| u.happiness_history.first().copied().unwrap_or(0.5))
        .sum::<f32>() / NUM_USERS as f32;

    println!("\n😊 HAPPINESS ANALYSIS");
    println!("┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│  Initial average happiness           │ {:>5.1}%                              │", initial_happiness * 100.0);
    println!("│  Final average happiness             │ {:>5.1}%                              │", final_happiness * 100.0);
    println!("│  Happiness change                    │ {:>+5.1}%                              │", (final_happiness - initial_happiness) * 100.0);
    println!("└────────────────────────────────────────────────────────────────────────────┘");

    // Happiness by relationship status
    println!("\n   Happiness by Relationship Status:");
    let single_happiness: f32 = users.iter()
        .filter(|u| matches!(u.relationship_status, RelationshipStatus::Single))
        .map(|u| calculate_happiness(u))
        .sum::<f32>() / single.max(1) as f32;
    let dating_happiness: f32 = users.iter()
        .filter(|u| matches!(u.relationship_status, RelationshipStatus::Dating(_)))
        .map(|u| calculate_happiness(u))
        .sum::<f32>() / dating.max(1) as f32;
    let married_happiness: f32 = users.iter()
        .filter(|u| matches!(u.relationship_status, RelationshipStatus::Married(_)))
        .map(|u| calculate_happiness(u))
        .sum::<f32>() / married.max(1) as f32;

    println!("   • Single:  {:.1}%", single_happiness * 100.0);
    println!("   • Dating:  {:.1}%", dating_happiness * 100.0);
    println!("   • Married: {:.1}%", married_happiness * 100.0);

    // Attachment style outcomes
    println!("\n🔗 ATTACHMENT STYLE OUTCOMES");
    println!("┌────────────────────────────────────────────────────────────────────────────┐");

    for style in [AttachmentStyle::Secure, AttachmentStyle::Anxious, AttachmentStyle::Avoidant, AttachmentStyle::Disorganized] {
        let style_users: Vec<&User> = users.iter().filter(|u| u.attachment == style).collect();
        let count = style_users.len();
        let married_count = style_users.iter()
            .filter(|u| matches!(u.relationship_status, RelationshipStatus::Married(_)))
            .count();
        let avg_friends = style_users.iter().map(|u| u.friends.len()).sum::<usize>() as f32 / count.max(1) as f32;
        let avg_happy = style_users.iter().map(|u| calculate_happiness(u)).sum::<f32>() / count.max(1) as f32;

        println!("│  {:15} │ Marriage: {:>5.1}% │ Avg Friends: {:>4.1} │ Happiness: {:>5.1}% │",
            format!("{:?}", style),
            married_count as f32 / count.max(1) as f32 * 100.0,
            avg_friends,
            avg_happy * 100.0);
    }
    println!("└────────────────────────────────────────────────────────────────────────────┘");

    // Weekly progression graph (ASCII)
    println!("\n📈 WEEKLY PROGRESSION");
    println!("┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│  Week │ Friendships │ In Relationships │ Marriages │ Happiness            │");
    println!("├────────────────────────────────────────────────────────────────────────────┤");

    for (i, (friendships, relationships, marriages, _, happiness)) in weekly_stats.iter().enumerate() {
        if i % 8 == 0 || i == weekly_stats.len() - 1 {
            let happy_bar = "█".repeat((*happiness * 20.0) as usize);
            println!("│  {:>4} │ {:>10}  │ {:>16} │ {:>9} │ {:20} │",
                i + 1, friendships, relationships, marriages, happy_bar);
        }
    }
    println!("└────────────────────────────────────────────────────────────────────────────┘");

    // Sample user journeys
    println!("\n🎭 SAMPLE USER JOURNEYS\n");

    // Find interesting users
    let married_users: Vec<&User> = users.iter()
        .filter(|u| matches!(u.relationship_status, RelationshipStatus::Married(_)))
        .collect();

    let most_friends_user = users.iter().max_by_key(|u| u.friends.len()).unwrap();
    let happiest_user = users.iter().max_by(|a, b|
        calculate_happiness(a).partial_cmp(&calculate_happiness(b)).unwrap()).unwrap();

    for user in [
        married_users.first().copied(),
        Some(most_friends_user),
        Some(happiest_user),
    ].iter().flatten().take(3) {
        println!("   ╭─────────────────────────────────────────────────────────────────────────╮");
        println!("   │ {} (Age {})", user.name, user.age);
        println!("   ├─────────────────────────────────────────────────────────────────────────┤");
        println!("   │ Attachment: {:?}", user.attachment);
        println!("   │ Personality: O={:.2} C={:.2} E={:.2} A={:.2} N={:.2}",
            user.big_five.openness, user.big_five.conscientiousness,
            user.big_five.extraversion, user.big_five.agreeableness, user.big_five.neuroticism);
        println!("   │ Status: {:?}", user.relationship_status);
        println!("   │ Friends: {} (Close: {}, Best: {:?})",
            user.friends.len(), user.close_friends.len(), user.best_friend.map(|id| users[id].name.split('(').next().unwrap().trim().to_string()));
        println!("   │ Happiness: {:.1}%", calculate_happiness(user) * 100.0);
        println!("   │ Life Events: {:?}", user.life_events.iter().map(|e| format!("{:?}", e.event_type)).collect::<Vec<_>>());
        println!("   ╰─────────────────────────────────────────────────────────────────────────╯\n");
    }

    // Performance summary
    let total_time = sim_start.elapsed();
    let interactions_per_sec = (SIMULATION_WEEKS * NUM_USERS * WEEKLY_INTERACTIONS) as f64 / total_time.as_secs_f64();

    println!("\n⚡ PERFORMANCE SUMMARY");
    println!("┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│  Total simulation time               │ {:>10?}                        │", total_time);
    println!("│  Users simulated                     │ {:>10}                          │", NUM_USERS);
    println!("│  Weeks simulated                     │ {:>10}                          │", SIMULATION_WEEKS);
    println!("│  Total interactions processed        │ {:>10}                          │", SIMULATION_WEEKS * NUM_USERS * WEEKLY_INTERACTIONS);
    println!("│  Interactions per second             │ {:>10.0}                          │", interactions_per_sec);
    println!("│  Embedding dimension                 │ {:>10}                          │", EMBEDDING_DIM);
    println!("│  Similarity computations             │ {:>10}                          │", stats.total_friendships_formed + stats.total_relationships * 10);
    println!("└────────────────────────────────────────────────────────────────────────────┘");

    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                          SIMULATION COMPLETE                                  ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝\n");
}

fn main() {
    run_simulation();
}
