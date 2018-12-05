use std::fs::File;
use std::io::Read;
use std::str;
use std::usize;

fn main() {
    let data = setup();
    println!("Size (a): {}", q1(data.to_owned()));

    let smallest = (('a' as u8)..('z' as u8 + 1))
        .fold(usize::MAX, |smallest, letter| {
              let upper_letter = (letter as u8) | 0x20;
              let current_data = data.to_owned()
                  .iter()
                  .filter(|it| (**it) | 0x20 != upper_letter)
                  .map(|it| *it)
                  .collect::<Vec<u8>>();
              let current = q1(current_data);
              if current < smallest {
                  current
              } else {
                  smallest
              }
        });
    println!("Smallest (b): {}", smallest);
}

fn setup() -> Vec<u8> {
    let mut file = File::open("./input.txt").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn q1(mut vec: Vec<u8>) -> usize {
    let mut has_modifications = true;
    while has_modifications {
        has_modifications = false;

        let mut to_remove = Vec::new();
        let mut prev = 0;
        let mut num_removed = 0;
        for (i, value) in vec.iter_mut().enumerate() {
            if prev == 0 {
                prev = *value;
                continue;
            }

            let diff = (*value as i32 - prev as i32).abs();
            prev = *value;

            if diff == 0x20 {
                to_remove.push(i - 1 - num_removed);
                num_removed += 2;
                prev = 0;
            }
        }

        for item in to_remove {
            vec.remove(item);
            vec.remove(item);
            has_modifications = true;
        }
    }

    str::from_utf8(&vec).unwrap().trim().len()
}
