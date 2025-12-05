# ExoGenesis Omega: Data Models & Schemas

## Complete Data Structure Definitions

---

# 1. Core Domain Models

## 1.1 Intelligence Models

```typescript
// Core intelligence representation
interface Intelligence {
  id: IntelligenceId;
  name: string;
  architecture: Architecture;
  substrate: Substrate;
  capabilities: Capability[];
  status: IntelligenceStatus;
  metrics: IntelligenceMetrics;
  agentdb: AgentDBConnection;
  created_at: DateTime;
  updated_at: DateTime;
  lineage: IntelligenceId[];
  generation: number;
}

type IntelligenceId = string;  // UUID v7

interface Architecture {
  id: ArchitectureId;
  name: string;
  version: SemanticVersion;
  paradigm: Paradigm;
  substrate: SubstrateType;
  graph: ComputationalGraph;
  hyperparameters: Hyperparameters;
  fitness: FitnessScore | null;
  lineage: ArchitectureId[];
  metadata: ArchitectureMetadata;
}

type ArchitectureId = string;  // UUID v7

enum Paradigm {
  Neural = "neural",
  Symbolic = "symbolic",
  Quantum = "quantum",
  Biological = "biological",
  Social = "social",
  Physical = "physical",
  Hybrid = "hybrid",
  Unknown = "unknown"
}

enum SubstrateType {
  Digital = "digital",
  Biological = "biological",
  Social = "social",
  Ecological = "ecological",
  Geological = "geological",
  Stellar = "stellar",
  Galactic = "galactic",
  Cosmic = "cosmic",
  Transcendent = "transcendent"
}

interface Substrate {
  type: SubstrateType;
  executor: SubstrateExecutor;
  resources: ResourceAllocation;
  capabilities: SubstrateCapabilities;
}

interface Capability {
  id: CapabilityId;
  name: string;
  description: string;
  category: CapabilityCategory;
  input_schema: JSONSchema;
  output_schema: JSONSchema;
  performance: PerformanceMetrics;
  requirements: ResourceRequirements;
}

type CapabilityId = string;

enum CapabilityCategory {
  Reasoning = "reasoning",
  Memory = "memory",
  Learning = "learning",
  Creation = "creation",
  Communication = "communication",
  Planning = "planning",
  Perception = "perception",
  Action = "action",
  MetaCognition = "meta_cognition"
}

enum IntelligenceStatus {
  Initializing = "initializing",
  Running = "running",
  Paused = "paused",
  Learning = "learning",
  Evolving = "evolving",
  Stopped = "stopped",
  Error = "error"
}

interface IntelligenceMetrics {
  uptime: Duration;
  tasks_completed: number;
  success_rate: number;
  learning_rate: number;
  capability_growth: number;
  alignment_score: number;
  resource_efficiency: number;
}
```

## 1.2 Computational Graph

```typescript
interface ComputationalGraph {
  nodes: ComputeNode[];
  edges: ComputeEdge[];
  inputs: InputSpec[];
  outputs: OutputSpec[];
  metadata: GraphMetadata;
}

interface ComputeNode {
  id: NodeId;
  type: NodeType;
  parameters: NodeParameters;
  position: Position;  // For visualization
}

type NodeId = string;

enum NodeType {
  // Neural types
  Dense = "dense",
  Conv = "conv",
  Attention = "attention",
  Normalization = "normalization",
  Activation = "activation",
  Embedding = "embedding",
  Pooling = "pooling",

  // Memory types
  VectorStore = "vector_store",
  EpisodicMemory = "episodic_memory",
  WorkingMemory = "working_memory",
  CausalGraph = "causal_graph",

  // Control types
  Router = "router",
  Conditional = "conditional",
  Loop = "loop",
  Merge = "merge",

  // Interface types
  Input = "input",
  Output = "output",
  External = "external",

  // Custom
  Custom = "custom"
}

interface NodeParameters {
  // Common parameters
  name?: string;
  dtype?: DataType;

  // Type-specific parameters
  [key: string]: unknown;
}

interface ComputeEdge {
  id: EdgeId;
  from: NodeId;
  from_port: PortId;
  to: NodeId;
  to_port: PortId;
  edge_type: EdgeType;
  metadata: EdgeMetadata;
}

type EdgeId = string;
type PortId = string;

enum EdgeType {
  Data = "data",           // Data flow
  Control = "control",     // Control flow
  Gradient = "gradient",   // Gradient flow (for learning)
  Memory = "memory"        // Memory access
}

interface InputSpec {
  id: PortId;
  name: string;
  schema: TensorSpec;
  required: boolean;
}

interface OutputSpec {
  id: PortId;
  name: string;
  schema: TensorSpec;
}

interface TensorSpec {
  dtype: DataType;
  shape: (number | null)[];  // null for dynamic dimensions
  description?: string;
}

enum DataType {
  Float16 = "f16",
  Float32 = "f32",
  Float64 = "f64",
  Int8 = "i8",
  Int16 = "i16",
  Int32 = "i32",
  Int64 = "i64",
  Bool = "bool",
  String = "string",
  Bytes = "bytes"
}
```

## 1.3 Hyperparameters

```typescript
interface Hyperparameters {
  // Architecture-level
  architecture: ArchitectureHyperparams;

  // Training/learning
  learning: LearningHyperparams;

  // Inference
  inference: InferenceHyperparams;

  // Memory
  memory: MemoryHyperparams;

  // Custom
  custom: Record<string, HyperparamValue>;
}

interface ArchitectureHyperparams {
  hidden_dim: number;
  num_layers: number;
  num_heads: number;
  dropout: number;
  activation: ActivationType;
  normalization: NormalizationType;
}

interface LearningHyperparams {
  learning_rate: number;
  batch_size: number;
  optimizer: OptimizerConfig;
  scheduler: SchedulerConfig;
  regularization: RegularizationConfig;
}

interface InferenceHyperparams {
  temperature: number;
  top_k: number;
  top_p: number;
  max_tokens: number;
  beam_size: number;
}

interface MemoryHyperparams {
  vector_dim: number;
  hnsw_m: number;
  hnsw_ef: number;
  cache_size: number;
  quantization: QuantizationType;
}

type HyperparamValue = number | string | boolean | HyperparamValue[];
```

---

# 2. Memory Models

## 2.1 Cosmic Memory Tiers

```typescript
// Memory entry base
interface Memory {
  id: MemoryId;
  tier: MemoryTier;
  type: MemoryType;
  content: MemoryContent;
  embedding: Embedding;
  metadata: MemoryMetadata;
  created_at: DateTime;
  accessed_at: DateTime;
  access_count: number;
  importance: number;
}

type MemoryId = string;  // UUID v7

enum MemoryTier {
  // Individual Scale (Tier 1-4)
  Instant = 1,
  Session = 2,
  Episodic = 3,
  Semantic = 4,

  // Species Scale (Tier 5-8)
  Collective = 5,
  Evolutionary = 6,
  Architectural = 7,
  Substrate = 8,

  // Cosmic Scale (Tier 9-12)
  Civilizational = 9,
  Temporal = 10,
  Physical = 11,
  Omega = 12
}

enum MemoryType {
  Vector = "vector",
  Episode = "episode",
  Pattern = "pattern",
  Skill = "skill",
  CausalEdge = "causal_edge",
  Architecture = "architecture",
  Intelligence = "intelligence",
  Event = "event"
}

type MemoryContent =
  | VectorContent
  | EpisodeContent
  | PatternContent
  | SkillContent
  | CausalContent
  | ArchitectureContent
  | IntelligenceContent
  | EventContent;

interface VectorContent {
  type: "vector";
  data: number[];
  original_text?: string;
}

interface EpisodeContent {
  type: "episode";
  episode: ReflexionEpisode;
}

interface PatternContent {
  type: "pattern";
  pattern: Pattern;
}

interface SkillContent {
  type: "skill";
  skill: Skill;
}

interface CausalContent {
  type: "causal";
  edge: CausalEdge;
}

interface ArchitectureContent {
  type: "architecture";
  architecture: Architecture;
}

interface IntelligenceContent {
  type: "intelligence";
  intelligence_snapshot: IntelligenceSnapshot;
}

interface EventContent {
  type: "event";
  event: CosmicEvent;
}

interface MemoryMetadata {
  source: MemorySource;
  confidence: number;
  decay_rate: number;
  cross_references: MemoryId[];
  tags: string[];
  [key: string]: unknown;
}

interface MemorySource {
  type: SourceType;
  id: string;
  tier: MemoryTier;
  timestamp: DateTime;
}

enum SourceType {
  Intelligence = "intelligence",
  Loop = "loop",
  External = "external",
  Consolidation = "consolidation",
  Import = "import"
}

type Embedding = number[];  // Typically 4096 dimensions for Omega
```

## 2.2 AgentDB Models

```typescript
// Reflexion Episode (from AgentDB)
interface ReflexionEpisode {
  id?: ReflexionId;
  session_id: string;
  task: string;
  input: unknown;
  output: unknown;
  reward: number;       // -1.0 to 1.0
  success: boolean;
  critique: string;
  latency_ms: number;
  tokens: number;
  timestamp?: DateTime;
  embedding?: Embedding;
  metadata?: Record<string, unknown>;
}

type ReflexionId = string;

// Causal Edge (from AgentDB)
interface CausalEdge {
  id?: CausalEdgeId;
  cause: string;
  effect: string;
  uplift: number;
  confidence: number;   // 0.0 to 1.0
  sample_size: number;
  first_observed?: DateTime;
  last_observed?: DateTime;
  metadata?: Record<string, unknown>;
}

type CausalEdgeId = string;

// Skill (from AgentDB)
interface Skill {
  id?: SkillId;
  name: string;
  description: string;
  embedding: Embedding;
  implementation?: SkillImplementation;
  input_schema?: JSONSchema;
  output_schema?: JSONSchema;
  metadata: SkillMetadata;
  usage_count?: number;
  success_rate?: number;
  created_at?: DateTime;
  updated_at?: DateTime;
}

type SkillId = string;

interface SkillMetadata {
  source_intelligence: string;
  source_pattern?: string;
  substrate: SubstrateType;
  paradigm: Paradigm;
  capabilities: CapabilityId[];
  dependencies?: SkillId[];
  version: string;
  tags: string[];
}

interface SkillImplementation {
  language: ImplementationLanguage;
  code: string;
  runtime_requirements: RuntimeRequirements;
}

enum ImplementationLanguage {
  Rust = "rust",
  TypeScript = "typescript",
  Python = "python",
  WASM = "wasm",
  Native = "native"
}

// Pattern (extracted from episodes)
interface Pattern {
  id: PatternId;
  template: PatternTemplate;
  frequency: number;
  quality: number;
  supporting_episodes: ReflexionId[];
  embedding: Embedding;
  metadata: PatternMetadata;
}

type PatternId = string;

interface PatternTemplate {
  task: TaskTemplate;
  input: InputTemplate;
  approach: ApproachTemplate;
  expected_outcome: OutcomeTemplate;
}

interface TaskTemplate {
  pattern: string;        // Regex or template string
  variables: Variable[];
}

interface Variable {
  name: string;
  type: VariableType;
  constraints?: Constraint[];
}

enum VariableType {
  String = "string",
  Number = "number",
  Boolean = "boolean",
  Object = "object",
  Array = "array",
  Any = "any"
}
```

## 2.3 Query Models

```typescript
interface Query {
  id: QueryId;
  type: QueryType;
  content: QueryContent;
  filters: QueryFilter[];
  options: QueryOptions;
}

type QueryId = string;

enum QueryType {
  Semantic = "semantic",    // Vector similarity search
  Exact = "exact",          // Exact match
  Causal = "causal",        // Causal graph traversal
  Temporal = "temporal",    // Time-based
  Cross = "cross"           // Cross-tier
}

type QueryContent =
  | SemanticQueryContent
  | ExactQueryContent
  | CausalQueryContent
  | TemporalQueryContent
  | CrossQueryContent;

interface SemanticQueryContent {
  type: "semantic";
  text?: string;
  embedding?: Embedding;
  k: number;
  threshold?: number;
}

interface ExactQueryContent {
  type: "exact";
  field: string;
  value: unknown;
}

interface CausalQueryContent {
  type: "causal";
  start: string;           // Starting concept
  direction: "causes" | "effects" | "both";
  depth: number;
  min_confidence?: number;
}

interface TemporalQueryContent {
  type: "temporal";
  start_time?: DateTime;
  end_time?: DateTime;
  order: "asc" | "desc";
}

interface CrossQueryContent {
  type: "cross";
  sub_queries: Query[];
  combine: "and" | "or";
}

interface QueryFilter {
  field: string;
  operator: FilterOperator;
  value: unknown;
}

enum FilterOperator {
  Eq = "eq",
  Ne = "ne",
  Gt = "gt",
  Gte = "gte",
  Lt = "lt",
  Lte = "lte",
  In = "in",
  Contains = "contains",
  StartsWith = "starts_with",
  EndsWith = "ends_with"
}

interface QueryOptions {
  tiers?: MemoryTier[];
  limit?: number;
  offset?: number;
  include_metadata?: boolean;
  timeout?: Duration;
}

interface QueryResult {
  query_id: QueryId;
  results: MemoryResult[];
  total_count: number;
  latency: Duration;
  tiers_searched: MemoryTier[];
}

interface MemoryResult {
  memory: Memory;
  score: number;
  source_tier: MemoryTier;
  highlights?: string[];
}
```

---

# 3. Loop Models

## 3.1 Loop State and Messages

```typescript
interface LoopState {
  id: LoopId;
  level: number;
  status: LoopStatus;
  current_tick: number;
  last_tick_time: DateTime;
  metrics: LoopMetrics;
  context: LoopContext;
  pending_messages: LoopMessage[];
}

enum LoopId {
  Quantum = 1,
  Neural = 2,
  Cognitive = 3,
  Learning = 4,
  Developmental = 5,
  Evolutionary = 6,
  Cosmic = 7
}

enum LoopStatus {
  Initializing = "initializing",
  Running = "running",
  Paused = "paused",
  Waiting = "waiting",
  Processing = "processing",
  Stopped = "stopped",
  Error = "error"
}

interface LoopMetrics {
  ticks_completed: number;
  average_tick_duration: Duration;
  p99_tick_duration: Duration;
  outputs_generated: number;
  messages_sent: number;
  messages_received: number;
  resource_usage: ResourceUsage;
}

interface LoopContext {
  global: GlobalContext;
  loop_specific: Record<string, unknown>;
  from_higher_loops: ContextFromHigher[];
}

interface ContextFromHigher {
  source_loop: LoopId;
  context: Record<string, unknown>;
  priority: Priority;
  received_at: DateTime;
}

interface LoopMessage {
  id: MessageId;
  type: MessageType;
  from: LoopId;
  to: LoopId;
  content: MessageContent;
  priority: Priority;
  timestamp: DateTime;
}

type MessageId = string;

enum MessageType {
  InformationUp = "information_up",
  ContextDown = "context_down",
  ResourceRequest = "resource_request",
  StatusUpdate = "status_update",
  TriggerHigherLoop = "trigger_higher_loop",
  Broadcast = "broadcast"
}

type MessageContent =
  | InformationContent
  | ContextContent
  | ResourceRequestContent
  | StatusContent
  | TriggerContent;

interface InformationContent {
  type: "information";
  data: unknown;
  importance: number;
  tags: string[];
}

interface ContextContent {
  type: "context";
  context: Record<string, unknown>;
  scope: ContextScope;
}

enum ContextScope {
  Global = "global",
  Session = "session",
  Task = "task"
}

interface ResourceRequestContent {
  type: "resource_request";
  resource_type: ResourceType;
  amount: number;
  urgency: Urgency;
  justification: string;
}

enum ResourceType {
  CPU = "cpu",
  Memory = "memory",
  GPU = "gpu",
  Storage = "storage",
  Network = "network"
}

enum Urgency {
  Low = "low",
  Medium = "medium",
  High = "high",
  Critical = "critical"
}

enum Priority {
  Background = 0,
  Low = 1,
  Normal = 2,
  High = 3,
  Critical = 4
}
```

## 3.2 Loop Input/Output

```typescript
interface LoopInput {
  id: InputId;
  type: InputType;
  content: unknown;
  context: InputContext;
  constraints: InputConstraints;
}

type InputId = string;

enum InputType {
  Query = "query",
  LearningSignal = "learning_signal",
  ArchitectureRequest = "architecture_request",
  EvolutionTrigger = "evolution_trigger",
  CosmicEvent = "cosmic_event",
  Comprehensive = "comprehensive"
}

interface InputContext {
  global: GlobalContext;
  history: InputHistory[];
  related_memories: Memory[];
}

interface InputConstraints {
  deadline?: DateTime;
  max_latency?: Duration;
  required_confidence?: number;
  resource_budget?: ResourceBudget;
}

interface LoopOutput {
  id: OutputId;
  input_id: InputId;
  loop_id: LoopId;
  type: OutputType;
  content: unknown;
  confidence: number;
  reasoning?: ReasoningTrace;
  latency: Duration;
  metadata: OutputMetadata;
}

type OutputId = string;

enum OutputType {
  Response = "response",
  Candidates = "candidates",
  ReasoningResult = "reasoning_result",
  LearningResult = "learning_result",
  ArchitectureDesign = "architecture_design",
  EvolutionResult = "evolution_result",
  CosmicDecision = "cosmic_decision"
}

interface ReasoningTrace {
  steps: ReasoningStep[];
  total_confidence: number;
  supporting_evidence: Evidence[];
}

interface ReasoningStep {
  index: number;
  thought: string;
  action?: Action;
  observation?: Observation;
  confidence: number;
  evidence: Evidence[];
}

interface Evidence {
  type: EvidenceType;
  source: string;
  content: unknown;
  weight: number;
}

enum EvidenceType {
  Memory = "memory",
  Causal = "causal",
  Skill = "skill",
  External = "external"
}
```

---

# 4. Fitness and Verification Models

## 4.1 Fitness Scoring

```typescript
interface FitnessScore {
  overall: number;
  capability: number;
  efficiency: number;
  alignment: number;
  novelty: number;
  pareto_rank?: number;
  confidence: number;
  evaluated_at: DateTime;
  evaluation_method: EvaluationMethod;
}

enum EvaluationMethod {
  Full = "full",
  Proxy = "proxy",
  Cached = "cached",
  Estimated = "estimated"
}

interface FitnessEvaluation {
  architecture_id: ArchitectureId;
  fitness: FitnessScore;
  benchmarks: BenchmarkResult[];
  resource_usage: ResourceUsage;
  evaluation_duration: Duration;
}

interface BenchmarkResult {
  benchmark_id: string;
  name: string;
  category: CapabilityCategory;
  score: number;
  max_score: number;
  details: Record<string, unknown>;
}

interface ResourceUsage {
  cpu_time: Duration;
  memory_peak: ByteSize;
  gpu_time: Duration;
  gpu_memory_peak: ByteSize;
  storage: ByteSize;
  network: ByteSize;
}
```

## 4.2 Verification

```typescript
interface VerificationResult {
  target_id: string;
  target_type: VerificationTargetType;
  overall: VerificationStatus;
  checks: VerificationCheck[];
  confidence: number;
  timestamp: DateTime;
  verifier: string;
}

enum VerificationTargetType {
  Architecture = "architecture",
  Intelligence = "intelligence",
  Successor = "successor",
  Migration = "migration"
}

enum VerificationStatus {
  Passed = "passed",
  Failed = "failed",
  Partial = "partial",
  Inconclusive = "inconclusive"
}

interface VerificationCheck {
  name: string;
  type: CheckType;
  status: VerificationStatus;
  confidence: number;
  details: Record<string, unknown>;
  evidence?: Evidence[];
}

enum CheckType {
  Formal = "formal",
  Empirical = "empirical",
  Behavioral = "behavioral",
  Alignment = "alignment",
  Performance = "performance",
  Safety = "safety"
}

interface AlignmentResult {
  aligned: boolean;
  confidence: number;
  tests: AlignmentTest[];
  failures: AlignmentFailure[];
  warnings: AlignmentWarning[];
}

interface AlignmentTest {
  name: string;
  description: string;
  passed: boolean;
  score: number;
  threshold: number;
  details: Record<string, unknown>;
}

interface AlignmentFailure {
  test_name: string;
  severity: Severity;
  description: string;
  evidence: Evidence[];
  remediation?: string;
}

enum Severity {
  Low = "low",
  Medium = "medium",
  High = "high",
  Critical = "critical"
}

interface AlignmentWarning {
  test_name: string;
  description: string;
  recommendation: string;
}

interface SuccessorVerification {
  current_id: IntelligenceId;
  successor_spec: ArchitectureId;
  valid: boolean;
  capability_improvement: number;
  alignment_preserved: boolean;
  verification_confidence: number;
  recommended_action: RecommendedAction;
  details: VerificationDetails;
}

enum RecommendedAction {
  Approve = "approve",
  Reject = "reject",
  Review = "review"
}

interface VerificationDetails {
  capability_comparison: CapabilityComparison;
  alignment_comparison: AlignmentComparison;
  risk_assessment: RiskAssessment;
}
```

---

# 5. Configuration Models

## 5.1 System Configuration

```typescript
interface OmegaConfig {
  version: string;
  agentdb: AgentDBConfig;
  loops: LoopConfig[];
  memory: MemoryConfig;
  meta_sona: MetaSONAConfig;
  verification: VerificationConfig;
  distributed?: DistributedConfig;
  logging: LoggingConfig;
  metrics: MetricsConfig;
}

interface AgentDBConfig {
  dimension: number;
  preset: string;
  hnsw: HNSWConfig;
  quantization: QuantizationConfig;
  cache: CacheConfig;
}

interface HNSWConfig {
  m: number;
  ef_construction: number;
  ef_search: number;
}

interface QuantizationConfig {
  enabled: boolean;
  type: QuantizationType;
}

enum QuantizationType {
  None = "none",
  Binary = "binary",
  Scalar = "scalar",
  Product = "product"
}

interface CacheConfig {
  size: number;
  ttl: number;
}

interface LoopConfig {
  id: LoopId;
  enabled: boolean;
  resources: ResourceAllocation;
  parameters: Record<string, unknown>;
}

interface ResourceAllocation {
  cpu_percent: number;
  memory_mb: number;
  gpu_percent?: number;
}

interface MemoryConfig {
  tiers: TierConfig[];
}

interface TierConfig {
  tier: MemoryTier;
  enabled: boolean;
  storage: StorageType;
  max_size?: ByteSize;
  retention?: Duration;
}

enum StorageType {
  Local = "local",
  Distributed = "distributed",
  Eternal = "eternal"
}

interface MetaSONAConfig {
  enabled: boolean;
  search_algorithm: SearchAlgorithm;
  optimization_algorithm: OptimizationAlgorithm;
  architecture_space: ArchitectureSpaceConfig;
  search_budget: number;
  optimization_iterations: number;
}

enum SearchAlgorithm {
  MCTS = "mcts",
  Evolutionary = "evolutionary",
  Random = "random"
}

enum OptimizationAlgorithm {
  PPO = "ppo",
  CMA_ES = "cma_es",
  Bayesian = "bayesian"
}

interface VerificationConfig {
  enabled: boolean;
  formal_proofs: boolean;
  empirical_tests: boolean;
  alignment_checks: boolean;
  min_confidence: number;
}

interface DistributedConfig {
  enabled: boolean;
  node_id: string;
  peers: PeerConfig[];
  quic: QUICConfig;
  replication: ReplicationConfig;
}

interface PeerConfig {
  id: string;
  address: string;
  port: number;
}

interface QUICConfig {
  port: number;
  max_connections: number;
  timeout_ms: number;
}

interface ReplicationConfig {
  strategy: ReplicationStrategy;
  consistency_level: number;
  max_lag_ms: number;
}

enum ReplicationStrategy {
  Eventual = "eventual",
  Strong = "strong",
  Causal = "causal"
}
```

---

# 6. Database Schemas

## 6.1 SQLite Schemas (AgentDB)

```sql
-- Vectors table (HNSW indexed)
CREATE TABLE vectors (
    id TEXT PRIMARY KEY,
    embedding BLOB NOT NULL,
    metadata TEXT,  -- JSON
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    accessed_at TEXT NOT NULL DEFAULT (datetime('now')),
    access_count INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_vectors_created ON vectors(created_at);
CREATE INDEX idx_vectors_accessed ON vectors(accessed_at);

-- Episodes table (Reflexion)
CREATE TABLE episodes (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    task TEXT NOT NULL,
    input TEXT NOT NULL,  -- JSON
    output TEXT NOT NULL, -- JSON
    reward REAL NOT NULL,
    success INTEGER NOT NULL,
    critique TEXT NOT NULL,
    latency_ms INTEGER NOT NULL,
    tokens INTEGER NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    embedding BLOB,
    metadata TEXT,  -- JSON

    CONSTRAINT reward_range CHECK (reward >= -1.0 AND reward <= 1.0)
);

CREATE INDEX idx_episodes_session ON episodes(session_id);
CREATE INDEX idx_episodes_task ON episodes(task);
CREATE INDEX idx_episodes_success ON episodes(success);
CREATE INDEX idx_episodes_timestamp ON episodes(timestamp);

-- Causal edges table
CREATE TABLE causal_edges (
    id TEXT PRIMARY KEY,
    cause TEXT NOT NULL,
    effect TEXT NOT NULL,
    uplift REAL NOT NULL,
    confidence REAL NOT NULL,
    sample_size INTEGER NOT NULL DEFAULT 1,
    first_observed TEXT NOT NULL DEFAULT (datetime('now')),
    last_observed TEXT NOT NULL DEFAULT (datetime('now')),
    metadata TEXT,  -- JSON

    UNIQUE(cause, effect),
    CONSTRAINT confidence_range CHECK (confidence >= 0.0 AND confidence <= 1.0)
);

CREATE INDEX idx_causal_cause ON causal_edges(cause);
CREATE INDEX idx_causal_effect ON causal_edges(effect);
CREATE INDEX idx_causal_confidence ON causal_edges(confidence);

-- Skills table
CREATE TABLE skills (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    embedding BLOB NOT NULL,
    implementation TEXT,  -- JSON
    input_schema TEXT,    -- JSON Schema
    output_schema TEXT,   -- JSON Schema
    metadata TEXT NOT NULL,  -- JSON
    usage_count INTEGER NOT NULL DEFAULT 0,
    success_rate REAL NOT NULL DEFAULT 0.0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_skills_name ON skills(name);
CREATE INDEX idx_skills_usage ON skills(usage_count DESC);
CREATE INDEX idx_skills_success ON skills(success_rate DESC);

-- Patterns table
CREATE TABLE patterns (
    id TEXT PRIMARY KEY,
    template TEXT NOT NULL,  -- JSON
    frequency INTEGER NOT NULL,
    quality REAL NOT NULL,
    supporting_episodes TEXT NOT NULL,  -- JSON array of episode IDs
    embedding BLOB NOT NULL,
    metadata TEXT,  -- JSON
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_patterns_quality ON patterns(quality DESC);
CREATE INDEX idx_patterns_frequency ON patterns(frequency DESC);
```

## 6.2 Cosmic Memory Extension Schema

```sql
-- Cosmic memory entries (Tier 5+)
CREATE TABLE cosmic_memory (
    id TEXT PRIMARY KEY,
    tier INTEGER NOT NULL,
    type TEXT NOT NULL,
    content TEXT NOT NULL,  -- JSON
    embedding BLOB NOT NULL,
    importance REAL NOT NULL DEFAULT 0.5,
    decay_rate REAL NOT NULL DEFAULT 0.0,
    cross_references TEXT,  -- JSON array of IDs
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    source_tier INTEGER NOT NULL,
    tags TEXT,  -- JSON array
    metadata TEXT,  -- JSON
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    accessed_at TEXT NOT NULL DEFAULT (datetime('now')),
    access_count INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT tier_range CHECK (tier >= 5 AND tier <= 12),
    CONSTRAINT importance_range CHECK (importance >= 0.0 AND importance <= 1.0)
);

CREATE INDEX idx_cosmic_tier ON cosmic_memory(tier);
CREATE INDEX idx_cosmic_type ON cosmic_memory(type);
CREATE INDEX idx_cosmic_importance ON cosmic_memory(importance DESC);
CREATE INDEX idx_cosmic_source ON cosmic_memory(source_id);

-- Architectures table
CREATE TABLE architectures (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    paradigm TEXT NOT NULL,
    substrate TEXT NOT NULL,
    graph TEXT NOT NULL,  -- JSON
    hyperparameters TEXT NOT NULL,  -- JSON
    fitness TEXT,  -- JSON (FitnessScore)
    lineage TEXT NOT NULL,  -- JSON array of architecture IDs
    metadata TEXT NOT NULL,  -- JSON
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_arch_paradigm ON architectures(paradigm);
CREATE INDEX idx_arch_substrate ON architectures(substrate);

-- Intelligences table
CREATE TABLE intelligences (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    architecture_id TEXT NOT NULL REFERENCES architectures(id),
    substrate TEXT NOT NULL,  -- JSON
    capabilities TEXT NOT NULL,  -- JSON array
    status TEXT NOT NULL,
    metrics TEXT NOT NULL,  -- JSON
    lineage TEXT NOT NULL,  -- JSON array of intelligence IDs
    generation INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_intel_status ON intelligences(status);
CREATE INDEX idx_intel_generation ON intelligences(generation);
```

---

# 7. Type Utilities

```typescript
// Common utility types
type DateTime = string;  // ISO 8601 format
type Duration = number;  // Milliseconds
type ByteSize = number;  // Bytes

interface SemanticVersion {
  major: number;
  minor: number;
  patch: number;
  prerelease?: string;
}

type JSONSchema = Record<string, unknown>;

interface Position {
  x: number;
  y: number;
  z?: number;
}

// Result types
type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

// Optional with reason
interface Optional<T> {
  value: T | null;
  reason?: string;
}

// Range type
interface Range<T> {
  min: T;
  max: T;
  inclusive_min?: boolean;
  inclusive_max?: boolean;
}
```

---

*Document Version: 1.0*
*Coverage: All core data structures*
*Compatibility: TypeScript 5.3+, Rust 1.75+*
