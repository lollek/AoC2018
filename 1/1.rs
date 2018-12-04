use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let numbers = setup();
    q1(&numbers);
    q2(&numbers);
}

fn setup() -> Vec<i64> {
    let file = File::open("./1.input.txt").unwrap();
    let inputbuf = BufReader::new(file);
    inputbuf.lines().map(|i| i.unwrap().parse::<i64>().unwrap()).collect()
}

fn q1(numbers: &Vec<i64>) {
    let mut value = 0;
    for num in numbers {
        value += num;
    }
    println!("q1: {}", value);
}

fn q2(numbers: &Vec<i64>) {
    let mut value = 0;
    let mut values = HashSet::new();

    'outer: loop {
        for num in numbers {
            match values.get(&value) {
                Some(_) => break 'outer,
                None => values.insert(value),
            };
            value += num;
        }
    }

    println!("q2: {}", value);
}
