//! Common utilities for omega examples

use std::time::Instant;

/// Timing helper for measuring execution
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        println!("⏱  Starting: {}", name);
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("⏱  {} completed in {}ms", self.name, self.elapsed_ms());
    }
}

/// Print a section header
pub fn print_section(title: &str) {
    println!("\n{}", "=".repeat(60));
    println!("  {}", title);
    println!("{}\n", "=".repeat(60));
}

/// Print a subsection header
pub fn print_subsection(title: &str) {
    println!("\n--- {} ---\n", title);
}

/// Format a float with fixed precision
pub fn format_f64(value: f64, precision: usize) -> String {
    format!("{:.prec$}", value, prec = precision)
}

/// Progress bar helper
pub fn print_progress(current: usize, total: usize, label: &str) {
    let pct = (current as f64 / total as f64 * 100.0) as usize;
    let filled = pct / 5;
    let bar: String = (0..20)
        .map(|i| if i < filled { '█' } else { '░' })
        .collect();
    print!("\r[{}] {}% - {}", bar, pct, label);
    if current == total {
        println!();
    }
}
