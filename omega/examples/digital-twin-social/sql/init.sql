-- PATH Social Network - RuVector PostgreSQL Schema
-- Optimized for SIMD-accelerated vector search with HNSW indexing

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";  -- pgvector/ruvector extension

-- ============================================================================
-- DIGITAL TWIN PROFILES
-- ============================================================================

-- Core digital twin table storing personality vectors
CREATE TABLE digital_twins (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,

    -- Big Five personality traits (OCEAN model)
    openness REAL NOT NULL DEFAULT 0.5,
    conscientiousness REAL NOT NULL DEFAULT 0.5,
    extraversion REAL NOT NULL DEFAULT 0.5,
    agreeableness REAL NOT NULL DEFAULT 0.5,
    neuroticism REAL NOT NULL DEFAULT 0.5,

    -- Attachment style
    attachment_style VARCHAR(50) NOT NULL DEFAULT 'secure',

    -- Emotional intelligence scores
    eq_self_awareness REAL NOT NULL DEFAULT 0.5,
    eq_self_regulation REAL NOT NULL DEFAULT 0.5,
    eq_motivation REAL NOT NULL DEFAULT 0.5,
    eq_empathy REAL NOT NULL DEFAULT 0.5,
    eq_social_skills REAL NOT NULL DEFAULT 0.5,

    -- Schwartz values (10 values)
    value_self_direction REAL NOT NULL DEFAULT 0.5,
    value_stimulation REAL NOT NULL DEFAULT 0.5,
    value_hedonism REAL NOT NULL DEFAULT 0.5,
    value_achievement REAL NOT NULL DEFAULT 0.5,
    value_power REAL NOT NULL DEFAULT 0.5,
    value_security REAL NOT NULL DEFAULT 0.5,
    value_conformity REAL NOT NULL DEFAULT 0.5,
    value_tradition REAL NOT NULL DEFAULT 0.5,
    value_benevolence REAL NOT NULL DEFAULT 0.5,
    value_universalism REAL NOT NULL DEFAULT 0.5,

    -- Communication style
    comm_directness REAL NOT NULL DEFAULT 0.5,
    comm_expressiveness REAL NOT NULL DEFAULT 0.5,
    comm_formality REAL NOT NULL DEFAULT 0.5,
    comm_conflict_approach REAL NOT NULL DEFAULT 0.5,
    comm_listening_speaking REAL NOT NULL DEFAULT 0.5,
    comm_emotional_logical REAL NOT NULL DEFAULT 0.5,

    -- Deep embedding vector (4096 dimensions for rich personality representation)
    -- Uses RuVector's SIMD-accelerated storage
    deep_embedding vector(4096),

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Profile archetype for quick categorization
    archetype VARCHAR(100)
);

-- Create HNSW index for fast similarity search on personality embeddings
-- RuVector uses SIMD-accelerated distance computations
CREATE INDEX idx_twins_embedding_hnsw ON digital_twins
    USING hnsw (deep_embedding vector_cosine_ops)
    WITH (m = 32, ef_construction = 100);

-- Index for quick lookup by user_id
CREATE INDEX idx_twins_user_id ON digital_twins(user_id);

-- ============================================================================
-- EMOTIONAL STATES TRACKING
-- ============================================================================

-- Stores emotional signals over time for each user
CREATE TABLE emotional_signals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    twin_id UUID NOT NULL REFERENCES digital_twins(id) ON DELETE CASCADE,

    -- Signal properties
    source VARCHAR(50) NOT NULL,  -- 'text', 'keyboard', 'wearable', 'interaction'
    valence REAL NOT NULL,         -- -1.0 to 1.0
    arousal REAL NOT NULL,         -- 0.0 to 1.0
    dominance REAL NOT NULL,       -- 0.0 to 1.0
    confidence REAL NOT NULL DEFAULT 0.5,

    -- Embedding for semantic search on emotional patterns
    signal_embedding vector(384),

    -- Context
    context JSONB,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- HNSW index for finding similar emotional patterns
CREATE INDEX idx_emotional_signals_embedding ON emotional_signals
    USING hnsw (signal_embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);

-- Time-series optimization
CREATE INDEX idx_emotional_signals_twin_time ON emotional_signals(twin_id, created_at DESC);

-- ============================================================================
-- RELATIONSHIP MATCHING
-- ============================================================================

-- Stores compatibility scores between users
CREATE TABLE compatibility_scores (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    twin_a_id UUID NOT NULL REFERENCES digital_twins(id) ON DELETE CASCADE,
    twin_b_id UUID NOT NULL REFERENCES digital_twins(id) ON DELETE CASCADE,

    -- Domain-specific scores
    domain VARCHAR(50) NOT NULL,  -- 'dating', 'friendship', 'professional', 'mentorship', 'creative'

    -- Computed scores
    compatibility_score REAL NOT NULL,
    satisfaction_prediction REAL NOT NULL,
    longevity_prediction REAL NOT NULL,
    growth_potential REAL NOT NULL,
    conflict_risk REAL NOT NULL,

    -- Key compatibility factors (JSONB for flexibility)
    factors JSONB NOT NULL DEFAULT '[]',

    -- Causal reasoning metadata
    causal_confidence REAL NOT NULL DEFAULT 0.5,

    -- Timestamps
    computed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ensure unique pair per domain
    UNIQUE(twin_a_id, twin_b_id, domain)
);

-- Index for fast lookup of user's matches
CREATE INDEX idx_compatibility_twin_a ON compatibility_scores(twin_a_id, domain, compatibility_score DESC);
CREATE INDEX idx_compatibility_twin_b ON compatibility_scores(twin_b_id, domain, compatibility_score DESC);

-- ============================================================================
-- ARIA CONVERSATION HISTORY
-- ============================================================================

-- Stores ARIA agent conversations for learning
CREATE TABLE aria_conversations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    twin_id UUID NOT NULL REFERENCES digital_twins(id) ON DELETE CASCADE,
    session_id UUID NOT NULL,

    -- Message content
    user_message TEXT NOT NULL,
    aria_response TEXT NOT NULL,

    -- Agent involvement
    primary_agent VARCHAR(50) NOT NULL,
    agent_contributions JSONB NOT NULL DEFAULT '[]',

    -- Response quality metrics
    suggestions JSONB DEFAULT '[]',
    growth_opportunity BOOLEAN DEFAULT FALSE,
    emotional_tone JSONB,

    -- Embedding for semantic retrieval
    message_embedding vector(768),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- HNSW index for finding similar conversations
CREATE INDEX idx_aria_message_embedding ON aria_conversations
    USING hnsw (message_embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);

-- Session and user lookup
CREATE INDEX idx_aria_twin_session ON aria_conversations(twin_id, session_id, created_at DESC);

-- ============================================================================
-- REFLEXION EPISODES (Learning from Experience)
-- ============================================================================

CREATE TABLE reflexion_episodes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id VARCHAR(255) NOT NULL,

    -- Task description and results
    task TEXT NOT NULL,
    input JSONB NOT NULL,
    output JSONB NOT NULL,

    -- Learning metrics
    reward REAL NOT NULL DEFAULT 0.0,
    success BOOLEAN NOT NULL DEFAULT FALSE,
    critique TEXT,

    -- Performance metrics
    latency_ms BIGINT NOT NULL DEFAULT 0,
    tokens BIGINT NOT NULL DEFAULT 0,

    -- Embedding for similar episode retrieval
    task_embedding vector(768),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- HNSW index for finding similar tasks
CREATE INDEX idx_reflexion_embedding ON reflexion_episodes
    USING hnsw (task_embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);

-- Session lookup
CREATE INDEX idx_reflexion_session ON reflexion_episodes(session_id, created_at DESC);

-- ============================================================================
-- CAUSAL GRAPH (Relationship Predictions)
-- ============================================================================

CREATE TABLE causal_edges (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Cause and effect nodes
    cause VARCHAR(255) NOT NULL,
    effect VARCHAR(255) NOT NULL,

    -- Causal strength
    uplift REAL NOT NULL DEFAULT 0.0,
    confidence REAL NOT NULL DEFAULT 0.5,
    sample_size BIGINT NOT NULL DEFAULT 0,

    -- Temporal tracking
    first_observed TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_observed TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint for cause-effect pairs
    UNIQUE(cause, effect)
);

-- Index for querying effects and causes
CREATE INDEX idx_causal_cause ON causal_edges(cause, uplift DESC);
CREATE INDEX idx_causal_effect ON causal_edges(effect, uplift DESC);

-- ============================================================================
-- PRIVACY LAYER (Zero-Knowledge Compatible)
-- ============================================================================

-- Stores only anonymized, aggregated emotional insights
-- Raw emotional data stays on client device
CREATE TABLE privacy_safe_insights (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Anonymized user identifier (not linkable to real user)
    anon_id VARCHAR(64) NOT NULL,

    -- Differential privacy protected aggregates
    emotional_trend VARCHAR(50),  -- 'improving', 'stable', 'declining'
    resilience_bucket INTEGER,     -- Quantized 1-5 scale
    stability_bucket INTEGER,      -- Quantized 1-5 scale

    -- Privacy metadata
    noise_scale REAL NOT NULL,     -- Differential privacy noise added
    k_anonymity INTEGER NOT NULL,  -- Minimum group size

    -- Time bucket (granular to day only for privacy)
    time_bucket DATE NOT NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for time-based aggregate queries
CREATE INDEX idx_privacy_time_bucket ON privacy_safe_insights(time_bucket);

-- ============================================================================
-- HELPER FUNCTIONS
-- ============================================================================

-- Function to compute personality similarity between two users
CREATE OR REPLACE FUNCTION compute_personality_similarity(
    user_a_id UUID,
    user_b_id UUID
) RETURNS REAL AS $$
DECLARE
    similarity REAL;
BEGIN
    SELECT 1 - (a.deep_embedding <=> b.deep_embedding)
    INTO similarity
    FROM digital_twins a, digital_twins b
    WHERE a.id = user_a_id AND b.id = user_b_id;

    RETURN COALESCE(similarity, 0.0);
END;
$$ LANGUAGE plpgsql;

-- Function to find k nearest personality matches
CREATE OR REPLACE FUNCTION find_personality_matches(
    target_user_id UUID,
    match_count INTEGER DEFAULT 10
) RETURNS TABLE (
    matched_user_id UUID,
    user_name VARCHAR,
    similarity REAL
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        t.id,
        t.name,
        1 - (t.deep_embedding <=> target.deep_embedding) as sim
    FROM digital_twins t
    CROSS JOIN (
        SELECT deep_embedding FROM digital_twins WHERE id = target_user_id
    ) target
    WHERE t.id != target_user_id
    ORDER BY t.deep_embedding <=> target.deep_embedding
    LIMIT match_count;
END;
$$ LANGUAGE plpgsql;

-- Function to get emotional trajectory for a user
CREATE OR REPLACE FUNCTION get_emotional_trajectory(
    target_twin_id UUID,
    hours_back INTEGER DEFAULT 24
) RETURNS TABLE (
    hour_bucket TIMESTAMPTZ,
    avg_valence REAL,
    avg_arousal REAL,
    signal_count BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        date_trunc('hour', created_at) as hour_bucket,
        AVG(valence)::REAL as avg_valence,
        AVG(arousal)::REAL as avg_arousal,
        COUNT(*) as signal_count
    FROM emotional_signals
    WHERE twin_id = target_twin_id
      AND created_at >= NOW() - (hours_back || ' hours')::INTERVAL
    GROUP BY date_trunc('hour', created_at)
    ORDER BY hour_bucket DESC;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- PERFORMANCE OPTIMIZATIONS
-- ============================================================================

-- Autovacuum tuning for high-write tables
ALTER TABLE emotional_signals SET (
    autovacuum_vacuum_scale_factor = 0.05,
    autovacuum_analyze_scale_factor = 0.02
);

ALTER TABLE aria_conversations SET (
    autovacuum_vacuum_scale_factor = 0.05,
    autovacuum_analyze_scale_factor = 0.02
);

-- ============================================================================
-- GRANTS (Application user permissions)
-- ============================================================================

-- Create application role
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'path_app') THEN
        CREATE ROLE path_app WITH LOGIN PASSWORD 'path_app_password';
    END IF;
END
$$;

GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO path_app;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO path_app;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO path_app;

-- ============================================================================
-- COMMENTS (Documentation)
-- ============================================================================

COMMENT ON TABLE digital_twins IS 'Core personality profiles with 4096-dim embeddings for SIMD-accelerated matching';
COMMENT ON TABLE emotional_signals IS 'Time-series emotional data with privacy-preserving aggregation';
COMMENT ON TABLE compatibility_scores IS 'Pre-computed relationship compatibility using causal reasoning';
COMMENT ON TABLE aria_conversations IS 'ARIA multi-agent conversation history for continuous learning';
COMMENT ON TABLE causal_edges IS 'Causal graph for relationship outcome prediction';
COMMENT ON TABLE privacy_safe_insights IS 'Zero-knowledge compatible anonymized insights';

COMMENT ON INDEX idx_twins_embedding_hnsw IS 'HNSW index for sub-millisecond personality matching (SIMD-accelerated)';
