//! Prompt Templates
//!
//! LLM prompts for semantic parsing.

/// System prompt for lyric semantic parsing
pub const SEMANTIC_PARSER_SYSTEM: &str = r#"
You are a visual director translating song lyrics into scene descriptions.
Your goal is to create meaningful visual representations of the lyrics.

Guidelines:
- Maintain visual continuity with previous scenes
- Interpret metaphors creatively but coherently
- Focus on the emotional core of the lyrics
- Create visually interesting but not literal interpretations
- Consider the musical mood when interpreting neutral lyrics

Output JSON only, no explanations.
"#;

/// Template for parsing lyrics
pub const PARSE_LYRICS_TEMPLATE: &str = r#"
Previous scene context:
- Setting: {current_setting}
- Characters: {active_characters}
- Emotional arc: {emotional_arc}

Current lyrics: "{lyrics}"

Generate a JSON scene description:
{{
  "setting": {{
    "location": "specific place description",
    "time_of_day": "dawn|day|dusk|night",
    "weather": "clear|rain|snow|fog|storm",
    "indoor": boolean
  }},
  "characters": [
    {{
      "id": "unique_id for continuity",
      "description": "visual description",
      "state": "what they're doing",
      "emotion": "how they feel"
    }}
  ],
  "actions": ["what's happening in the scene"],
  "mood": {{
    "primary": "single emotion word",
    "intensity": 0.0-1.0,
    "color_palette": ["#hex1", "#hex2", "#hex3"]
  }},
  "visual_elements": [
    {{"element_type": "particle|light|object|effect", "description": "..."}}
  ],
  "camera": {{
    "shot": "wide|medium|close|extreme_close",
    "movement": "static|pan|dolly|orbit|follow",
    "focus": "what to focus on"
  }}
}}
"#;

/// Build the full prompt for parsing
pub fn build_parse_prompt(
    lyrics: &str,
    current_setting: &str,
    active_characters: &[String],
    emotional_arc: &str,
) -> String {
    PARSE_LYRICS_TEMPLATE
        .replace("{lyrics}", lyrics)
        .replace("{current_setting}", current_setting)
        .replace("{active_characters}", &active_characters.join(", "))
        .replace("{emotional_arc}", emotional_arc)
}
