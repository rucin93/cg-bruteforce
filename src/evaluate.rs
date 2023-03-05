use std::collections::HashMap;


const SOLUTIONS: [(&str, &str); 13] = [
    ("evil", "0,3,5,6,9,10,12,15,17,18,20,23,24,27,29,30,33,34,36,39,40,43,45,46,48"),
    ("odious", "1,2,4,7,8,11,13,14,16,19,21,22,25,26,28,31,32,35,37,38,41,42,44,47,49,50"),
    ("abnundant", "12,18,20,24,30,36,40,42,48,54,56,60,66,70,72,78,80,84,88,90,96,100,102,104"),
    ("kolakoski", "1,2,2,1,1,2,1,2,2,1,2,2,1,1,2,1,1,2,2,1,2,1,1,2,1,2,2,1,1,2,1,1"),
    ("kolakoski based 0", "0,1,1,0,0,1,0,1,1,0,1,1,0,0,1,0,0,1,1,0,1,0,0,1,0,1,1,0,0,1,0,0"),
    ("lucky", "1,3,7,9,13,15,21,25,31,33,37,43,49,51,63,67,69,73,75,79,87,93,99,105,111,115"),
    ("niven", "1,2,3,4,5,6,7,8,9,10,12,18,20,21,24,27,30,36,40,42,45,48,50,54,60,63,70,72,80,81,84,90,100,102,108,110,111,112,114,117,120"),
    ("prime", "2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137"),
    ("pernicious", "3,5,6,7,9,10,11,12,13,14,17,18,19,20,21,22,24,25,26,28,31,33,34,35,36,37,38,40,41,42,44,47,48,49,50"),
    ("pernicious long", "3,5,6,7,9,10,11,12,13,14,17,18,19,20,21,22,24,25,26,28,31,33,34,35,36,37,38,40,41,42,44,47,48,49,50,52,55,56,59,61,62,65,66,67,68,69,70"),
    ("recaman", "0,1,3,6,2,7,13,20,12,21,11,22,10,23,9,24,8,25,43,62,42,63,41,18,42,17,43,16,44,15,45,14,46,79,113,78"),
    ("smith", "4,22,27,58,85,94,121,166,202,265,274,319,346,355,378,382,391,438,454,483,517,526,535,562,576,588,627"),
    ("van-eck", "0,0,1,0,2,0,2,2,1,6,0,5,0,2,6,5,4,0,5,3,0,3,2,9,0,4,9,3,6,14,0,6,3,5,15,0,5,3,5,2,17,0,6,11,0,3,8,0"),
];

pub fn evaluate(pattern: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();

    if !check_code(pattern) {
        // wrong euqation
        return results;
    }


    let map_zero=evaluate_based((&pattern).to_string(), 0, 0);
    let map_non_zero=evaluate_based((&pattern).to_string(), 0, 1);
    let from_one_zero = evaluate_based((&pattern).to_string(), 1, 0);
    let from_one_non_zero = evaluate_based((&pattern).to_string(), 1, 1);

    for (_i, solution) in SOLUTIONS.iter().enumerate() {
        if pattern.contains("j") {
            if map_zero.join(",").contains(solution.1) {
                results.push((
                    solution.0.to_string(),
                    format!("for(i=j=0;i<50;i++){}||print(j=i)", rpn_to_infix(pattern)),
                ));
            } 
            
            if map_non_zero.join(",").contains(solution.1) {
                results.push((
                    solution.0.to_string(),
                    format!("for(j=i=0;i<50;i++){}&&print(j=i)", rpn_to_infix(pattern)),
                ));
            }

            if from_one_zero.join(",").contains(solution.1) {
                results.push((
                    solution.0.to_string(),
                    format!("for(j=i=1;i<50;i++){}&&print(j=i)", rpn_to_infix(pattern)),
                ));
            }

            if from_one_non_zero.join(",").contains(solution.1) {
                results.push((
                    solution.0.to_string(),
                    format!("for(j=i=1;i<50;i++){}&&print(j=i)", rpn_to_infix(pattern)),
                ));
            }
        } else {
            if map_zero.join(",").contains(solution.1) {
                results.push((
                    solution.0.to_string(),
                    format!("for(i=0;i<50;i++){}||print(i)", rpn_to_infix(pattern)),
                ));
            } 
            
            if map_non_zero.join(",").contains(solution.1) {
                results.push( (
                    solution.0.to_string(),
                    format!("for(i=0;i<50;i++){}&&print(i)", rpn_to_infix(pattern)),
                ));
            }

            if from_one_zero.join(",").contains(solution.1) {
                results.push((
                    solution.0.to_string(),
                    format!("for(i=1;i<51;i++){}&&print(j=i)", rpn_to_infix(pattern)),
                ));
            }

            if from_one_non_zero.join(",").contains(solution.1) {
                results.push((
                    solution.0.to_string(),
                    format!("for(i=1;i<51;i++){}&&print(j=i)", rpn_to_infix(pattern)),
                ));
            }
        }
    }

    return results;
}

fn evaluate_based(pattern: String, from: i32, base: i32) -> Vec<String> {
    let mut map = Vec::new();
    let mut j = from;
    for x in from..=100 {
        let expression = pattern.replace("i", &x.to_string()).replace("j", &j.to_string());
        let result = match evaluate_rpn(&expression) {
            Ok(r) => r,
            Err(_e) => {
                // eprintln!("Error evaluating expression {}: {}", expression, _e);
                continue;
            }
        };
        if base != 0 {
            if result != 0.0 {
                map.push(x.to_string());
                j = x;
            }
        } else {
            if result == 0.0 {
                map.push(x.to_string());
                j = x;
            }
        }
       
    }

    return map;
}

pub fn pattern_to_equation(pattern: &str) -> Vec<String> {
    let mut equations = Vec::new();

    if pattern.is_empty() {
        return equations;
    }

    let char_map: HashMap<char, Vec<char>> = [
        ('x', vec!['i', 'j']),
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

        equations.push(infix_to_rpn(&equation));
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
        && !code.contains("~j**")
        && !code.starts_with("+j**")
        && !code.starts_with("-j**")
        && !code.contains("!j**")
        && !code.contains("j+++")
        && !code.contains("**-j")
        && !regex::Regex::new(r"(\*)1(\D|)").unwrap().is_match(code)
        && !regex::Regex::new(r"(\D)0\D").unwrap().is_match(code)
        && !regex::Regex::new(r"0\d").unwrap().is_match(code)
        && !regex::Regex::new(r"\d\+\+").unwrap().is_match(code)
        && !regex::Regex::new(r"(\d|i)--i").unwrap().is_match(code)
        && !regex::Regex::new(r"[^\di]-i\*\*").unwrap().is_match(code)
        && !regex::Regex::new(r"(\d|j)--j").unwrap().is_match(code)
        && !regex::Regex::new(r"[^\dj]-j\*\*").unwrap().is_match(code)
}

fn infix_to_rpn(expression: &str) -> String {
    let mut output_queue: Vec<String> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();

    let operators = vec!["+", "-", "*", "/", "%", "^", "&", "|", "~", "!"];

    let mut iter = expression.chars().peekable();
    while let Some(c) = iter.next() {
        if c.is_numeric() || c == '.' {
            let mut number = c.to_string();
            while let Some(next) = iter.peek() {
                if next.is_numeric() || *next == '.' {
                    number.push(*next);
                    iter.next();
                } else {
                    break;
                }
            }
            output_queue.push(number);
        } else if c == 'i' {
            output_queue.push(String::from("i"));
        } else if c == 'j' {
            output_queue.push(String::from("j"));
        } else if operators.contains(&c.to_string().as_str()) {
            while let Some(top) = operator_stack.last() {
                if operators.contains(&top.as_str())
                    && ((c != '^' && top != "^") || (c == '^' && top == "^"))
                    && precedence(c.to_string().as_str()) <= precedence(top)
                {
                    output_queue.push(operator_stack.pop().unwrap());
                } else {
                    break;
                }
            }
            operator_stack.push(String::from(c));
        } else if c == '(' {
            operator_stack.push(String::from(c));
        } else if c == ')' {
            while let Some(top) = operator_stack.pop() {
                if top == "(" {
                    break;
                } else {
                    output_queue.push(top);
                }
            }
        } else if c == '~' {
            while let Some(top) = operator_stack.last() {
                if top == "~" || top == "!" {
                    break;
                } else {
                    operator_stack.push(String::from(c));
                    break;
                }
            }
        } else if c == '!' {
            while let Some(top) = operator_stack.last() {
                if top == "~" || top == "!" {
                    break;
                } else {
                    operator_stack.push(String::from(c));
                    break;
                }
            }
        } else if !c.is_whitespace() {
            eprintln!("Invalid token: {}", c);
        }
    }

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }

    output_queue.join(" ")
}

fn precedence(op: &str) -> u8 {
    match op {
        "~" | "!" => 5,       // negation first
        "*" | "/" | "%" => 4, // then multiplication / division / modulo
        "+" | "-" => 3,       // then addition / subtraction
        "&" => 2,             // then and
        "^" => 1,             // then xor
        "|" => 0,             // then or
        _ => 1,               // default
    }
}

fn evaluate_rpn(expression: &str) -> Result<f64, &'static str> {
    let mut stack: Vec<f64> = Vec::new();

    for token in expression.split_whitespace() {
        if let Ok(number) = token.parse::<f64>() {
            stack.push(number);
        } else {
            let (a, b) = match (stack.pop(), stack.pop()) {
                (Some(x), Some(y)) => (y, x),
                _ => return Err("Insufficient operands"),
            };
            match token {
                "+" => stack.push(a + b),
                "-" => stack.push(a - b),
                "*" => stack.push(a * b),
                "/" => {
                    if b == 0.0 {
                        return Err("Division by zero");
                    } else {
                        stack.push(a / b);
                    }
                }
                "^" => stack.push((a as i64 ^ b as i64) as f64),
                "&" => stack.push((a as i64 & b as i64) as f64),
                "|" => stack.push((a as i64 | b as i64) as f64),
                "%" => stack.push(a % b),
                _ => return Err("Invalid operator"),
            }
        }
    }

    match stack.pop() {
        Some(result) => Ok(result),
        None => Err("Expression is empty"),
    }
}

fn rpn_to_infix(expression: &str) -> String {
    let mut stack: Vec<String> = Vec::new();

    for token in expression.split_whitespace() {
        if let Ok(number) = token.parse::<f64>() {
            stack.push(number.to_string());
        } else if token == "i" {
            stack.push(String::from("i"));
        } else if token == "j" {
            stack.push(String::from("j"));
        } else {
            let (a, b) = match (stack.pop(), stack.pop()) {
                (Some(x), Some(y)) => (y, x),
                _ => return String::from("Invalid expression"),
            };
            let operator = match token {
                "+" | "-" | "*" | "/" | "%" | "^" | "&" | "|" => token.to_string(),
                "~" => format!("~{}", a),
                "!" => format!("!{}", a),
                _ => return String::from("Invalid operator"),
            };
            let expr = match operator.as_str() {
                "~" | "!" => format!("{}({})", operator, a),
                _ => format!("({}{}{})", a, operator, b),
            };
            stack.push(expr);
        }
    }

    match stack.pop() {
        Some(result) => result,
        None => String::from("Invalid expression"),
    }
}
