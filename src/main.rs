use std::process::Command;
use std::cell::RefCell;

extern crate serde_json;
extern crate alt_tabber;

use alt_tabber::i3::{ Node, Type };
use serde_json::{from_slice};


fn main() {
    let output = Command::new("i3-msg")
        .arg("-t")
        .arg("get_tree")
        .output()
        .unwrap();

    let nodes: Node = from_slice(&output.stdout).unwrap();
    // println!("{:#?}", nodes);

    let ws = RefCell::new(&nodes);
    let result = RefCell::new(&nodes);

    find_focused_ws(&nodes, &ws, &result);
    println!("Focuses Workspace: {:?}", result);
    // Find Focused node

    // From there find parent workspace

    // Build up focused array
}

fn flattenNodes(root: &Node) {

    for node in root.nodes.iter() {
        match node {
            Node { focused: true, ..} => println!("Focused Node: {:?}", node),
            Node { type_con: Type::Workspace, ..} => println!("Workspace Node {:?}", node),
            _ => println!("{:?}", root),
        }

        flattenNodes(&node)
    }
}

fn find_focused_ws<'a>(root: &'a Node, ws: &RefCell<&'a Node>, result: &RefCell<&'a Node>) {
    for node in root.nodes.iter() {
        match node {
            Node { focused: true, ..} => {
                println!("Focused Node: {:?}", node);
                *result.borrow_mut() = &ws.borrow();
                break;
            },
            Node { type_con: Type::Workspace, ..} => {
                println!("Workspace Node {:?}", node);
                *ws.borrow_mut() = &node
            },
            _ => println!("{:?}", root),
        }
        find_focused_ws(&node, ws, result);
    }
}
