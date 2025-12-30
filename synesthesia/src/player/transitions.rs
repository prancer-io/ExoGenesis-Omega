//! Transition Engine
//!
//! Handles visual transitions between song sections, video segments,
//! and clarity levels. Beat-synchronized and musically-aware.

use crate::music::{MusicUnderstanding, SectionType};
use crate::player::synth_loader::Transition;

/// Types of visual transitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionType {
    /// Instant cut (for drops, dramatic moments)
    Cut,
    /// Smooth crossfade (default)
    Crossfade,
    /// Wipe synchronized to beat
    BeatWipe,
    /// Morphing blend (for climax moments)
    Morph,
    /// Flash to white then reveal
    FlashReveal,
    /// Zoom blur transition
    ZoomBlur,
    /// Dissolve with particles
    ParticleDissolve,
    /// Chromatic split and rejoin
    ChromaticSplit,
}

impl TransitionType {
    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "cut" => Self::Cut,
            "crossfade" => Self::Crossfade,
            "beat_wipe" | "beatwipe" => Self::BeatWipe,
            "morph" => Self::Morph,
            "flash" | "flash_reveal" => Self::FlashReveal,
            "zoom" | "zoom_blur" => Self::ZoomBlur,
            "dissolve" | "particle_dissolve" => Self::ParticleDissolve,
            "chromatic" | "chromatic_split" => Self::ChromaticSplit,
            _ => Self::Crossfade,
        }
    }

    /// Get shader-friendly enum value
    pub fn shader_id(&self) -> u32 {
        match self {
            Self::Cut => 0,
            Self::Crossfade => 1,
            Self::BeatWipe => 2,
            Self::Morph => 3,
            Self::FlashReveal => 4,
            Self::ZoomBlur => 5,
            Self::ParticleDissolve => 6,
            Self::ChromaticSplit => 7,
        }
    }
}

/// Active transition state
#[derive(Debug, Clone)]
pub struct ActiveTransition {
    /// Transition type
    pub transition_type: TransitionType,
    /// Progress (0.0 = start, 1.0 = complete)
    pub progress: f32,
    /// Total duration in seconds
    pub duration: f32,
    /// Time elapsed
    pub elapsed: f32,
    /// From segment ID
    pub from_segment: u32,
    /// To segment ID
    pub to_segment: u32,
    /// Beat-synced phase (0-1)
    pub beat_phase: f32,
}

impl ActiveTransition {
    /// Create new transition
    pub fn new(trans: &Transition) -> Self {
        Self {
            transition_type: TransitionType::from_str(&trans.transition_type),
            progress: 0.0,
            duration: trans.duration as f32,
            elapsed: 0.0,
            from_segment: trans.from_segment,
            to_segment: trans.to_segment,
            beat_phase: 0.0,
        }
    }

    /// Update transition progress
    pub fn update(&mut self, delta: f32, beat_strength: f32) {
        self.elapsed += delta;
        self.progress = (self.elapsed / self.duration).clamp(0.0, 1.0);

        // Update beat phase for beat-synced transitions
        self.beat_phase = beat_strength;
    }

    /// Is transition complete?
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }

    /// Get eased progress for smooth transitions
    pub fn eased_progress(&self) -> f32 {
        // Ease in-out cubic
        let t = self.progress;
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }

    /// Get transition shader uniforms
    pub fn get_uniforms(&self) -> TransitionUniforms {
        TransitionUniforms {
            transition_type: self.transition_type.shader_id(),
            progress: self.eased_progress(),
            beat_phase: self.beat_phase,
            from_segment: self.from_segment,
            to_segment: self.to_segment,
        }
    }
}

/// Uniforms for transition shader
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransitionUniforms {
    pub transition_type: u32,
    pub progress: f32,
    pub beat_phase: f32,
    pub from_segment: u32,
    pub to_segment: u32,
}

/// Transition Engine
/// Manages section-based and video segment transitions
pub struct TransitionEngine {
    /// Currently active transition
    active: Option<ActiveTransition>,

    /// Pending transitions from .synth file
    pending: Vec<Transition>,

    /// Current segment index
    current_segment: u32,

    /// Last section type for detecting changes
    last_section: SectionType,

    /// Auto-transition settings
    auto_transition_on_section: bool,
    auto_duration: f32,
}

impl TransitionEngine {
    /// Create new transition engine
    pub fn new() -> Self {
        Self {
            active: None,
            pending: Vec::new(),
            current_segment: 0,
            last_section: SectionType::Unknown,
            auto_transition_on_section: true,
            auto_duration: 0.5,
        }
    }

    /// Load transitions from .synth file
    pub fn load_transitions(&mut self, transitions: Vec<Transition>) {
        self.pending = transitions;
        self.pending.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    /// Update engine state
    pub fn update(&mut self, time: f64, delta: f32, music: &MusicUnderstanding) {
        // Update active transition
        if let Some(ref mut trans) = self.active {
            trans.update(delta, music.signal.beat_strength);
            if trans.is_complete() {
                self.current_segment = trans.to_segment;
                self.active = None;
            }
        }

        // Check for pending transitions
        if self.active.is_none() {
            if let Some(idx) = self.pending.iter().position(|t| t.time <= time && t.time > time - 0.1) {
                let trans = self.pending.remove(idx);
                self.active = Some(ActiveTransition::new(&trans));
            }
        }

        // Auto-transition on section change
        if self.auto_transition_on_section && self.active.is_none() {
            if music.section.section_type != self.last_section {
                self.trigger_section_transition(music);
                self.last_section = music.section.section_type;
            }
        }
    }

    /// Trigger transition for section change
    fn trigger_section_transition(&mut self, music: &MusicUnderstanding) {
        let transition_type = self.choose_transition_for_section(&music.section.section_type, music.is_climax);

        self.active = Some(ActiveTransition {
            transition_type,
            progress: 0.0,
            duration: self.auto_duration,
            elapsed: 0.0,
            from_segment: self.current_segment,
            to_segment: self.current_segment + 1,
            beat_phase: music.signal.beat_strength,
        });
    }

    /// Choose appropriate transition for section type
    fn choose_transition_for_section(&self, section: &SectionType, is_climax: bool) -> TransitionType {
        if is_climax {
            return TransitionType::FlashReveal;
        }

        match section {
            SectionType::Drop => TransitionType::Cut,
            SectionType::Chorus => TransitionType::FlashReveal,
            SectionType::Buildup => TransitionType::ZoomBlur,
            SectionType::Breakdown => TransitionType::ParticleDissolve,
            SectionType::Bridge => TransitionType::Morph,
            SectionType::Verse => TransitionType::Crossfade,
            SectionType::Outro => TransitionType::ChromaticSplit,
            _ => TransitionType::Crossfade,
        }
    }

    /// Is a transition active?
    pub fn is_transitioning(&self) -> bool {
        self.active.is_some()
    }

    /// Get active transition
    pub fn active_transition(&self) -> Option<&ActiveTransition> {
        self.active.as_ref()
    }

    /// Get current transition uniforms
    pub fn get_uniforms(&self) -> TransitionUniforms {
        self.active
            .as_ref()
            .map(|t| t.get_uniforms())
            .unwrap_or_default()
    }

    /// Get current segment
    pub fn current_segment(&self) -> u32 {
        self.current_segment
    }

    /// Manually trigger a transition
    pub fn trigger(&mut self, to_segment: u32, transition_type: TransitionType, duration: f32) {
        self.active = Some(ActiveTransition {
            transition_type,
            progress: 0.0,
            duration,
            elapsed: 0.0,
            from_segment: self.current_segment,
            to_segment,
            beat_phase: 0.0,
        });
    }

    /// Reset for new song
    pub fn reset(&mut self) {
        self.active = None;
        self.pending.clear();
        self.current_segment = 0;
        self.last_section = SectionType::Unknown;
    }
}

impl Default for TransitionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// WGSL shader code for transitions
pub const TRANSITION_SHADER: &str = r#"
// ═══════════════════════════════════════════════════════════════════
// TRANSITION SHADER
// Implements various transition effects between video segments
// ═══════════════════════════════════════════════════════════════════

struct TransitionUniforms {
    transition_type: u32,
    progress: f32,
    beat_phase: f32,
    from_segment: u32,
    to_segment: u32,
};

@group(1) @binding(0) var<uniform> trans: TransitionUniforms;
@group(1) @binding(1) var from_texture: texture_2d<f32>;
@group(1) @binding(2) var to_texture: texture_2d<f32>;
@group(1) @binding(3) var tex_sampler: sampler;

// ─────────────────────────────────────────────────────────────────
// TRANSITION FUNCTIONS
// ─────────────────────────────────────────────────────────────────

fn transition_cut(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    if (progress < 0.5) {
        return textureSample(from_texture, tex_sampler, uv);
    } else {
        return textureSample(to_texture, tex_sampler, uv);
    }
}

fn transition_crossfade(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let from_color = textureSample(from_texture, tex_sampler, uv);
    let to_color = textureSample(to_texture, tex_sampler, uv);
    return mix(from_color, to_color, progress);
}

fn transition_beat_wipe(uv: vec2<f32>, progress: f32, beat: f32) -> vec4<f32> {
    // Wipe synced to beat with pulsing edge
    let edge = progress + sin(beat * 6.28) * 0.05;
    let from_color = textureSample(from_texture, tex_sampler, uv);
    let to_color = textureSample(to_texture, tex_sampler, uv);

    if (uv.x < edge) {
        return to_color;
    } else {
        return from_color;
    }
}

fn transition_morph(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    // Distort UVs during transition
    let distort = sin(progress * 3.14159) * 0.1;
    let from_uv = uv + vec2<f32>(distort * sin(uv.y * 10.0), 0.0);
    let to_uv = uv - vec2<f32>(distort * sin(uv.y * 10.0), 0.0);

    let from_color = textureSample(from_texture, tex_sampler, from_uv);
    let to_color = textureSample(to_texture, tex_sampler, to_uv);

    return mix(from_color, to_color, progress);
}

fn transition_flash_reveal(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let from_color = textureSample(from_texture, tex_sampler, uv);
    let to_color = textureSample(to_texture, tex_sampler, uv);

    // Flash white at midpoint
    let flash = smoothstep(0.4, 0.5, progress) * smoothstep(0.6, 0.5, progress);

    var result = mix(from_color, to_color, smoothstep(0.3, 0.7, progress));
    result = mix(result, vec4<f32>(1.0), flash * 0.8);

    return result;
}

fn transition_zoom_blur(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let blur_amount = sin(progress * 3.14159) * 0.05;

    var from_color = vec4<f32>(0.0);
    var to_color = vec4<f32>(0.0);

    // Radial blur
    for (var i = 0; i < 8; i = i + 1) {
        let t = f32(i) / 8.0;
        let offset = (uv - center) * blur_amount * t;
        from_color = from_color + textureSample(from_texture, tex_sampler, uv + offset);
        to_color = to_color + textureSample(to_texture, tex_sampler, uv - offset);
    }
    from_color = from_color / 8.0;
    to_color = to_color / 8.0;

    return mix(from_color, to_color, progress);
}

fn transition_chromatic_split(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let split = sin(progress * 3.14159) * 0.03;

    let from_r = textureSample(from_texture, tex_sampler, uv + vec2<f32>(split, 0.0)).r;
    let from_g = textureSample(from_texture, tex_sampler, uv).g;
    let from_b = textureSample(from_texture, tex_sampler, uv - vec2<f32>(split, 0.0)).b;

    let to_r = textureSample(to_texture, tex_sampler, uv - vec2<f32>(split, 0.0)).r;
    let to_g = textureSample(to_texture, tex_sampler, uv).g;
    let to_b = textureSample(to_texture, tex_sampler, uv + vec2<f32>(split, 0.0)).b;

    let from_color = vec4<f32>(from_r, from_g, from_b, 1.0);
    let to_color = vec4<f32>(to_r, to_g, to_b, 1.0);

    return mix(from_color, to_color, progress);
}

// ─────────────────────────────────────────────────────────────────
// MAIN TRANSITION
// ─────────────────────────────────────────────────────────────────

fn apply_transition(uv: vec2<f32>) -> vec4<f32> {
    switch (trans.transition_type) {
        case 0u: { return transition_cut(uv, trans.progress); }
        case 1u: { return transition_crossfade(uv, trans.progress); }
        case 2u: { return transition_beat_wipe(uv, trans.progress, trans.beat_phase); }
        case 3u: { return transition_morph(uv, trans.progress); }
        case 4u: { return transition_flash_reveal(uv, trans.progress); }
        case 5u: { return transition_zoom_blur(uv, trans.progress); }
        case 7u: { return transition_chromatic_split(uv, trans.progress); }
        default: { return transition_crossfade(uv, trans.progress); }
    }
}
"#;
