use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let strings = setup();
    q1(&strings);
    q2(&strings);
}

fn setup() -> Vec<String> {
    let file = File::open("./input.txt").unwrap();
    let inputbuf = BufReader::new(file);
    inputbuf.lines().map(Result::unwrap).collect()
}

fn q1(strings: &Vec<String>) {
    let mut twos = 0;
    let mut threes = 0;

    for string in strings {
        let mut values = HashMap::new();
        for letter in string.chars() {
            let counter = values.entry(letter).or_insert(0);
            *counter += 1;
        }

        for counter_value in values.values() {
            if *counter_value == 2 {
                twos += 1;
                break;
            }
        }
        for counter_value in values.values() {
            if *counter_value == 3 {
                threes += 1;
                break;
            }
        }
    }

    println!("q1: Twos: {}. Threes: {}. Sum: {}", twos, threes, twos * threes);
}

fn q2(strings: &Vec<String>) {
    for string_index in 0..strings.len() {
        let prev_strings = &strings[0..string_index];
        let curr_string = &strings[string_index];

        for prev_string in prev_strings {
            let mut matches = 0;
            for it in curr_string.chars().zip(prev_string.chars()) {
                let (this, other) = it;
                if this == other {
                    matches += 1;
                }
            }

            if matches == curr_string.len() - 1 {
                let result = curr_string.chars().zip(prev_string.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _b)| a)
                    .collect::<String>();
                println!("q2: {} and {} matches, giving {}", curr_string,
                         prev_string, result);
            }
        }
    }
}
