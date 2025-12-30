//! Integration Tests for Synesthesia Player
//!
//! Tests the Rust runtime player components.

#[cfg(test)]
mod tests {
    use synesthesia::player::{SynthLoader, TransitionType, TransitionEngine};
    use synesthesia::shaders::{ShaderUniforms, VERTEX_SHADER, FRAGMENT_SHADER};
    use synesthesia::styles::{StyleManager, ColorPalette};
    use synesthesia::revelation::ClarityBreakdown;

    #[test]
    fn test_shader_uniforms_default() {
        let uniforms = ShaderUniforms::default();

        assert_eq!(uniforms.clarity, 0.0);
        assert_eq!(uniforms.energy, 0.0);
        assert!(uniforms.resolution[0] > 0.0);
    }

    #[test]
    fn test_shader_uniforms_update_time() {
        let mut uniforms = ShaderUniforms::default();
        uniforms.update_time(1.5, 0.016);

        assert!((uniforms.time - 1.5).abs() < 0.001);
        assert!((uniforms.delta_time - 0.016).abs() < 0.001);
    }

    #[test]
    fn test_shader_code_valid() {
        // Basic validation that shader code is present
        assert!(VERTEX_SHADER.len() > 100);
        assert!(FRAGMENT_SHADER.len() > 100);
        assert!(VERTEX_SHADER.contains("vs_main"));
        assert!(FRAGMENT_SHADER.contains("fs_main"));
    }

    #[test]
    fn test_transition_types() {
        assert_eq!(TransitionType::from_str("cut"), TransitionType::Cut);
        assert_eq!(TransitionType::from_str("crossfade"), TransitionType::Crossfade);
        assert_eq!(TransitionType::from_str("morph"), TransitionType::Morph);
        assert_eq!(TransitionType::from_str("flash"), TransitionType::FlashReveal);
        assert_eq!(TransitionType::from_str("unknown"), TransitionType::Crossfade);
    }

    #[test]
    fn test_transition_shader_ids() {
        assert_eq!(TransitionType::Cut.shader_id(), 0);
        assert_eq!(TransitionType::Crossfade.shader_id(), 1);
        assert_eq!(TransitionType::FlashReveal.shader_id(), 4);
    }

    #[test]
    fn test_transition_engine_new() {
        let engine = TransitionEngine::new();

        assert!(!engine.is_transitioning());
        assert_eq!(engine.current_segment(), 0);
    }

    #[test]
    fn test_transition_engine_trigger() {
        let mut engine = TransitionEngine::new();

        engine.trigger(1, TransitionType::FlashReveal, 0.5);

        assert!(engine.is_transitioning());
        assert_eq!(engine.current_segment(), 0); // Still at 0 until complete
    }

    #[test]
    fn test_style_manager_defaults() {
        let manager = StyleManager::new();

        // Should have default styles
        assert!(manager.get("default").is_some());
        assert!(manager.get("neon").is_some());
        assert!(manager.get("ethereal").is_some());
        assert!(manager.get("cinematic").is_some());
    }

    #[test]
    fn test_style_manager_set() {
        let mut manager = StyleManager::new();

        assert!(manager.set("neon"));
        assert_eq!(manager.current().name, "neon");

        assert!(!manager.set("nonexistent"));
    }

    #[test]
    fn test_style_palette_rgb_conversion() {
        let palette = ColorPalette {
            primary: [0.0, 1.0, 0.5],      // Red
            secondary: [120.0, 1.0, 0.5],  // Green
            accent: [240.0, 1.0, 0.5],     // Blue
            background: [0.0, 0.0, 0.0],
            highlight: [0.0, 0.0, 1.0],
            shadow: [0.0, 0.0, 0.0],
        };

        let rgb = palette.primary_rgb();
        assert!(rgb[0] > 0.9); // Red should be near 1
        assert!(rgb[1] < 0.1); // Green should be near 0
        assert!(rgb[2] < 0.1); // Blue should be near 0
    }

    #[test]
    fn test_clarity_breakdown_default() {
        let breakdown = ClarityBreakdown::default();

        assert_eq!(breakdown.total, 0.0);
        assert_eq!(breakdown.signal, 0.0);
        assert_eq!(breakdown.theory, 0.0);
        assert_eq!(breakdown.structure, 0.0);
        assert_eq!(breakdown.emotion, 0.0);
    }

    #[test]
    fn test_synth_loader_invalid_file() {
        // Should return false for non-existent file
        assert!(!SynthLoader::is_valid("/nonexistent/file.synth"));
    }

    #[test]
    fn test_all_preset_styles_valid() {
        let manager = StyleManager::new();

        let style_names = ["default", "neon", "ethereal", "cinematic",
                          "retro", "minimal", "psychedelic", "noir"];

        for name in style_names {
            let style = manager.get(name);
            assert!(style.is_some(), "Style '{}' should exist", name);

            let style = style.unwrap();
            assert!(!style.name.is_empty());
            assert!(!style.description.is_empty());
            assert!(style.intensity > 0.0);
        }
    }
}
