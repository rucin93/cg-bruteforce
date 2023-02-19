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
    let unique_keys: Arc<std::sync::Mutex<std::collections::HashSet<String>>> =
        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashSet::new()));

    for chunk in db.chunks(db.len() / num_cpus::get()) {
        let tx = tx.clone();
        let chunk = Arc::new(chunk.to_vec());
        let unique_keys = unique_keys.clone();
        pool.spawn(move || {
            let thread_id = std::thread::current().id();
            for pattern in chunk.to_vec() {
                println!("{:?} - pattern: {}", thread_id, pattern);
                for code in pattern_to_equation(&pattern).iter() {
                    let ev = evaluate(code);
                    if !ev.0.is_empty() {
                        println!("{:?} - Found solution: {} {}", thread_id, ev.0, ev.1);
                        if !unique_keys.lock().unwrap().contains(&ev.0.clone()) {
                            unique_keys.lock().unwrap().insert(ev.0.clone());
                            tx.send(ev).unwrap();
                        }

                        println!("Found solutions for: {:?}", unique_keys.lock().unwrap());
                    }
                }
            }
        });
    }
    drop(tx);
    let results: Vec<(String, String)> = rx.into_iter().collect();

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
