use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
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
        generator::generate_patterns(1, 6);
        println!("Generated in: {:.2?} ", timer.elapsed());
    }
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::set_flags_from_string("--max_old_space_size=4096");
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let evaluate_time = Instant::now();
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

    pool.install(|| {
        db.par_iter().for_each(|pattern| {
            // get thread id
            let thread_id = std::thread::current().id();
            println!("{:?} - pattern: {}", thread_id, pattern);
            let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
            let handle_scope = &mut v8::HandleScope::new(isolate);
            let context = v8::Context::new(handle_scope);
            let scope = &mut v8::ContextScope::new(handle_scope, context);
            for code in pattern_to_equation(&pattern).iter() {
                evaluate(scope, code);
            }
            // wait for thread to finish
            // std::thread::sleep(std::time::Duration::from_millis(100));
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
