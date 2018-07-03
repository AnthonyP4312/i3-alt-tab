use std::collections::VecDeque;
// use std::boxed::Box;
use std::process::Command;
// use std::cell::RefCell;

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

    // Find Focused node
    // let empty_node = Node {
    //     name: Some(String::from("rip")),
    //     type_con: Type::Root,
    //     focused: false,
    //     window: Some(0),
    //     nodes: vec![],
    // };

    let ws = queued_ws(nodes);
    println!("Got the workspace {:?}", ws)
    // From there find parent workspace

    // Build up focused array
}

fn queued_ws(root: Node) -> Node{
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut ws: Node = root.clone();
    queue.push_back(root);
    loop {
        let this_node = queue.pop_back().unwrap();
        for node in this_node.nodes.iter() {
            queue.push_back(node.clone());
        }
        match this_node {
            Node { focused: true, ..} => {
                println!("Focused Node: {:?}", this_node);
                break;
            },
            Node { type_con: Type::Workspace, ..} => {
                println!("Workspace Node {:?}", this_node);
                ws = this_node
            },
            _ => println!("Regular Node: {:?}", this_node),
        }
    }
    ws
}

// Complains about the recursion at the end moving data we we have to contain the mutation in a refcell
// fn find_focused_ws<'a, 'b>(root: &'a Node, ws: RefCell<&'a Node>, result: RefCell<&'b Node>) {
//     for node in root.nodes.iter() {
//         match node {
//             Node { focused: true, ..} => {
//                 println!("Focused Node: {:?}", node);
//                 *result.borrow_mut() = ws.borrow().clone();
//                 break;
//             },
//             Node { type_con: Type::Workspace, ..} => {
//                 println!("Workspace Node {:?}", node);
//                 *ws.borrow_mut() = node
//             },
//             _ => (),
//         }
//         find_focused_ws(&node, ws, result);
//     }
// }

// Since this uses a pre-order traversal the borrow checker seems to think its fine
// Maybe because it literally doesnt work
// For some reason i cant mutate focus outside of the scope of the function
// Maybe this is what cells are for
// fn find_focused_ws_pre<'a>(root: &'a Node, mut ws: &'a Node, mut focus: Box<bool>) {
//     for node in root.nodes.iter() {
//         find_focused_ws_pre(&node, &ws, focus);
//         println!("{}", focus);
//         match node {
//             Node { focused: true, ..} => {
//                 println!("Focused Node: {:?}", node);
//                 *focus = true;
//                 println!("{}", focus);
//             },
//             Node { type_con: Type::Workspace, ..} if *focus => {
//                 println!("Workspace Node {:?}", node);
//                 ws = node;
//             },
//             _ => (),
//         }
//     }
// }
