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

    // From there find parent workspace
    let ws = queued_ws(nodes);

    // Build up array of windows on focused WS
    let mut windows = flatten_children(ws);

    // Sort the windows Highest to Lowest, then Left to Right
    windows.sort_by(|a, b| a.rect.y.cmp(&b.rect.y));
    windows.sort_by(|a, b| a.rect.x.cmp(&b.rect.x));

    // Find the next in line (wrapping to front if last window)
    let focused_index = windows.iter().position(|n| n.focused).unwrap();
    let next = if focused_index < windows.len() - 1 {
        windows.iter().nth(focused_index + 1).unwrap()
    } else {
        windows.iter().nth(0).unwrap()
    };

    // Send Message
    Command::new("i3-msg")
        .arg(format!("[con_id={}] focus", next.id))
        .output();
}

fn flatten_children(ws: Node) -> Vec<Node> {
    let mut children = vec![];
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_back(ws);
    loop {
        let this_node = queue.pop_back().unwrap();
        for node in this_node.nodes.iter() {
            match node {
                Node { window: Some(_), ..} => {
                    children.push(node.clone());
                },
                _ => queue.push_back(node.clone())
            }
        }
        if queue.is_empty() { break; }
    }
    children
}

fn queued_ws(root: Node) -> Node{
    // Lots of cloning, probably a better way to do this.
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
                break;
            },
            Node { type_con: Type::Workspace, ..} => {
                ws = this_node
            },
            _ => (),
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
