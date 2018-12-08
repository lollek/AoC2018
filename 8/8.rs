use std::fs::File;
use std::io::Read;

fn main() {
    let data = setup();
    let head = parse_next_node(&mut data.iter());
    q1(&head);
    q2(&head);
}

fn setup() -> Vec<u8> {
    let mut file_as_string = String::new();
    File::open("./input.txt").unwrap()
        .read_to_string(&mut file_as_string).unwrap();
    file_as_string.split_whitespace()
        .map(|num| num.parse::<u8>().unwrap())
        .collect()
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
}

fn parse_next_node<'a, I>(iter: &mut I) -> Node
    where I: Iterator<Item = &'a u8>
{
    let num_child_nodes = iter.next().unwrap();
    let num_metadata_entries = iter.next().unwrap();
    Node {
        children: (0..*num_child_nodes).map(|_| parse_next_node(iter)).collect(),
        metadata: (0..*num_metadata_entries).map(|_| iter.next().unwrap().to_owned()).collect(),
    }
}

fn sum_metadata(node: &Node) -> u64 {
    let metadata_sum: u64 = node.metadata.iter().sum::<u8>() as u64;
    let children_metadata_sum = node.children.iter().map(|it| sum_metadata(&it)).sum::<u64>();
    metadata_sum + children_metadata_sum
}

fn q1(head: &Node) {
    let metadata_sum = sum_metadata(&head);
    println!("q1: {}", metadata_sum);
}

fn sum_metadata_q2(node: &Node) -> u64 {
    if node.children.is_empty() {
        return node.metadata.iter().sum::<u8>() as u64;
    }

    node.metadata.iter().fold(0, |sum, i| {
        if i == &0 {
            sum
        } else if let Some(child) = node.children.get(*i as usize - 1) {
            sum + sum_metadata_q2(child)
        } else {
            sum
        }
    })
}

fn q2(head: &Node) {
    let metadata_sum = sum_metadata_q2(&head);
    println!("q2: {}", metadata_sum);
}
