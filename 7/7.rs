use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Node {
    requirements: Vec<u8>,
}

impl Node {
    fn new() -> Self {
        Node {
            requirements: vec![],
        }
    }
}

fn main() {
    let lines = setup();
    let nodes = connect_nodes(lines);
    q1(&nodes);
    q2(&nodes);
}

fn setup() -> Vec<(u8, u8)> {
    let file = File::open("./input.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            (split[1].as_bytes()[0], split[7].as_bytes()[0])
        })
        .collect()
}

fn connect_nodes(tuples: Vec<(u8, u8)>) -> HashMap<u8, Node> {
    let mut collection = HashMap::new();
    for (req, this) in tuples.into_iter() {
        collection.entry(req).or_insert(Node::new());
        let this_node = collection.entry(this).or_insert(Node::new());

        this_node.requirements.push(req);
    }
    collection
}

fn q1(nodes: &HashMap<u8, Node>) {
    print!("q1: ");
    let mut counted_nodes = HashSet::new();

    while counted_nodes.len() != nodes.len() {
        let mut uncounted_nodes = nodes.iter()
            .filter(|(key, value)| {
                if counted_nodes.contains(key) {
                    return false;
                }
                value.requirements.iter().all(|it| counted_nodes.contains(&it))
            })
        .map(|(key, _)| key)
        .collect::<Vec<&u8>>();

        uncounted_nodes.sort();
        let node_to_count = uncounted_nodes.get(0).unwrap();
        counted_nodes.insert(node_to_count.to_owned());

        print!("{}", **node_to_count as char);
    }
    println!();
}

#[derive(Copy, Clone)]
struct Worker {
    node: Option<u8>,
    work_left: u8,
}

impl Worker {
    fn new() -> Self {
        Worker {
            node: None,
            work_left: 0,
        }
    }
}

fn q2(nodes: &HashMap<u8, Node>) {
    print!("q2: ");
    let mut counted_nodes = HashSet::new();
    let mut workers = [Worker::new(); 5];
    let mut step_counter = 0;

    while counted_nodes.len() != nodes.len() {
        let nodes_in_progress = workers.iter()
            .filter(|it| !it.node.is_none())
            .map(|it| it.node.unwrap())
            .collect::<Vec<u8>>();

        let mut uncounted_nodes = nodes.iter()
            .filter(|(key, value)| {
                if counted_nodes.contains(*key) {
                    return false;
                }
                if nodes_in_progress.contains(key) {
                    return false;
                }
                value.requirements.iter().all(|it| counted_nodes.contains(it))
            })
        .map(|(key, _)| key)
        .collect::<Vec<&u8>>();

        uncounted_nodes.sort_by(|a, b| b.cmp(a));

        for worker in workers.iter_mut() {
            if let None = worker.node {
                if let Some(node_key) = uncounted_nodes.pop() {
                    worker.node = Some(node_key.to_owned());
                    worker.work_left = node_key - ('A' as u8) + 61;
                }
            }

            if let Some(node) = worker.node {
                worker.work_left -= 1;
                if worker.work_left <= 0 {
                    worker.node = None;
                    counted_nodes.insert(node);
                    print!("{}", node as char);
                }
            }
        }

        step_counter += 1;
    }
    println!(" - {} steps", step_counter);
}
