use std::time::Instant;

use num_format::{Locale, ToFormattedString};
use rand::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Width of arena
    #[structopt(short, long)]
    width: u32,

    /// Height of arena
    #[structopt(short, long)]
    height: u32,

    /// Number of entities
    #[structopt(short, long)]
    count: usize,

    /// Size of squares
    #[structopt(short, long)]
    min_size: u32,

    /// Size of squares
    #[structopt(short, long)]
    max_size: u32,

    /// Bitshift cell size (powers of 2)
    #[structopt(short, long)]
    cell_size: u32,
}

fn main() {
    let opt = Opt::from_args();
    let mut grid = supergrid::Grid::new(2048, opt.cell_size);
    grid.clear(); // Ensure clearing grid doesn't make it unusable later on.

    println!("Setup:");
    println!(
        "\tArena width:         {}",
        opt.width.to_formatted_string(&Locale::en)
    );
    println!(
        "\tArena height:        {}",
        opt.height.to_formatted_string(&Locale::en)
    );
    println!(
        "\tArena max size:      {}",
        grid.count().to_formatted_string(&Locale::en)
    );
    println!(
        "\tCell size:           {}x{}",
        1 << opt.cell_size,
        1 << opt.cell_size
    );
    println!(
        "\tEntity count:        {}",
        opt.count.to_formatted_string(&Locale::en)
    );
    println!("\tMinimum entity size: {}x{}", opt.min_size, opt.min_size);
    println!("\tMaximum entity size: {}x{}", opt.max_size, opt.max_size);

    let mut rng = rand::thread_rng();
    let mut entities = vec![];
    let now = Instant::now();
    for i in 0..opt.count {
        let ent = supergrid::Entity {
            id: i as u32,
            x: rng.gen_range(0..opt.width),
            y: rng.gen_range(0..opt.height),
            width: if opt.min_size == opt.max_size {
                opt.max_size
            } else {
                rng.gen_range(opt.min_size..opt.max_size)
            },
            height: if opt.min_size == opt.max_size {
                opt.max_size
            } else {
                rng.gen_range(opt.min_size..opt.max_size)
            },
        };

        grid.insert(&ent).expect("too many entities in cell");
        entities.push(ent);
    }
    println!(
        "Took {:?} to insert {} entities; average: {:?}",
        now.elapsed(),
        opt.count.to_formatted_string(&Locale::en),
        now.elapsed() / opt.count as u32
    );
    let mut hits = 0;
    let now = Instant::now();
    for ent in entities {
        hits += grid.query(&ent.into()).len();
    }
    println!(
        "Took {:?} to probe {} entities; average: {:?}",
        now.elapsed(),
        opt.count.to_formatted_string(&Locale::en),
        now.elapsed() / opt.count as u32
    );
    println!(
        "Collisions: {}; average: {}",
        hits.to_formatted_string(&Locale::en),
        hits as f32 / opt.count as f32
    );
}
