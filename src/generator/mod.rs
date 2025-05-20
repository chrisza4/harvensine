use std::{fs::File, io::Write, path::Path};

use crate::calc::haversine;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pair {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    pairs: Vec<Pair>,
}

#[allow(dead_code)]
pub fn generate(path: &Path, count: u32) -> Result<f64, std::io::Error> {
    // Generate 64 points
    let mut randomizer = rand::rngs::SmallRng::from_entropy();
    let mut file = File::create(path).unwrap();

    let cluster_centered: [Pair; 64] = std::array::from_fn(|_| Pair {
        x0: randomizer.gen_range(-180f64..180f64),
        x1: randomizer.gen_range(-180f64..180f64),
        y0: randomizer.gen_range(-180f64..180f64),
        y1: randomizer.gen_range(-180f64..180f64),
    });

    // Generate cluster around 64 points with random length
    // for i in 0..count {
    let pairs: Vec<Pair> = (0..count)
        .map(|i| {
            let center = cluster_centered.get((i % 64) as usize).unwrap();
            let xlength = 30f64;
            let ylength = xlength / 2f64;
            Pair {
                x0: center.x0 + randomizer.gen_range(-xlength..xlength),
                x1: center.x1 + randomizer.gen_range(-xlength..xlength),
                y0: center.y0 + randomizer.gen_range(-ylength..ylength),
                y1: center.y1 + randomizer.gen_range(-ylength..ylength),
            }
        })
        .collect();

    let sum: f64 = pairs
        .iter()
        .map(|c| haversine(c.x0, c.x1, c.y0, c.y1, 6372.8))
        .sum::<f64>()
        / (pairs.len() as f64);

    let output = Output { pairs };

    file.write_all(serde_json::to_vec(&output).unwrap().as_slice())?;
    file.flush()?;

    let sum_filename = path.to_str().unwrap().to_owned() + "_sum";
    let sum_path = Path::new(sum_filename.as_str());
    let mut file = File::create(sum_path).unwrap();
    file.write_all(format!("Sum: {}", sum).as_bytes()).unwrap();
    Ok(sum)
}
