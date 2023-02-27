use std::{
    fs::{self, OpenOptions},
    io::Write,
};

const VAR: char = 'x'; // i
const CONST: char = '2'; // 1, 2, 3, 4, 5, 6, 7, 8, 9, 0
const CHANGE: char = 'i'; // i++, i--
const RIGHT_ARG_OPERATOR: char = '~'; // ~i, !i, ++i, --i, -i
const TWO_ARG_OPERATOR: char = '*'; // 0+1, 0-1, 0*1, 0/1, 0**1, 0%1, 0&&1, 0&1, 0||1, 0|1, 0<<1, 0>>1, 0^1
const LEFT_PARENS: char = '(';
const RIGHT_PARENS: char = ')';
const PATTERNS_TXT_PATH: &str = "./patterns.txt";
const PATTERN_CHARS: [char; 6] = [
    VAR,
    CONST,
    // CHANGE,
    RIGHT_ARG_OPERATOR,
    TWO_ARG_OPERATOR,
    LEFT_PARENS,
    RIGHT_PARENS,
];

// export module
pub fn generate_patterns(min_length: usize, max_length: usize) {
    fs::write(PATTERNS_TXT_PATH, "").expect("Failed to write file.");
    let mut file = OpenOptions::new()
        .append(true)
        .open(PATTERNS_TXT_PATH)
        .expect("Unable to open file");
    let mut map = Vec::new();

    for i in min_length..=max_length {
        println!("Generating patterns of length: {}", i);
        let word = PATTERN_CHARS[0].to_string().repeat(i);
        let mut word_chars: Vec<char> = word.chars().collect();

        for j in 1..(PATTERN_CHARS.len().pow(i as u32)) {
            for k in 0..i {
                if j % PATTERN_CHARS.len().pow(k as u32) == 0 {
                    let char_ = word_chars[k];
                    let char_index = PATTERN_CHARS.iter().position(|&c| c == char_).unwrap() + 1;
                    let next_char = PATTERN_CHARS.get(char_index).unwrap_or(&PATTERN_CHARS[0]);
                    let next_chars: Vec<char> = next_char.to_string().chars().collect();

                    word_chars.splice(
                        (k as usize)..(k as usize + next_chars.len()),
                        next_chars.iter().cloned(),
                    );
                }
            }

            let generated_word: String = word_chars.iter().collect();
            if check_pattern(&generated_word) {
                map.push(generated_word.clone());
                // append to file
            }
        }
    }
    map.sort_by_key(|s| s.matches("2").count());
    for pattern in map {
        file.write_all(format!("{}\n", pattern).as_bytes())
            .expect("Unable to write data");
    }
}

fn check_pattern(pattern: &str) -> bool {
    let last = pattern.len() - 1;

    !(!pattern.contains(VAR) ||
      pattern.chars().nth(0).unwrap() == TWO_ARG_OPERATOR || // ^*
      pattern.chars().nth(last).unwrap() == RIGHT_ARG_OPERATOR || // !$
      pattern.chars().nth(last).unwrap() == TWO_ARG_OPERATOR || // *$
      pattern.ends_with(&format!("{}{}", CONST, CHANGE)) || // 2++$
      pattern.ends_with(&format!("{}{}{}", VAR, TWO_ARG_OPERATOR, CHANGE)) || // i*++$
      pattern.ends_with(&format!("{}{}", TWO_ARG_OPERATOR, CHANGE)) || // *++$
      pattern.contains(&format!("{}{}", VAR, CONST)) || // 3x
      pattern.contains(&format!("{}{}", CONST, VAR)) || // x3
      pattern.contains(&format!("{}{}", VAR, VAR)) || // xx
      pattern.contains(&format!("{}{}", TWO_ARG_OPERATOR, TWO_ARG_OPERATOR)) || // %%
      pattern.contains(&format!("{}{}", RIGHT_ARG_OPERATOR, RIGHT_ARG_OPERATOR)) || // !!
      pattern.contains(&format!("{}{}", CHANGE, CHANGE)) || // ++++
      pattern.contains(&format!("{}{}", RIGHT_ARG_OPERATOR, TWO_ARG_OPERATOR)) || // !*
      pattern.contains(&format!("{}{}", VAR, RIGHT_ARG_OPERATOR)) || // x!
      pattern.contains(&format!("{}{}", CONST, CHANGE)) || // 2++
      pattern.contains(&format!("{}{}", CHANGE, CONST)) || // ++2
      pattern.contains(&format!("{}{}", RIGHT_ARG_OPERATOR, CONST)) || // !2
      pattern.contains(&format!("{}{}", CONST, RIGHT_ARG_OPERATOR)) || // 2!
      pattern.contains(&format!("{}{}", CHANGE, TWO_ARG_OPERATOR)) || // ++*
      pattern.contains(&format!("{}{}{}", VAR, CHANGE, VAR)) || // x++x
      pattern.contains(&format!("{}{}", CHANGE, RIGHT_ARG_OPERATOR)) || // ++!
      pattern.contains(&format!("{}{}{}", CHANGE, VAR, CHANGE)) || // ++x++
      pattern.contains(&format!("{}{}", RIGHT_ARG_OPERATOR, CHANGE)) || // !++
      pattern.contains(&format!("{}{}{}{}{}", VAR, TWO_ARG_OPERATOR, VAR, TWO_ARG_OPERATOR, VAR)) || // !++
      pattern.starts_with(&format!("{}{}{}{}{}", CONST, TWO_ARG_OPERATOR, CONST, TWO_ARG_OPERATOR, CONST)) || // !++
      pattern.starts_with(&format!("{}{}{}{}", CONST, CONST, TWO_ARG_OPERATOR, CONST)) || // !++
      pattern.starts_with(&format!("{}{}{}{}{}{}{}{}", CONST, CONST, TWO_ARG_OPERATOR, CONST, TWO_ARG_OPERATOR, CONST, TWO_ARG_OPERATOR, CONST)) || // !++
      // pattern.matches(VAR).count() > 1 || // !++
      // !pattern.contains(LEFT_PARENS) ||
      // pattern.contains(&format!("{}{}{}", CONST, TWO_ARG_OPERATOR, CONST)) || // 2*2
      pattern.contains(&format!("{}", CONST).repeat(3)) ||
      !is_valid_parens(pattern)) // 2222
}

fn is_valid_parens(pattern: &str) -> bool {
    let left_parens_index = pattern.find(LEFT_PARENS);
    let right_parens_index = pattern.find(RIGHT_PARENS);

    if let (Some(left), Some(right)) = (left_parens_index, right_parens_index) {
        if left < right {
            let sub_pattern = &pattern[left + 1..right];
            return !((!pattern.contains(&format!("{}{}", TWO_ARG_OPERATOR, LEFT_PARENS))
                && !pattern.starts_with(LEFT_PARENS))
                || (!pattern.contains(&format!("{}{}", RIGHT_PARENS, TWO_ARG_OPERATOR))
                    && !pattern.ends_with(RIGHT_PARENS))
                || (!pattern.contains(&format!("{}{}", LEFT_PARENS, VAR))
                    && !pattern.contains(&format!("{}{}", LEFT_PARENS, CONST)))
                || (!pattern.contains(&format!("{}{}", VAR, RIGHT_PARENS))
                    && !pattern.contains(&format!("{}{}", CONST, RIGHT_PARENS)))
                || pattern.contains(&format!("{}{}", LEFT_PARENS, RIGHT_PARENS))
                || pattern.contains(&format!(
                    "{}{}{}",
                    LEFT_PARENS, TWO_ARG_OPERATOR, RIGHT_PARENS
                ))
                || pattern.contains(&format!(
                    "{}{}{}",
                    LEFT_PARENS, RIGHT_ARG_OPERATOR, RIGHT_PARENS
                ))
                || pattern.contains(&format!("{}{}{}", LEFT_PARENS, CHANGE, RIGHT_PARENS))
                || pattern.contains(&format!("{}{}{}", LEFT_PARENS, VAR, RIGHT_PARENS))
                || pattern.contains(&format!("{}{}{}", LEFT_PARENS, CONST, RIGHT_PARENS))
                || pattern.contains(&format!(
                    "{}{}{}{}",
                    LEFT_PARENS, CONST, CONST, RIGHT_PARENS
                ))
                || (pattern.starts_with(LEFT_PARENS) && pattern.ends_with(RIGHT_PARENS))
                || (!sub_pattern.contains(VAR))
                || pattern.matches(LEFT_PARENS).count() > 1
                || pattern.matches(LEFT_PARENS).count() != pattern.matches(RIGHT_PARENS).count());
        }
    }

    !pattern.contains(LEFT_PARENS) && !pattern.contains(RIGHT_PARENS)
}
