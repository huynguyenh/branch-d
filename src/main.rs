extern crate clap;

use clap::App;

use std::process::Command;

fn main() {
    let _matches = App::new("bd")
        .version("0.1")
        .author("huynguyen <hoanghuy2908@gmail.com>")
        .about("Do some stupid stuff noone care to do")
        .get_matches();

    let list_of_branches = get_list_of_branches();

    if list_of_branches.len() > 1 {
        for i in list_of_branches {
            if i.len() > 0 {
                delete_branch(&i)
            }
        }
    }
}

fn delete_branch(name: &String) {
    let output = Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(name.trim())
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    println!("{}", String::from(String::from_utf8_lossy(&output.stdout)));
}

fn get_list_of_branches() -> Vec<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--merged")
        .arg("develop")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let output: String = String::from(String::from_utf8_lossy(&output.stdout));

    let lines: Vec<&str> = output.split("\n").collect();

    let mut res: Vec<String> = Vec::new();

    for i in &lines {
        let s: String = String::from(*i);
        res.push(s);
    }

    res
}
