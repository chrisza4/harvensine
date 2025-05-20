use std::path::Path;

mod calc;
mod generator;

fn main() {
    let p = Path::new("output/output");
    let sum = generator::generate(p, 1_000_000);
    println!("Expected sum: {}", sum.unwrap());
}
