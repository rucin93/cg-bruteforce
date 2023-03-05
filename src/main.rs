extern crate rand;

use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
    sync::Arc,
    time::Instant, collections::HashMap,
};

use evaluate::{evaluate, pattern_to_equation};

mod evaluate;
mod generator;

fn main() {
    // let threads = 2;
    let threads = num_cpus::get();
    let generate = false;
    if generate {
        let timer = Instant::now();
        generator::generate_patterns(1, 11);
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
    let mut done = Vec::new();

    if let Ok(lines) = read_lines("done.txt") {
        for line in lines {
            if let Ok(pattern) = line {
                done.push(pattern);
            }
        }
    }

    let mut found = HashMap::new();

    if let Ok(lines) = read_lines("found.txt") {
        for line in lines {
            if let Ok(pattern) = line {
                let mut pattern = pattern.split(" ");
                let pattern = (pattern.next().unwrap().to_string(), pattern.next().unwrap().to_string());
                found.insert(pattern.0, pattern.1);
            }
        }
    }

    println!("Found: {:?}", found);

    let map: Vec<String> = db.clone().into_iter().collect();
    // let mut db_chunks: Vec<Vec<String>> = vec![vec![]; 40];

    // for s in db {
    //     let count = s.matches("2").count();
    //     db_chunks[count].push(s);
    // }
    // db_chunks.retain(|chunk| !chunk.is_empty());

    let mut diffs: Vec<(String, i32)> = map
        .iter()
        .map(|s| {
            let diff = s.chars().fold(0, |acc, c| match c {
                '2' => acc + 10,
                '*' => acc + 7,
                '~' => acc + 2,
                'x' => acc + 2,
                '(' | ')' => acc + 1,
                _ => acc,
            });
            (s.to_string(), diff)
        })
        .collect();

    // diffs.sort_by_key(|(_, diff)| *diff);
    diffs.sort_by_key(|(_, diff)| -diff);
    let mut db_chunks: Vec<Vec<String>> = vec![vec![]; 40];
    for (s, _) in diffs {
        if done.contains(&s) {
            continue;
        }
        let count = s.len();
        db_chunks[count].push(s);
    }

    db_chunks.retain(|chunk| !chunk.is_empty());

    // println!("{:?}", db_chunks);
    // let mut map: Vec<String> = db.clone().into_iter().collect();
    // map.sort_by_key(|s| s.matches("2").count());
    // let mut rng = rand::thread_rng();

    // map.shuffle(&mut rng);

    println!("DB length : {}", map.len());

    // pass file to each thread

    // multithreaded
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();
    let evaluate_time = Instant::now();
    let (tx, rx) = std::sync::mpsc::channel();
    let unique_keys: Arc<std::sync::Mutex<std::collections::HashMap<String, String>>> =
        std::sync::Arc::new(std::sync::Mutex::new(found));
    for map in db_chunks {
        let mut chunks = map.chunks(1);
        if map.len() >= threads {
            chunks = map.chunks(map.len() / threads);
        }
        for chunk in chunks {
            let tx = tx.clone();
            let chunk = Arc::new(chunk.to_vec());
            let unique_keys = unique_keys.clone();
            pool.spawn(move || {
                let thread_id = std::thread::current().id();
                for pattern in chunk.to_vec() {
                    let mut file = OpenOptions::new()
                        .append(true)
                        .open("done.txt")
                        .expect("Unable to open file");
                    println!("{:?} - pattern: {}", thread_id, pattern);
                    for code in pattern_to_equation(&pattern).iter() {
                        let ev = evaluate(code);
                        for res in ev {
                            if !res.0.is_empty() {
                                let message = format!("{} {}\n", res.0.clone(), res.1.clone());

                                let cloned = res.clone();
                                unique_keys.lock().unwrap().entry(res.0.clone()).and_modify(|e| {
                                    if e.len() > res.1.clone().len() {
                                        *e = res.1.clone();
                                    }
                                }).or_insert(cloned.1.clone());

                                tx.send(res).unwrap();
 
                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .open("found.txt")
                                    .expect("Unable to open file");
                                
                                for (key, value) in unique_keys.lock().unwrap().iter() {
                                    file.write_all(format!("{} {}\n", key, value).as_bytes())
                                        .expect("Unable to write data");
                                }

                                println!("{}", &message);
                            }
                        }
                    }

                    file.write_all(format!("{}\n", pattern).as_bytes())
                        .expect("Unable to write data");
                }
            });
        }
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
