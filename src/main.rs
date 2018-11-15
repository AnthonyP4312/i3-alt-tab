use std::collections::VecDeque;
use std::process::Command;

extern crate i3_alt_tab;
extern crate serde_json;
extern crate structopt;

use i3_alt_tab::i3::{Args, Direction, Node, Type, WSOptions, Workspace};
use serde_json::from_slice;
use structopt::StructOpt;
use Direction::*;

fn main() {
    // Pull our node tree from i3
    let args = Args::from_args();

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
    let workspaces: Vec<i32> = select_workspaces(&args, workspaces);
    let workspaces: Vec<&Node> = find_wss(workspaces, &nodes);

    // Flatten together all the windows under these workspaces
    let mut windows: Vec<&Node> = workspaces
        .into_iter()
        .map(flatten_children)
        .map(|mut w| {
            w.sort_by(|a, b| a.rect.x.cmp(&b.rect.x));
            w.sort_by(|a, b| a.rect.y.cmp(&b.rect.y));
            w
        })
        .flatten()
        .collect();

    // Sort the windows Highest to Lowest, then Left to Right
    // windows.sort_by(|a, b| a.rect.x.cmp(&b.rect.x));
    // windows.sort_by(|a, b| a.rect.y.cmp(&b.rect.y));

    // Find the next in line (wrapping to front if last window)
    let focused_index = windows.iter().position(|n| n.focused).unwrap();
    let next = match args.direction {
        Left if focused_index == 0 => windows.iter().last().unwrap(),
        Left => windows.iter().nth(focused_index - 1).unwrap(),
        Right if focused_index + 1 == windows.len() => windows[0],
        Right => windows.iter().nth(focused_index + 1).unwrap(),
    };

    // Send Message
    Command::new("i3-msg")
        .arg(format!("[con_id={}] focus", next.id))
        .output()
        .unwrap();
}

/// Based on the choice for workspaces filter down the workspaces
/// to a list of numbers
fn select_workspaces(args: &Args, workspaces: Vec<Workspace>) -> Vec<i32> {
    let ws = workspaces.into_iter();
    let result: Vec<Workspace> = match args.workspaces {
        WSOptions::All => ws.collect(),
        WSOptions::Focused => ws.filter(|w| w.focused).collect(),
        WSOptions::Visible => ws.filter(|w| w.visible).collect(),
    };

    result.into_iter().map(|w| w.num).collect()
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
    // Find some property based testing shit
}

fn find_wss(targets: Vec<i32>, root: &Node) -> Vec<&Node> {
    let mut wss: Vec<&Node> = vec![];
    let mut queue: VecDeque<&Node> = VecDeque::new();
    queue.push_back(root);
    loop {
        if queue.is_empty() {
            break;
        }

        let this_node = queue.pop_back().unwrap();
        match this_node {
            Node {
                num: Some(ws_num), ..
            } => {
                if targets.contains(&ws_num) {
                    wss.push(this_node)
                }
            }
            _ => {
                for node in this_node.nodes.iter() {
                    queue.push_back(node);
                }
            }
        }
    }
    wss
}
