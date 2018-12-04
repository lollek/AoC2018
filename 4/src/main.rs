extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq)]
enum Action {
    BeginShift,
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Log {
    monthdate: String,
    minute: i32,
    action: Action,
    guard: Option<i32>,
}

fn main() {
    let lines = setup();
    q1(&lines);
}

fn setup() -> Vec<Log> {
    let file = File::open("./input.txt").unwrap();
    let line_re = Regex::new(r"\[\d{4}-(\d{2}-\d{2}) (\d{2}):(\d{2})\] (.+)").unwrap();
    let guard_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut lines = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    lines.sort_unstable();

    lines.iter()
        .map(|line| {
            let captures = line_re.captures(&line).unwrap();

            let monthdate = captures[1].to_owned();
            let hour = &captures[2];
            let minute =
                if hour == "00" {
                    captures[3].parse::<i32>().unwrap().to_owned()
                } else {
                    0
                };
            let action = match &captures[4] {
                "wakes up" => Action::WakeUp,
                "falls asleep" => Action::FallAsleep,
                _ => Action::BeginShift,
            };
            let guard =
                if action == Action::BeginShift {
                    Some(guard_re.captures(&captures[4])
                         .unwrap()[1]
                         .parse::<i32>()
                         .unwrap()
                         .to_owned())
                } else {
                    None
                };

            Log {
                monthdate: monthdate,
                minute: minute,
                action: action,
                guard: guard,
            }
        })
        .collect()
}

fn q1(events: &Vec<Log>) {
    let mut map = HashMap::new();
    let mut curr_guard: Option<i32> = None;

    for event in events {

        match event.action {
            Action::BeginShift => {
                curr_guard = Some(event.guard.unwrap());
                map.entry(curr_guard.unwrap())
                    .or_insert(HashMap::new())
                    .entry(&event.monthdate)
                    .or_insert([false; 60]);
            },
            Action::FallAsleep => {
                let mut curr_shift = map.entry(curr_guard.unwrap())
                    .or_insert(HashMap::new())
                    .entry(&event.monthdate)
                    .or_insert([false; 60]);
                for minute in event.minute..60 {
                    curr_shift[minute as usize] = true;
                }
            },
            Action::WakeUp => {
                let mut curr_shift = map.entry(curr_guard.unwrap())
                    .or_insert(HashMap::new())
                    .entry(&event.monthdate)
                    .or_insert([false; 60]);
                for minute in event.minute..60 {
                    curr_shift[minute as usize] = false;
                }
            }
        }
    }

    println!("Sum: Total amount of minutes asleep
Same-minute: Total number of minutes asleep as a single minute
Same-minute-id: Which minute it was most often asleep at

Guard number |  Sum | Same-minute | Same-minute-id
--------------------------------------------------");
    for (guard, shifts) in map.iter() {
        let mut counter = 0;
        let mut minutes = HashMap::new();

        for shift in shifts.values() {
            for (minute, was_asleep) in shift.iter().enumerate() {
                if *was_asleep {
                    counter += 1;
                    let min_counter = minutes.entry(minute).or_insert(0);
                    *min_counter += 1;
                }
            }
        }

        let mut most_asleep = 0;
        let mut most_asleep_minute = 0;
        for (minute, count) in minutes.iter() {
            if *count > most_asleep {
                most_asleep = *count;
                most_asleep_minute = *minute;
            }
        }

        println!("Guard {:6} | {:4} | {:11} | {:13}", guard, counter, most_asleep, most_asleep_minute);
    }
}
