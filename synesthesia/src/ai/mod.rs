//! AI Module
//!
//! Semantic parsing and scene generation using LLMs.

mod semantic;
mod prompts;

pub use semantic::{SemanticParser, SemanticScene, Setting, Character, Mood, VisualElement, CameraDirection};
