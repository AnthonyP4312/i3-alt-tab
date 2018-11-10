use std::collections::VecDeque;
use std::env;
use std::process::Command;

extern crate i3_alt_tab;
extern crate serde_json;
extern crate structopt;

use i3_alt_tab::i3::{
    WSOptions,
    Direction,
    Args,
    Node,
    Type,
    Workspace,
};
use serde_json::from_slice;
use Direction::*;
use structopt::StructOpt;

fn main() {
    // Pull our node tree from i3
    let args = Args::from_args();
    println!("{:?}", args);

    // Pull node tree from i3
    let output = Command::new("i3-msg")
        .arg("-t")
        .arg("get_tree")
        .output()
        .unwrap();
    let nodes: Node = from_slice(&output.stdout).unwrap();


    // Pull workspaces from i3
    let output = Command::new("i3-msg")
        .arg("-t")
        .arg("get_workspaces")
        .output()
        .unwrap();
    let workspaces: Vec<Workspace> = from_slice(&output.stdout).unwrap();
    println!("{:?}", workspaces);


    // From there find parent workspace
    let ws = queued_ws2(&nodes).unwrap();

    // Build up array of windows on focused WS
    let mut windows = flatten_children(ws);

    // Sort the windows Highest to Lowest, then Left to Right
    windows.sort_by(|a, b| a.rect.y.cmp(&b.rect.y));
    windows.sort_by(|a, b| a.rect.x.cmp(&b.rect.x));

    // Find the next in line (wrapping to front if last window)
    let focused_index = windows.iter().position(|n| n.focused).unwrap();

    // Determine whether we're moving back or forwards
    let args: Vec<String> = env::args().collect();

    let next = match args[1].as_ref() {
        "prev" if focused_index == 0 => windows.iter().last().unwrap(),
        "prev" => windows.iter().nth(focused_index - 1).unwrap(),
        "next" if focused_index == windows.len() => windows[0],
        "next" => windows.iter().nth(focused_index + 1).unwrap(),
        _ => panic!("Must provide either next or prev as an argument"),
    };

    // Send Message
    Command::new("i3-msg")
        .arg(format!("[con_id={}] focus", next.id))
        .output();
}

fn flatten_children(ws: &Node) -> Vec<&Node> {
    let mut children = vec![];
    let mut queue: VecDeque<&Node> = VecDeque::new();
    queue.push_back(ws);
    loop {
        let this_node = queue.pop_back().unwrap();
        for node in this_node.nodes.iter() {
            match node {
                Node {
                    window: Some(_), ..
                } => children.push(node),
                _ => queue.push_back(node),
            }
        }
        if queue.is_empty() {
            break;
        }
    }
    children
}

#[cfg(test)]
mod test_flatten {
    use super::*;
    use i3_alt_tab::i3::Rect;
    fn setup() {
        let child_a = Node {
            name: Some(String::from("child_a")),
            id: 123,
            type_con: Type::Con,
            rect: Rect {
                x: 0,
                y: 0,
                height: 0,
                width: 0,
            },
            focused: true,
            window: Some(123),
            nodes: vec![],
        };
        let child_b = Node { ..child_a.clone() };
        let child_c = Node {
            nodes: vec![child_a.clone(), child_b],
            ..child_a.clone()
        };
        let node_with_children = Node { ..child_a.clone() };
        return ();
    }
    #[test]
    fn flattens() {}
}

fn queued_ws(root: Node) -> Node {
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
            Node { focused: true, .. } => {
                break;
            }
            Node {
                type_con: Type::Workspace,
                ..
            } => ws = this_node,
            _ => (),
        }
    }
    ws
}

// Maybe try it with references
fn queued_ws2(root: &Node) -> Option<&Node> {
    // I am bad at the code
    let mut queue: VecDeque<&Node> = VecDeque::new();
    let mut ws: Option<&Node> = None;
    queue.push_back(root);
    loop {
        let this_node = queue.pop_back().unwrap();
        for node in this_node.nodes.iter() {
            queue.push_back(node);
        }
        match this_node {
            Node { focused: true, .. } => {
                break;
            }
            Node {
                type_con: Type::Workspace,
                ..
            } => ws = Some(this_node),
            _ => (),
        }
    }
    ws
}

fn recursive_ws<'a>(root: &'a Node, parent: Option<&'a Node>) -> Option<&'a Node> {
    let focused_child = root.nodes.iter().find(|n| n.focused);

    match focused_child {
        // If we find a focused child, bubble up this parent
        Some(_) => parent,
        // If none of the nodes children are focused then dig deeper
        None => {
            let children: Vec<Option<&Node>> = root.nodes
                .iter()
                .map(|r| recursive_ws(r, Some(root)))
                .collect();
            let focus = children.iter().find(|&n| match n {
                Some(_) => true,
                None => false,
            });
            println!(
                "{:?}",
                match parent {
                    Some(x) => Some(&x.name),
                    None => None,
                }
            );
            match focus {
                Some(_) => parent,
                None => None,
            }
        }
    }
}
