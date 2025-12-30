//! Song Structure Analysis - Layer 3
//!
//! Identifies song sections: Intro, Verse, Chorus, Bridge, etc.
//! This is computed offline and loaded at runtime.

/// Song section types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SectionType {
    #[default]
    Unknown,
    Intro,
    Verse,
    PreChorus,
    Chorus,
    Bridge,
    Breakdown,
    Buildup,
    Drop,
    Outro,
    Instrumental,
}

impl SectionType {
    /// Get visual energy level for this section type
    pub fn energy_level(&self) -> f32 {
        match self {
            SectionType::Unknown => 0.5,
            SectionType::Intro => 0.3,
            SectionType::Verse => 0.5,
            SectionType::PreChorus => 0.7,
            SectionType::Chorus => 0.9,
            SectionType::Bridge => 0.4,
            SectionType::Breakdown => 0.2,
            SectionType::Buildup => 0.6,
            SectionType::Drop => 1.0,
            SectionType::Outro => 0.3,
            SectionType::Instrumental => 0.6,
        }
    }

    /// Should visuals be more "revealed" here?
    pub fn revelation_boost(&self) -> f32 {
        match self {
            SectionType::Chorus => 0.3,
            SectionType::Drop => 0.4,
            SectionType::PreChorus => 0.1,
            SectionType::Buildup => 0.15,
            _ => 0.0,
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            SectionType::Unknown => "Unknown",
            SectionType::Intro => "Intro",
            SectionType::Verse => "Verse",
            SectionType::PreChorus => "Pre-Chorus",
            SectionType::Chorus => "Chorus",
            SectionType::Bridge => "Bridge",
            SectionType::Breakdown => "Breakdown",
            SectionType::Buildup => "Buildup",
            SectionType::Drop => "Drop",
            SectionType::Outro => "Outro",
            SectionType::Instrumental => "Instrumental",
        }
    }
}

/// A section of the song
#[derive(Debug, Clone, Default)]
pub struct Section {
    /// Section type
    pub section_type: SectionType,

    /// Start time (seconds)
    pub start_time: f64,

    /// End time (seconds)
    pub end_time: f64,

    /// Energy level of this section
    pub energy: f32,

    /// How many times we've heard this (1 = first time)
    pub repetition: u8,

    /// Confidence of detection
    pub confidence: f32,
}

impl Section {
    /// Get section duration
    pub fn duration(&self) -> f64 {
        self.end_time - self.start_time
    }

    /// Get progress at given time (0.0 - 1.0)
    pub fn progress_at(&self, time: f64) -> f32 {
        if time < self.start_time {
            0.0
        } else if time > self.end_time {
            1.0
        } else {
            ((time - self.start_time) / self.duration()) as f32
        }
    }

    /// Is approaching end? (within threshold seconds)
    pub fn approaching_end(&self, time: f64, threshold: f64) -> bool {
        time >= self.end_time - threshold && time < self.end_time
    }
}

/// Stored structure analysis (from offline processing)
#[derive(Debug, Clone, Default)]
pub struct StructureAnalysis {
    /// All sections in order
    pub sections: Vec<Section>,

    /// Total song duration
    pub duration: f64,
}

impl StructureAnalysis {
    /// Get section at time
    pub fn section_at(&self, time: f64) -> Section {
        for section in &self.sections {
            if time >= section.start_time && time < section.end_time {
                return section.clone();
            }
        }

        // Return last section if past end, or default
        self.sections.last().cloned().unwrap_or_default()
    }

    /// Get section progress at time
    pub fn section_progress_at(&self, time: f64) -> f32 {
        let section = self.section_at(time);
        section.progress_at(time)
    }

    /// Is approaching section change?
    pub fn approaching_change(&self, time: f64, lookahead: f64) -> bool {
        let section = self.section_at(time);
        section.approaching_end(time, lookahead)
    }

    /// Get section index at time
    pub fn section_index_at(&self, time: f64) -> usize {
        for (i, section) in self.sections.iter().enumerate() {
            if time >= section.start_time && time < section.end_time {
                return i;
            }
        }
        self.sections.len().saturating_sub(1)
    }

    /// Count how many times this section type has occurred up to this point
    pub fn repetition_at(&self, time: f64) -> u8 {
        let current = self.section_at(time);
        let current_idx = self.section_index_at(time);

        let count = self.sections[..=current_idx]
            .iter()
            .filter(|s| s.section_type == current.section_type)
            .count();

        count as u8
    }
}

/// Structure analyzer (placeholder - real impl uses Essentia)
pub struct StructureAnalyzer {
    // Would contain ML model state
}

impl StructureAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Analyze structure offline (this would call Essentia)
    pub fn analyze_offline(&self, _audio_path: &str) -> StructureAnalysis {
        // Placeholder - real implementation would:
        // 1. Load audio file
        // 2. Compute features (MFCC, chroma, etc.)
        // 3. Segment using MSAF or similar
        // 4. Classify segments

        StructureAnalysis::default()
    }
}
