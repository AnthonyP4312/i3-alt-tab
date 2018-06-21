use std::process::Command;
use std::str::from_utf8;

extern crate serde_json;

use serde_json::{from_slice};


fn main() {
    let output = Command::new("i3-msg")
        .arg("-t")
        .arg("get_tree")
        .output()
        .unwrap();

    let output_str = from_utf8(&output.stdout).unwrap();
    println!("{:?}", output_str);

}
