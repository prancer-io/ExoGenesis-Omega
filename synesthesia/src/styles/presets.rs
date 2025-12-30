//! Style Presets
//!
//! Pre-defined visual styles for different moods and aesthetics.

use super::{Style, ColorPalette, EffectParams, TransitionPrefs, ColorMapping};
use std::collections::HashMap;

/// Default balanced style
pub fn default_style() -> Style {
    Style {
        name: "default".to_string(),
        description: "Balanced style that works with any music".to_string(),
        palette: ColorPalette {
            primary: [220.0, 0.6, 0.5],    // Blue
            secondary: [280.0, 0.5, 0.4],  // Purple
            accent: [45.0, 0.8, 0.6],      // Gold
            background: [240.0, 0.2, 0.1], // Dark blue
            highlight: [0.0, 0.0, 1.0],    // White
            shadow: [240.0, 0.3, 0.05],    // Near black
        },
        effects: EffectParams::default(),
        transitions: TransitionPrefs::default(),
        emotion_colors: HashMap::new(),
        clarity_modifier: 1.0,
        intensity: 1.0,
    }
}

/// Neon cyberpunk style
pub fn neon_style() -> Style {
    Style {
        name: "neon".to_string(),
        description: "Vibrant neon cyberpunk aesthetic".to_string(),
        palette: ColorPalette {
            primary: [320.0, 1.0, 0.5],    // Hot pink
            secondary: [180.0, 1.0, 0.5],  // Cyan
            accent: [60.0, 1.0, 0.5],      // Yellow
            background: [260.0, 0.8, 0.08], // Deep purple
            highlight: [300.0, 0.8, 0.9],  // Light pink
            shadow: [280.0, 0.9, 0.05],    // Dark purple
        },
        effects: EffectParams {
            bloom: 0.6,
            chromatic_aberration: 0.03,
            vignette: 0.4,
            grain: 0.02,
            motion_blur: 0.1,
            contrast: 1.3,
            saturation: 1.4,
            gamma: 0.9,
            glow_radius: 1.5,
            beat_reactivity: 1.5,
        },
        transitions: TransitionPrefs {
            default_type: "flash_reveal".to_string(),
            duration: 0.3,
            beat_sync: true,
            drop_flash: 1.0,
            section_transitions: true,
        },
        emotion_colors: HashMap::new(),
        clarity_modifier: 1.2,
        intensity: 1.3,
    }
}

/// Ethereal dreamy style
pub fn ethereal_style() -> Style {
    Style {
        name: "ethereal".to_string(),
        description: "Soft, dreamy, and otherworldly".to_string(),
        palette: ColorPalette {
            primary: [200.0, 0.4, 0.7],    // Light blue
            secondary: [280.0, 0.3, 0.8],  // Lavender
            accent: [340.0, 0.3, 0.85],    // Soft pink
            background: [220.0, 0.15, 0.15], // Muted blue-grey
            highlight: [0.0, 0.0, 0.95],   // Soft white
            shadow: [230.0, 0.2, 0.1],     // Soft shadow
        },
        effects: EffectParams {
            bloom: 0.5,
            chromatic_aberration: 0.005,
            vignette: 0.2,
            grain: 0.08,
            motion_blur: 0.2,
            contrast: 0.9,
            saturation: 0.8,
            gamma: 0.95,
            glow_radius: 2.0,
            beat_reactivity: 0.7,
        },
        transitions: TransitionPrefs {
            default_type: "crossfade".to_string(),
            duration: 1.0,
            beat_sync: false,
            drop_flash: 0.3,
            section_transitions: true,
        },
        emotion_colors: HashMap::new(),
        clarity_modifier: 0.9,
        intensity: 0.7,
    }
}

/// Cinematic film style
pub fn cinematic_style() -> Style {
    Style {
        name: "cinematic".to_string(),
        description: "Film-like with dramatic lighting".to_string(),
        palette: ColorPalette {
            primary: [30.0, 0.5, 0.4],     // Warm orange
            secondary: [200.0, 0.4, 0.3],  // Teal
            accent: [45.0, 0.7, 0.6],      // Gold
            background: [20.0, 0.1, 0.08], // Dark warm
            highlight: [40.0, 0.2, 0.9],   // Warm white
            shadow: [230.0, 0.3, 0.05],    // Cool shadow
        },
        effects: EffectParams {
            bloom: 0.25,
            chromatic_aberration: 0.015,
            vignette: 0.5,
            grain: 0.1,
            motion_blur: 0.15,
            contrast: 1.15,
            saturation: 0.9,
            gamma: 0.8,
            glow_radius: 0.8,
            beat_reactivity: 0.9,
        },
        transitions: TransitionPrefs {
            default_type: "morph".to_string(),
            duration: 0.8,
            beat_sync: true,
            drop_flash: 0.5,
            section_transitions: true,
        },
        emotion_colors: HashMap::new(),
        clarity_modifier: 1.1,
        intensity: 1.0,
    }
}

/// Retro 80s style
pub fn retro_style() -> Style {
    Style {
        name: "retro".to_string(),
        description: "80s synthwave aesthetic".to_string(),
        palette: ColorPalette {
            primary: [300.0, 0.8, 0.5],    // Magenta
            secondary: [180.0, 0.9, 0.45], // Cyan
            accent: [55.0, 1.0, 0.5],      // Yellow
            background: [270.0, 0.6, 0.1], // Deep purple
            highlight: [320.0, 0.5, 0.8],  // Pink
            shadow: [280.0, 0.7, 0.05],    // Dark purple
        },
        effects: EffectParams {
            bloom: 0.7,
            chromatic_aberration: 0.04,
            vignette: 0.35,
            grain: 0.15,
            motion_blur: 0.05,
            contrast: 1.2,
            saturation: 1.3,
            gamma: 0.85,
            glow_radius: 1.8,
            beat_reactivity: 1.4,
        },
        transitions: TransitionPrefs {
            default_type: "chromatic_split".to_string(),
            duration: 0.4,
            beat_sync: true,
            drop_flash: 0.9,
            section_transitions: true,
        },
        emotion_colors: HashMap::new(),
        clarity_modifier: 1.1,
        intensity: 1.2,
    }
}

/// Minimal clean style
pub fn minimal_style() -> Style {
    Style {
        name: "minimal".to_string(),
        description: "Clean and understated".to_string(),
        palette: ColorPalette {
            primary: [0.0, 0.0, 0.8],      // Light grey
            secondary: [0.0, 0.0, 0.6],    // Medium grey
            accent: [210.0, 0.5, 0.5],     // Subtle blue
            background: [0.0, 0.0, 0.05],  // Near black
            highlight: [0.0, 0.0, 1.0],    // White
            shadow: [0.0, 0.0, 0.02],      // Black
        },
        effects: EffectParams {
            bloom: 0.1,
            chromatic_aberration: 0.0,
            vignette: 0.15,
            grain: 0.0,
            motion_blur: 0.0,
            contrast: 1.05,
            saturation: 0.3,
            gamma: 0.9,
            glow_radius: 0.5,
            beat_reactivity: 0.5,
        },
        transitions: TransitionPrefs {
            default_type: "crossfade".to_string(),
            duration: 0.6,
            beat_sync: false,
            drop_flash: 0.2,
            section_transitions: true,
        },
        emotion_colors: HashMap::new(),
        clarity_modifier: 1.0,
        intensity: 0.6,
    }
}

/// Psychedelic style
pub fn psychedelic_style() -> Style {
    let mut emotion_colors = HashMap::new();
    // Override all emotions with rainbow spectrum
    emotion_colors.insert("joy".to_string(), ColorMapping { hue: 60.0, saturation: 1.0, lightness: 0.6 });
    emotion_colors.insert("euphoria".to_string(), ColorMapping { hue: 320.0, saturation: 1.0, lightness: 0.6 });
    emotion_colors.insert("peace".to_string(), ColorMapping { hue: 160.0, saturation: 1.0, lightness: 0.5 });

    Style {
        name: "psychedelic".to_string(),
        description: "Trippy kaleidoscope visuals".to_string(),
        palette: ColorPalette {
            primary: [0.0, 1.0, 0.5],       // Full saturation red
            secondary: [120.0, 1.0, 0.5],   // Green
            accent: [240.0, 1.0, 0.5],      // Blue
            background: [280.0, 0.5, 0.1],  // Purple
            highlight: [60.0, 1.0, 0.7],    // Yellow
            shadow: [300.0, 0.8, 0.1],      // Magenta
        },
        effects: EffectParams {
            bloom: 0.8,
            chromatic_aberration: 0.05,
            vignette: 0.2,
            grain: 0.05,
            motion_blur: 0.3,
            contrast: 1.4,
            saturation: 1.6,
            gamma: 0.8,
            glow_radius: 2.5,
            beat_reactivity: 2.0,
        },
        transitions: TransitionPrefs {
            default_type: "zoom_blur".to_string(),
            duration: 0.5,
            beat_sync: true,
            drop_flash: 1.0,
            section_transitions: true,
        },
        emotion_colors,
        clarity_modifier: 0.8,
        intensity: 1.5,
    }
}

/// Film noir style
pub fn noir_style() -> Style {
    Style {
        name: "noir".to_string(),
        description: "Black and white with dramatic shadows".to_string(),
        palette: ColorPalette {
            primary: [0.0, 0.0, 0.7],      // Light grey
            secondary: [0.0, 0.0, 0.3],    // Dark grey
            accent: [40.0, 0.3, 0.5],      // Sepia hint
            background: [0.0, 0.0, 0.03],  // Near black
            highlight: [0.0, 0.0, 0.95],   // White
            shadow: [0.0, 0.0, 0.0],       // Pure black
        },
        effects: EffectParams {
            bloom: 0.2,
            chromatic_aberration: 0.0,
            vignette: 0.6,
            grain: 0.2,
            motion_blur: 0.1,
            contrast: 1.5,
            saturation: 0.1,
            gamma: 0.75,
            glow_radius: 0.8,
            beat_reactivity: 0.8,
        },
        transitions: TransitionPrefs {
            default_type: "crossfade".to_string(),
            duration: 0.7,
            beat_sync: false,
            drop_flash: 0.4,
            section_transitions: true,
        },
        emotion_colors: HashMap::new(),
        clarity_modifier: 1.2,
        intensity: 0.9,
    }
}

/// Choose style based on music characteristics
pub fn auto_style_for_genre(genre: &str) -> &'static str {
    match genre.to_lowercase().as_str() {
        "electronic" | "edm" | "techno" | "house" => "neon",
        "ambient" | "new age" | "classical" => "ethereal",
        "synthwave" | "retrowave" | "80s" => "retro",
        "jazz" | "blues" | "soul" => "cinematic",
        "metal" | "rock" | "punk" => "noir",
        "psychedelic" | "trance" | "progressive" => "psychedelic",
        "minimal" | "acoustic" | "folk" => "minimal",
        _ => "default",
    }
}

/// Choose style based on emotion
pub fn style_for_emotion(emotion: &crate::music::Emotion) -> &'static str {
    use crate::music::Emotion;
    match emotion {
        Emotion::Euphoria | Emotion::Excitement => "neon",
        Emotion::Peace | Emotion::Tenderness => "ethereal",
        Emotion::Nostalgia => "retro",
        Emotion::Sadness | Emotion::Melancholy => "noir",
        Emotion::Chaos => "psychedelic",
        Emotion::Intensity | Emotion::Anger => "cinematic",
        Emotion::Joy | Emotion::Triumph => "default",
        _ => "default",
    }
}
