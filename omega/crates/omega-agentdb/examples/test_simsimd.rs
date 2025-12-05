use simsimd::SpatialSimilarity;

fn main() {
    let a = vec![1.0f32, 0.0, 0.0];
    let b = vec![1.0f32, 0.0, 0.0];
    let c = vec![0.0f32, 1.0, 0.0];
    
    println!("Testing SimSIMD f32::cosine() output:");
    println!("\nIdentical vectors [1,0,0] vs [1,0,0]:");
    match f32::cosine(&a, &b) {
        Some(val) => println!("  SimSIMD result: {}", val),
        None => println!("  SimSIMD returned None"),
    }
    
    println!("\nOrthogonal vectors [1,0,0] vs [0,1,0]:");
    match f32::cosine(&a, &c) {
        Some(val) => println!("  SimSIMD result: {}", val),
        None => println!("  SimSIMD returned None"),
    }
    
    println!("\n--- Interpretation ---");
    println!("If SimSIMD returns SIMILARITY: Identical=1.0, Orthogonal=0.0");
    println!("If SimSIMD returns DISTANCE: Identical=0.0, Orthogonal=1.0");
}
