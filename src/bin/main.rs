

use clap::Parser;
use rust_cssr::CSSR;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Command-line arguments for the CSSR algorithm.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input data file.
    #[arg(short, long)]
    file: String,

    /// Maximum history length to consider.
    #[arg(short, long, default_value_t = 10)]
    max_history: usize,

    /// Significance level for the chi-square test.
    #[arg(short, long, default_value_t = 0.05)]
    alpha: f32,
}

fn main() {
    let args = Args::parse();

    match read_data(&args.file) {
        Ok(data) => {
            let alphabet: HashSet<u32> = data.iter().cloned().collect();

            let mut cssr = CSSR::new(alphabet);
            cssr.run(&data, args.max_history, args.alpha);

            println!("Number of causal states: {}", cssr.states.len());
            for (i, state) in cssr.states.iter().enumerate() {
                println!("State {}:", i);
                println!("  Histories: {:?}", state.histories);
                println!("  Next symbol distribution: {:?}", state.next_symbol_dist);
            }
        }
        Err(e) => {
            eprintln!("Error reading data file: {}", e);
        }
    }
}

fn read_data<P>(filename: P) -> Result<Vec<u32>, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.and_then(|l| {
                l.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
        .collect()
}
