#![allow(dead_code)]

use crate::utils;
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
    is_last: bool,
}

type NodeRef = Rc<RefCell<Node>>;

impl Node {
    fn new(name: String) -> Self {
        Node {
            name,
            left: None,
            right: None,
            is_last: false,
        }
    }
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 8, puzzle 1");

    let input = utils::file::read_input("src/day08/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let instructions = lines[0].chars().collect::<Vec<char>>();

    let mut node_map: HashMap<String, NodeRef> = HashMap::new();
    let mut first_node: Option<NodeRef> = None;

    for (i, line) in lines.iter().enumerate().skip(2) {
        let name = line[0..3].to_string();

        let node = Rc::new(RefCell::new(Node::new(name.clone())));

        if name == "AAA" {
            first_node = Some(Rc::clone(&node));
        } else if name == "ZZZ" {
            node.borrow_mut().is_last = true;
        }

        node_map.insert(name.clone(), node);
    }

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

    let mut steps: u64 = 1;
    let mut repeates: u64 = 0;
    let n = first_node.unwrap();
    let mut current_node = Rc::clone(&n);
    let mut found = false;

    println!("Instructions: {:?}", instructions.len());

    while !found {
        for instruction in &instructions {
            if *instruction == 'L' {
                let left = current_node.borrow().left.clone().unwrap();
                current_node = left;
            } else if *instruction == 'R' {
                let right = current_node.borrow().right.clone().unwrap();
                current_node = right;
            }

            if current_node.borrow().is_last {
                found = true;
                break;
            }

            steps += 1;
        }
        repeates += 1;

        println!("Repeates: {} steps: {}", repeates, steps);
    }

    println!("Steps: {}", steps);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 8, puzzle 2");

    println!("Time elapsed: {:?}", start.elapsed());
}
