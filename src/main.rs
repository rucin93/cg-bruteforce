use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::Arc,
    sync::Mutex,
    time::Instant,
};

use evaluate::{evaluate, pattern_to_equation};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

mod evaluate;
mod generator;

fn main() {
    let generate = true;
    if generate {
        let timer = Instant::now();
        generator::generate_patterns(10, 10);
        println!("Generated in: {:.2?} ", timer.elapsed());
    }

    let mut db = Vec::new();

    if let Ok(lines) = read_lines("patterns.txt") {
        for line in lines {
            if let Ok(pattern) = line {
                db.push(pattern);
            }
        }
    }

    println!("DB length : {}", db.len());

    // single threaded
    // for pattern in db {
    //     for code in pattern_to_equation(&pattern).iter() {
    //         evaluate(scope, code);
    //     }
    // }

    // multithreaded
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let evaluate_time = Instant::now();
    pool.install(|| {
        // counter to be shared between threads
        let counter = Arc::new(Mutex::new(0));

        db.par_iter().for_each(|pattern| {
            // increment counter
            let mut counter = counter.lock().unwrap();
            let thread_id = std::thread::current().id();
            // println!("{:?} - pattern: {}", thread_id, pattern);
            for code in pattern_to_equation(&pattern).iter() {
                evaluate(code);
            }
            *counter += 1;
            println!("{} - {:?} - pattern: {}", counter, thread_id, pattern);
        });
    });

    println!("Evaluated in: {:.2?} ", evaluate_time.elapsed());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
