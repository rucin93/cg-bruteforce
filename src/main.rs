use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::Arc,
    time::Instant,
};

use evaluate::{evaluate, pattern_to_equation};

mod evaluate;
mod generator;

fn main() {
    let generate = false;
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
    let (tx, rx) = std::sync::mpsc::channel();

    for chunk in db.chunks(db.len() / num_cpus::get()) {
        let tx = tx.clone();
        let chunk = Arc::new(chunk.to_vec());
        pool.spawn(move || {
            let thread_id = std::thread::current().id();
            for pattern in chunk.to_vec() {
                println!("{:?} - pattern: {}", thread_id, pattern);
                for code in pattern_to_equation(&pattern).iter() {
                    let ev = evaluate(code);
                    if ev != "" {
                        tx.send(ev).unwrap();
                    }
                }
            }
        });
    }
    drop(tx);
    let results: Vec<String> = rx.into_iter().filter(|x| !x.is_empty()).collect();
    println!("Results: {:?}", results);
    println!("Evaluated in: {:.2?} ", evaluate_time.elapsed());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
