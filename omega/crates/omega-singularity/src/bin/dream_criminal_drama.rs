//! 🎬 CRIMINAL DRAMA GENERATOR - ENHANCED VERSION
//!
//! Generates the ultimate 5-minute criminal drama short film.
//! Runs 1000 iterations with crime-specific enhancements.

use omega_singularity::dream_cinema::*;
use std::time::Instant;
use rand::Rng;

// Criminal drama specific elements
const CRIME_SETTINGS: &[(&str, &str)] = &[
    ("THE INTERROGATION ROOM", "Concrete walls sweating under fluorescent buzz. A metal table bolted to the floor. Two chairs. One-way mirror reflecting nothing but guilt."),
    ("THE ABANDONED WAREHOUSE", "Rust and shadows. Broken skylights letting in columns of dying light. The smell of old blood and fresh fear."),
    ("THE MIDNIGHT DINER", "Neon bleeding through rain-streaked windows. A waitress who's seen too much. Coffee that tastes like regret."),
    ("THE PARKING GARAGE", "Echoing footsteps. Flickering lights. Concrete pillars hiding what shouldn't be seen. Oil stains that might not be oil."),
    ("THE MOTEL ROOM", "Peeling wallpaper. Stained curtains. A bed where confessions happen. The ice machine humming its mechanical prayer."),
    ("THE ROOFTOP", "City lights below like scattered evidence. Wind carrying sirens. Nowhere left to run but down."),
    ("THE BACK ALLEY", "Dumpsters overflow with secrets. Fire escapes like cages. Shadows that move wrong."),
    ("THE EVIDENCE LOCKER", "Boxes of lives reduced to tagged items. Cold storage. Every object screams a story no one wants to hear."),
];

const CRIME_DIALOGUE: &[(&str, &str, &str)] = &[
    ("I didn't have a choice.", "We always have a choice. You just didn't like yours.", "Justification meets judgment"),
    ("You don't know what I've done.", "I know exactly what you've done. The question is: do you?", "Confession seeking absolution"),
    ("The money was never about the money.", "It's always about the money. Or what the money represents.", "Motive revealed through denial"),
    ("They'll never find the body.", "They already have. That's why I'm here.", "The walls closing in"),
    ("I trusted you.", "Trust is just another word for leverage.", "Betrayal crystallized"),
    ("How long have you known?", "Long enough to make my own arrangements.", "Power dynamics shifting"),
    ("There's blood on your hands too.", "The difference is, I can wash mine clean.", "Moral complexity"),
    ("You're going to kill me.", "I'm going to give you a choice. That's more than you gave them.", "Justice or vengeance"),
    ("I can explain everything.", "Don't. Every word you say makes it worse.", "Silence as condemnation"),
    ("What do you want?", "What everyone wants. The truth. And then we'll see.", "The interrogation begins"),
];

const CRIME_CHARACTERS: &[(&str, &str, &str, &str)] = &[
    ("MARCUS COLE", "Homicide detective with 20 years of ghosts behind his eyes. His wedding ring tan line never faded.", "Find the truth before it finds him", "That he's become what he hunts"),
    ("ELENA VANCE", "Defense attorney who crossed a line she can't uncross. Armani suits hiding armor.", "Bury the evidence of her own crime", "The victim she forgot"),
    ("VICTOR 'GHOST' REYES", "Former enforcer trying to go straight. His past keeps writing checks his future can't cash.", "Protect his daughter from his legacy", "That redemption is a lie"),
    ("SARAH CHEN", "Forensic accountant who found numbers that don't add up. The deeper she digs, the more graves she finds.", "Expose the conspiracy", "Becoming part of it"),
    ("DETECTIVE JAMES MORRISON", "Internal Affairs. Everyone hates him. That's how he knows he's doing his job right.", "Clean the department", "His own sins coming due"),
    ("NINA FROST", "The widow who isn't grieving. Her husband's death was the first honest thing in their marriage.", "Claim what's hers", "That she's next"),
];

/// Enhanced criminal drama generator
struct CriminalDramaGenerator {
    rng: rand::rngs::ThreadRng,
}

impl CriminalDramaGenerator {
    fn new() -> Self {
        Self { rng: rand::thread_rng() }
    }

    fn generate_title(&mut self) -> String {
        let titles = [
            "The Hollow Men",
            "Debts Paid in Blood",
            "Before Dawn Breaks",
            "The Confession",
            "What We Bury",
            "Shadows of Intent",
            "The Last Good Man",
            "Complicit",
            "The Weight of Silence",
            "Nothing Clean",
            "After the Fall",
            "Sins of Omission",
            "The Reckoning",
            "Blood Ledger",
            "Mercy's Edge",
        ];
        titles[self.rng.gen_range(0..titles.len())].to_string()
    }

    fn generate_logline(&mut self, chars: &[CrimeCharacter]) -> String {
        if chars.len() >= 2 {
            format!(
                "When {} discovers a secret that could destroy everything, {} must choose between \
                justice and survival in a five-minute descent into moral darkness where every choice \
                leads to damnation.",
                chars[0].name.split_whitespace().next().unwrap_or("someone"),
                chars[1].name.split_whitespace().next().unwrap_or("someone")
            )
        } else {
            "In five minutes of mounting dread, a desperate soul confronts the truth they've been \
            running from, in a criminal drama where silence speaks louder than confessions.".to_string()
        }
    }

    fn select_characters(&mut self) -> Vec<CrimeCharacter> {
        let mut indices: Vec<usize> = (0..CRIME_CHARACTERS.len()).collect();
        let mut selected = Vec::new();

        for _ in 0..3.min(CRIME_CHARACTERS.len()) {
            let idx = self.rng.gen_range(0..indices.len());
            let char_idx = indices.remove(idx);
            let (name, desc, motivation, fear) = CRIME_CHARACTERS[char_idx];
            selected.push(CrimeCharacter {
                name: name.to_string(),
                description: desc.to_string(),
                motivation: motivation.to_string(),
                fear: fear.to_string(),
            });
        }

        selected
    }

    fn generate_scene(&mut self, num: usize, chars: &[CrimeCharacter]) -> CrimeScene {
        let (setting_name, setting_desc) = CRIME_SETTINGS[self.rng.gen_range(0..CRIME_SETTINGS.len())];
        let (line1, line2, subtext) = CRIME_DIALOGUE[self.rng.gen_range(0..CRIME_DIALOGUE.len())];

        let chars_in_scene: Vec<_> = if chars.len() >= 2 && self.rng.gen_bool(0.7) {
            vec![chars[0].clone(), chars[1].clone()]
        } else if !chars.is_empty() {
            vec![chars[0].clone()]
        } else {
            vec![]
        };

        let description = self.generate_action(&chars_in_scene, setting_name, num);

        CrimeScene {
            number: num,
            setting: setting_name.to_string(),
            setting_description: setting_desc.to_string(),
            description,
            dialogue: if chars_in_scene.len() >= 2 {
                vec![
                    (chars_in_scene[0].name.clone(), line1.to_string(), Some(subtext.to_string())),
                    (chars_in_scene[1].name.clone(), line2.to_string(), None),
                ]
            } else if !chars_in_scene.is_empty() {
                vec![(chars_in_scene[0].name.clone(), line1.to_string(), Some(subtext.to_string()))]
            } else {
                vec![]
            },
            visual_note: self.get_visual_note(num),
            score_note: self.get_score_note(num),
        }
    }

    fn generate_action(&mut self, chars: &[CrimeCharacter], setting: &str, scene_num: usize) -> String {
        let actions = match scene_num {
            1 => vec![
                "enters, checking over their shoulder. The door closes with the finality of a cell.",
                "waits in the shadows. They've been here before. The room remembers.",
                "stands frozen. This is the moment they've been dreading. It's finally here.",
            ],
            2 | 3 => vec![
                "The tension is a living thing between them. Words hang unspoken, heavy as evidence.",
                "circles the space, predator or prey unclear. Every step echoes guilt.",
                "sits motionless, hands visible. The stillness of someone with nothing left to lose.",
            ],
            4 | 5 | 6 => vec![
                "The truth surfaces like a body from deep water. Cold. Undeniable.",
                "pulls out the evidence. It falls between them like a verdict.",
                "breaks. The mask finally cracks. Underneath: something worse than expected.",
                "moves to the window. The city below offers no escape. Never did.",
            ],
            7 | 8 => vec![
                "makes a choice that can't be unmade. The moment stretches into eternity.",
                "reaches for the gun. Time stops. This is how it ends. Or begins.",
                "walks away. The hardest thing they've ever done. They don't look back.",
                "collapses. It's over. Nothing will ever be the same.",
            ],
            _ => vec![
                "The silence speaks everything words cannot.",
            ],
        };

        let action = actions[self.rng.gen_range(0..actions.len())];
        let char_ref = if !chars.is_empty() {
            chars[0].name.split_whitespace().next().unwrap_or("They").to_uppercase()
        } else {
            "FIGURE".to_string()
        };

        format!("{}\n\n{} {}",
            setting,
            char_ref,
            action
        )
    }

    fn get_visual_note(&mut self, scene_num: usize) -> String {
        match scene_num {
            1 => "WIDE SHOT, slowly pushing in. Harsh overhead lighting creating deep shadows. Cool blue-gray palette.",
            2 | 3 => "MEDIUM TWO-SHOT, tension held in negative space. Shallow focus shifting between subjects.",
            4 | 5 => "CLOSE-UPS intercut. Every micro-expression matters. Handheld tremor entering the frame.",
            6 => "DUTCH ANGLE increasing. The world tilting off its axis. High contrast, nearly noir.",
            7 => "EXTREME CLOSE-UP on eyes, hands, details. The geography of guilt.",
            8 => "PULL BACK to wide. The figures small against the void. Hold. Let it breathe.",
            _ => "TABLEAU. Let the composition tell the story.",
        }.to_string()
    }

    fn get_score_note(&mut self, scene_num: usize) -> String {
        match scene_num {
            1 => "Low drone. Single piano note, sustaining. The sound of dread approaching.",
            2 | 3 => "Pulse builds. Processed strings creating unease. Silence weaponized.",
            4 | 5 => "Tension peaks. Percussion like heartbeats. The score becomes visceral.",
            6 => "Dissonance. Nothing resolves. The musical equivalent of a held breath.",
            7 => "Everything strips away. A single held note. Then silence.",
            8 => "Sparse resolution. Melancholic piano. The weight of consequence.",
            _ => "Atmospheric tension. Every silence earned.",
        }.to_string()
    }

    fn generate_twist(&mut self, chars: &[CrimeCharacter]) -> CrimeTwist {
        let twists = [
            ("IDENTITY REVEAL", "The detective and the criminal share the same face. Twins. One chose the badge, one chose the blood. Neither made it out clean."),
            ("THE FRAME", "The evidence was planted. By the victim. Their final act of revenge from beyond the grave."),
            ("INSIDE JOB", "The 'witness' isn't a witness. They're the architect. Every confession was a trap."),
            ("THE DEAL", "There is no investigation. This was always an execution. The only question: who ordered it?"),
            ("GENERATIONAL SIN", "The case file is decades old. The victim: their father. The killer: already in the room."),
            ("THE RECORDING", "Every word was captured. Not as evidence. As leverage. The real crime is just beginning."),
            ("COMPLICIT", "The 'innocent' party knew everything. They didn't stop it because they needed it to happen."),
            ("DEAD MAN WALKING", "The person they've been hunting died three months ago. Someone else has been wearing their life."),
        ];

        let (twist_type, description) = twists[self.rng.gen_range(0..twists.len())];

        CrimeTwist {
            twist_type: twist_type.to_string(),
            description: description.to_string(),
            reveal_scene: self.rng.gen_range(5..=7),
        }
    }

    fn generate(&mut self) -> CriminalDrama {
        let title = self.generate_title();
        let characters = self.select_characters();
        let logline = self.generate_logline(&characters);

        let mut scenes = Vec::new();
        for i in 1..=8 {
            scenes.push(self.generate_scene(i, &characters));
        }

        let twist = self.generate_twist(&characters);

        CriminalDrama {
            title,
            logline,
            characters,
            scenes,
            twist,
        }
    }
}

#[derive(Clone)]
struct CrimeCharacter {
    name: String,
    description: String,
    motivation: String,
    fear: String,
}

struct CrimeScene {
    number: usize,
    setting: String,
    setting_description: String,
    description: String,
    dialogue: Vec<(String, String, Option<String>)>,
    visual_note: String,
    score_note: String,
}

struct CrimeTwist {
    twist_type: String,
    description: String,
    reveal_scene: usize,
}

struct CriminalDrama {
    title: String,
    logline: String,
    characters: Vec<CrimeCharacter>,
    scenes: Vec<CrimeScene>,
    twist: CrimeTwist,
}

fn score_drama(drama: &CriminalDrama) -> f64 {
    let mut score = 0.0;

    // Title quality
    let good_titles = ["Blood", "Silence", "Confession", "Shadow", "Dead", "Last", "Hollow"];
    for word in good_titles {
        if drama.title.contains(word) {
            score += 5.0;
        }
    }

    // Character depth (max 25)
    score += (drama.characters.len() as f64 * 8.0).min(25.0);

    // Scene variety (max 20) - unique settings
    let unique_settings: std::collections::HashSet<_> = drama.scenes.iter()
        .map(|s| s.setting.clone())
        .collect();
    score += (unique_settings.len() as f64 * 4.0).min(20.0);

    // Dialogue quality (max 25)
    let total_lines: usize = drama.scenes.iter()
        .map(|s| s.dialogue.len())
        .sum();
    let subtext_lines: usize = drama.scenes.iter()
        .flat_map(|s| s.dialogue.iter())
        .filter(|(_, _, sub)| sub.is_some())
        .count();
    score += (total_lines as f64 * 2.0 + subtext_lines as f64 * 3.0).min(25.0);

    // Twist quality (max 15)
    let twist_power = match drama.twist.twist_type.as_str() {
        "IDENTITY REVEAL" | "DEAD MAN WALKING" => 15.0,
        "THE FRAME" | "COMPLICIT" => 12.0,
        _ => 10.0,
    };
    score += twist_power;

    // Bonus for dark settings
    let dark_settings = drama.scenes.iter()
        .filter(|s| s.setting.contains("INTERROGATION") ||
                   s.setting.contains("WAREHOUSE") ||
                   s.setting.contains("ALLEY"))
        .count();
    score += dark_settings as f64 * 2.0;

    score
}

fn format_screenplay(drama: &CriminalDrama) -> String {
    let mut output = String::new();

    // Title page
    output.push_str(&format!(r#"
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                                                                                                  ║
║                                                                                                  ║
║                                         "{}"
║                                                                                                  ║
║                                     A Criminal Drama                                             ║
║                                                                                                  ║
║                                  Written by DREAM CINEMA AI                                      ║
║                              1000 Iterations • Best of Generation                                ║
║                                                                                                  ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝


LOGLINE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

{}


CHARACTERS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

"#, drama.title, drama.logline));

    for character in &drama.characters {
        output.push_str(&format!(r#"
{}
    {}
    WANTS: {}
    FEARS: {}
"#, character.name, character.description, character.motivation, character.fear));
    }

    output.push_str(r#"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                                         SCREENPLAY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#);

    for scene in &drama.scenes {
        output.push_str(&format!(r#"

SCENE {} — {}
{}

{}
"#, scene.number, scene.setting, scene.setting_description, scene.description));

        for (character, line, subtext) in &scene.dialogue {
            output.push_str(&format!(r#"
                              {}
                    {}
"#, character.split_whitespace().next().unwrap_or(character).to_uppercase(), line));
            if let Some(sub) = subtext {
                output.push_str(&format!("                    [{}]\n", sub));
            }
        }

        output.push_str(&format!(r#"
[CAMERA: {}]
[SCORE: {}]
"#, scene.visual_note, scene.score_note));

        // Insert twist at reveal scene
        if scene.number == drama.twist.reveal_scene {
            output.push_str(&format!(r#"
╔═══════════════════════════════════════════════════════════════════════════════════════╗
║  THE TWIST: {}
║  {}
╚═══════════════════════════════════════════════════════════════════════════════════════╝
"#, drama.twist.twist_type, drama.twist.description));
        }
    }

    output.push_str(r#"


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                                           THE END
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DIRECTOR'S NOTES:

• This is a five-minute descent into moral complexity
• Every silence should feel dangerous
• The camera never lies, but the characters always do
• The twist should reframe everything that came before
• End on ambiguity — let the audience carry the weight

RUNTIME: ~5 minutes
GENRE: Criminal Drama / Psychological Thriller
TONE: Tense, morally ambiguous, noir-influenced

Generated by DREAM CINEMA AI
From 1000 iterations of collective neural dreaming
"#);

    output
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║  🎬 DREAM CINEMA: CRIMINAL DRAMA GENERATOR v2.0                                       ║");
    println!("║  Running 1000 iterations to find the ultimate 5-minute crime script                  ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝\n");

    let start = Instant::now();
    let iterations = 1000;

    let mut generator = CriminalDramaGenerator::new();
    let mut best_drama: Option<CriminalDrama> = None;
    let mut best_score = 0.0;
    let mut scores: Vec<f64> = Vec::with_capacity(iterations);

    for i in 0..iterations {
        let drama = generator.generate();
        let score = score_drama(&drama);
        scores.push(score);

        if score > best_score {
            best_score = score;
            println!("  [{}] 🎯 New best: \"{}\" (score: {:.2})", i + 1, drama.title, best_score);
            best_drama = Some(drama);
        }

        if (i + 1) % 200 == 0 {
            println!("  ⏳ Progress: {}/{} iterations", i + 1, iterations);
        }
    }

    let elapsed = start.elapsed();

    // Statistics
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let avg_score: f64 = scores.iter().sum::<f64>() / scores.len() as f64;
    let median_score = scores[scores.len() / 2];
    let top_10_avg: f64 = scores.iter().rev().take(10).sum::<f64>() / 10.0;

    println!("\n═══════════════════════════════════════════════════════════════════════════════════════");
    println!("                              🎬 GENERATION COMPLETE");
    println!("═══════════════════════════════════════════════════════════════════════════════════════");
    println!("  Total iterations: {}", iterations);
    println!("  Time elapsed: {:?}", elapsed);
    println!("  Average score: {:.2}", avg_score);
    println!("  Median score: {:.2}", median_score);
    println!("  Top 10 average: {:.2}", top_10_avg);
    println!("  BEST SCORE: {:.2}", best_score);
    println!("═══════════════════════════════════════════════════════════════════════════════════════\n");

    if let Some(drama) = best_drama {
        println!("{}", format_screenplay(&drama));
    }
}
