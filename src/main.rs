use std::collections::VecDeque;
use std::process::Command;

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
    let next = if focused_index == 0 {
        windows.iter().last().unwrap()
    } else {
        windows.iter().nth(focused_index - 1).unwrap()
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

#[cfg(test)]
mod test_flatten {
    use alt_tabber::i3::Rect;
    use super::*;
    fn setup() {
        let child_a = Node {
            name: Some(String::from("child_a")),
            id: 123,
            type_con: Type::Con,
            rect: Rect {
                x: 0,
                y: 0,
                height: 0,
                width: 0
            },
            focused: true,
            window: Some(123),
            nodes: vec![]
        };
        let child_b = Node {
            ..child_a.clone()
        };
        let child_c = Node {
            nodes: vec![child_a.clone(), child_b],
            ..child_a.clone()
        };
        let node_with_children = Node {
            ..child_a.clone()
        };
        return ()
    }
    #[test]
    fn flattens() {

    }
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
