use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::Instant,
};

use evaluate::{evaluate, pattern_to_equation};

mod evaluate;
mod generator;

fn main() {
    let generate = false;
    if generate {
        let timer = Instant::now();
        generator::generate_patterns(7, 20);
        println!("Generated in: {:.2?} ", timer.elapsed());
    }

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
    let handle_scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope);
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let evaluate_time = Instant::now();
    let mut db = Vec::new();

    if let Ok(lines) = read_lines("patterns.txt") {
        for line in lines {
            if let Ok(pattern) = line {
                db.push(pattern);
            }
        }
    }
    println!("{}", db.len());

    // split db into chunks and sen
    for pattern in db {
        for code in pattern_to_equation(&pattern).iter() {
            evaluate(scope, code);
        }
    }

    println!("Evaluated in: {:.2?} ", evaluate_time.elapsed());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
