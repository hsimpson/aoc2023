#![allow(dead_code)]

use num::integer::lcm;

use crate::utils;
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

type NodeRef = Rc<RefCell<Node>>;

impl Node {
    fn new(name: String) -> Self {
        Node {
            name,
            left: None,
            right: None,
        }
    }
}

fn node_map(lines: &[&str]) -> HashMap<String, NodeRef> {
    let mut node_map: HashMap<String, NodeRef> = HashMap::new();

    for line in lines.iter().skip(2) {
        let name = line[0..3].to_string();
        let node = Rc::new(RefCell::new(Node::new(name.clone())));
        node_map.insert(name.clone(), node);
    }

    node_map
}

fn build_tree(lines: &[&str], node_map: &HashMap<String, NodeRef>) {
    for line in lines.iter().skip(2) {
        let name = line[0..3].to_string();
        let node = node_map.get(&name).unwrap();

        let left_name = line[7..10].to_string();
        let right_name = line[12..15].to_string();

        let left_node = node_map.get(&left_name).unwrap();
        let right_node = node_map.get(&right_name).unwrap();

        node.borrow_mut().left = Some(Rc::clone(left_node));
        node.borrow_mut().right = Some(Rc::clone(right_node));
    }
}

fn start_nodes(lines: &[&str], node_map: &HashMap<String, NodeRef>) -> Vec<NodeRef> {
    let mut start_nodes: Vec<NodeRef> = Vec::new();

    for line in lines.iter().skip(2) {
        let name = line[0..3].to_string();

        if name.ends_with('A') {
            let node = node_map.get(&name).unwrap();
            start_nodes.push(node.clone());
        }
    }

    start_nodes
}

fn steps_to_zzz(node: &NodeRef, instructions: &[char]) -> u32 {
    let mut steps = 1;
    // let mut repeates: u32 = 0;
    let mut current_node = Rc::clone(node);
    let mut found = false;

    while !found {
        for instruction in instructions {
            if *instruction == 'L' {
                let left = current_node.borrow().left.clone().unwrap();
                current_node = left;
            } else if *instruction == 'R' {
                let right = current_node.borrow().right.clone().unwrap();
                current_node = right;
            }

            if current_node.borrow().name == "ZZZ" {
                found = true;
                break;
            }

            steps += 1;
        }
        // repeates += 1;
        // println!("Repeates: {} steps: {}", repeates, steps);
    }

    steps
}

fn steps_to_xxz(node: &NodeRef, instructions: &[char]) -> u32 {
    let mut steps = 1;
    // let mut repeates: u32 = 0;
    let mut current_node = Rc::clone(node);
    let mut found = false;

    while !found {
        for instruction in instructions {
            if *instruction == 'L' {
                let left = current_node.borrow().left.clone().unwrap();
                current_node = left;
            } else if *instruction == 'R' {
                let right = current_node.borrow().right.clone().unwrap();
                current_node = right;
            }

            if current_node.borrow().name.ends_with('Z') {
                found = true;
                break;
            }

            steps += 1;
        }
        // repeates += 1;
        // println!("Repeates: {} steps: {}", repeates, steps);
    }

    steps
}

fn lcmm(numbers: &mut Vec<u64>) -> u64 {
    if numbers.len() == 2 {
        return lcm(numbers[0], numbers[1]);
    }
    let first = numbers.remove(0);
    lcm(first, lcmm(numbers))
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 8, puzzle 1");

    let input = utils::file::read_input("src/day08/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let instructions = lines[0].chars().collect::<Vec<char>>();

    let node_map = node_map(&lines);
    let first_node = node_map.get("AAA").unwrap();

    build_tree(&lines, &node_map);

    let steps = steps_to_zzz(first_node, &instructions);

    println!("Steps: {}", steps);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 8, puzzle 2");

    let input = utils::file::read_input("src/day08/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let instructions = lines[0].chars().collect::<Vec<char>>();

    let node_map = node_map(&lines);
    build_tree(&lines, &node_map);

    let start_nodes = start_nodes(&lines, &node_map);

    let mut steps_list: Vec<u64> = Vec::new();

    for node in start_nodes {
        let steps = steps_to_xxz(&node, &instructions);
        steps_list.push(steps as u64);
    }

    let steps = lcmm(&mut steps_list);

    println!("Steps: {}", steps);
    println!("Time elapsed: {:?}", start.elapsed());
}
