use quick_js::Context;
use std::collections::HashMap;

// use v8::{ContextScope, HandleScope};

const SOLUTIONS: [(&str, &str); 12] = [
    ("evil", "0,3,5,6,9,10,12,15,17,18,20,23,24,27,29,30,33,34,36,39,40,43,45,46,48"),
    ("odious", "1,2,4,7,8,11,13,14,16,19,21,22,25,26,28,31,32,35,37,38,41,42,44,47,49,50"),
    ("abnundant", "12,18,20,24,30,36,40,42,48,54,56,60,66,70,72,78,80,84,88,90,96,100,102,104"),
    ("kolakoski", "1,2,2,1,1,2,1,2,2,1,2,2,1,1,2,1,1,2,2,1,2,1,1,2,1,2,2,1,1,2,1,1"),
    ("lucky", "1,3,7,9,13,15,21,25,31,33,37,43,49,51,63,67,69,73,75,79,87,93,99,105,111,115"),
    ("niven", "1,2,3,4,5,6,7,8,9,10,12,18,20,21,24,27,30,36,40,42,45,48,50,54,60,63,70,72,80,81,84,90,100,102,108,110,111,112,114,117,120"),
    ("prime", "2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137"),
    ("pernicious", "3,5,6,7,9,10,11,12,13,14,17,18,19,20,21,22,24,25,26,28,31,33,34,35,36,37,38,40,41,42,44,47,48,49,50"),
    ("pernicious long", "3,5,6,7,9,10,11,12,13,14,17,18,19,20,21,22,24,25,26,28,31,33,34,35,36,37,38,40,41,42,44,47,48,49,50,52,55,56,59,61,62,65,66,67,68,69,70"),
    ("recaman", "0,1,3,6,2,7,13,20,12,21,11,22,10,23,9,24,8,25,43,62,42,63,41,18,42,17,43,16,44,15,45,14,46,79,113,78"),
    ("smith", "4,22,27,58,85,94,121,166,202,265,274,319,346,355,378,382,391,438,454,483,517,526,535,562,576,588,627"),
    ("van-eck", "0,0,1,0,2,0,2,2,1,6,0,5,0,2,6,5,4,0,5,3,0,3,2,9,0,4,9,3,6,14,0,6,3,5,15,0,5,3,5,2,17,0,6,11,0,3,8,0"),
];

const STOP_STRING: &str = ",(safeBreak++ < 50)";

const TEST_CODES: [fn(&str) -> String; 8] = [
    |code| format!("for(i=0;i++{};)print({})", STOP_STRING, code),
    |code| format!("for(i=0;(i++{});){}&&print(i)", STOP_STRING, code),
    |code| format!("for(i=0;(i++{});){}||print(i)", STOP_STRING, code),
    |code| format!("for(i=1;(i++{});){}&&print(i)", STOP_STRING, code),
    |code| format!("for(i=1;(i++{});){}||print(i)", STOP_STRING, code),
    |code| format!("for(i=0;i{};i++)print({})", STOP_STRING, code),
    |code| format!("for(i=1;i++{};)print({})", STOP_STRING, code),
    |code| format!("for(i=1;i{};i++)print({})", STOP_STRING, code),
];

const JS_EVAL: &str = "
var results = [];
var i = 0;
var enoughResults = false;
var safeBreak = 0;

function print(a) {
  if (results.length < 50) {
    results.push(a);
  } else {
    enoughResults = true;
  }
}

";

pub fn evaluate(pattern: &str) -> (String, String) {
    if !check_code(pattern) {
        return ("".to_string(), "".to_string());
    }

    let context = Context::new().unwrap();

    for function in TEST_CODES {
        let js_code = JS_EVAL.to_owned()
            + &function(pattern)
            + "
              results.join(',');
              ";

        let result = context.eval(&js_code).unwrap();
        let result = result.as_str().unwrap();
        for (_i, solution) in SOLUTIONS.iter().enumerate() {
            if result.starts_with(solution.1) {
                return (solution.0.to_string(), function(pattern));
            }
        }
    }

    drop(context);
    return ("".to_string(), "".to_string());
}

pub fn pattern_to_equation(pattern: &str) -> Vec<String> {
    let mut equations = Vec::new();

    if pattern.is_empty() {
        return equations;
    }

    let char_map: HashMap<char, Vec<char>> = [
        ('x', vec!['i']),
        ('2', vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']),
        // ('i', vec!['+', '+', '-', '-']),
        ('~', vec!['~', '!']),
        ('*', vec!['+', '-', '*', '/', '%', '&', '|', '^']),
        ('(', vec!['(']),
        (')', vec![')']),
    ]
    .iter()
    .cloned()
    .collect();

    let mask: Vec<_> = pattern
        .chars()
        .map(|c| {
            for (key, value) in char_map.iter() {
                if *key as char == c {
                    return value;
                }
            }
            panic!("Invalid character: {}", c);
        })
        .collect();

    let mut params = vec![1; mask.len()];
    for i in (0..mask.len() - 1).rev() {
        params[i] = params[i + 1] * mask[i + 1].len();
    }

    let end: usize = mask
        .iter()
        .map(|v| v.len())
        .try_fold(1usize, |acc, len| acc.checked_mul(len))
        .unwrap_or(0);

    for n in 0..end {
        let equation = mask
            .iter()
            .enumerate()
            .map(|(i, current)| {
                let index = (n / params[i]) % current.len();
                current[index]
            })
            .collect::<String>();

        equations.push(equation);
    }

    return equations;
}

fn check_code(code: &str) -> bool {
    !code.contains("---")
        && !code.contains("~i**")
        && !code.starts_with("+i**")
        && !code.starts_with("-i**")
        && !code.contains("!i**")
        && !code.contains("i+++")
        && !code.contains("**-i")
        && !regex::Regex::new(r"0\d").unwrap().is_match(code)
        && !regex::Regex::new(r"\d\+\+").unwrap().is_match(code)
        && !regex::Regex::new(r"(\d|i)--i").unwrap().is_match(code)
        && !regex::Regex::new(r"[^\di]-i\*\*").unwrap().is_match(code)
}
