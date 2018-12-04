use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let coords = setup();
    q1(&coords);
}

#[derive(Debug)]
struct Coordinate {
    name: String,
    start_x: i32,
    start_y: i32,
    width: i32,
    height: i32,
}

impl Coordinate {
     fn from(string: String) -> Self {
         let parts = string.split_whitespace().collect::<Vec<&str>>();
         let (num, coordinate, size) = (parts[0], parts[2], parts[3]);

         let xy_parts = coordinate.split(|c| c == ',' || c ==  ':').collect::<Vec<&str>>();
         let (start_x, start_y) = (xy_parts[0], xy_parts[1]);

         let wh_parts = size.split('x').collect::<Vec<&str>>();
         let (width, height) = (wh_parts[0], wh_parts[1]);

         Coordinate {
             name: num.to_string(),
             start_x: start_x.parse::<i32>().unwrap(),
             start_y: start_y.parse::<i32>().unwrap(),
             width: width.parse::<i32>().unwrap(),
             height: height.parse::<i32>().unwrap(),
         }
     }
}

fn setup() -> Vec<Coordinate> {
    let file = File::open("./input.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(Coordinate::from)
        .collect()
}

fn q1(coords: &Vec<Coordinate>) {
    let mut map = HashMap::new();
    fn xy_to_index(x: i32, y: i32) -> i32 { x + (y * 10000) };

    for coord in coords {
        for w in 0..coord.width {
            for h in 0..coord.height {
                let entry = map.entry(xy_to_index(coord.start_x + w, coord.start_y + h))
                    .or_insert(0);
                *entry += 1;
            }
        }
    }

    let counter = map.values().filter(|&i| i > &1).fold(0, |sum, _| sum + 1);
    println!("q1: {}", counter);

    // q2
    'outer: for coord in coords {
        for w in 0..coord.width {
            for h in 0..coord.height {
                let entry = map.get(&(xy_to_index(coord.start_x + w, coord.start_y + h)))
                    .unwrap();
                if entry > &1 {
                    continue 'outer;
                }
            }
        }
        println!("q2: {:?}", coord);
        break;
    }
}
