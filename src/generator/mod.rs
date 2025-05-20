use std::{fs::File, io::Write};

use rand::Rng;
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
  pairs: Vec<Pair>
}
pub fn generate(count: u32) -> Result<(), std::io::Error> {
    // Generate 64 points

    let mut file = File::create("test").unwrap();
    let cluster_centered: [Pair; 64] = std::array::from_fn(|_| Pair {
        x0: rand::thread_rng().gen_range(-180f64..180f64),
        x1: rand::thread_rng().gen_range(-180f64..180f64),
        y0: rand::thread_rng().gen_range(-180f64..180f64),
        y1: rand::thread_rng().gen_range(-180f64..180f64),
    });
    // println!("{:?}", cluster_centered);

    // Generate cluster around 64 points with random length
    // for i in 0..count {
    let pairs: Vec<Pair> = (0..count).map(|i| {
        let center = cluster_centered.get((i % 64) as usize).unwrap();
        let xlength = 30f64;
        let ylength = xlength / 2f64;
        Pair {
            x0: center.x0 + rand::thread_rng().gen_range(-xlength..xlength),
            x1: center.x0 + rand::thread_rng().gen_range(-xlength..xlength),
            y0: center.x0 + rand::thread_rng().gen_range(-ylength..ylength),
            y1: center.x0 + rand::thread_rng().gen_range(-ylength..ylength),
        }
    }).collect();
    let output = Output {
      pairs
    };
    file.write_all(serde_json::to_vec(&output).unwrap().as_slice())?;   
    file.flush()?;
    Ok(())
}
