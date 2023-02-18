use std::time::Instant;

use evaluate::pattern_to_equation;

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

    evaluate::evaluate(scope);

    pattern_to_equation("2*x");
}
