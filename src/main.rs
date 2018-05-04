extern crate clap;

use clap::{App, Arg};

use std::process::Command;

fn main() {
    const DEFAULT_BRANCH: &str = "develop";
    let matches = App::new("branch-d")
        .version("0.2")
        .author("huynguyen <hoanghuy2908@gmail.com>")
        .about("Do some stupid stuff noone care to do")
        .arg(
            Arg::with_name("branch")
                .short("b")
                .help("Delete branches which have been merged into input branch")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fetch")
                .short("f")
                .help("fetch all branches before delete")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let br: String = String::from(matches.value_of("branch").unwrap_or(DEFAULT_BRANCH));
    let fetch: bool = matches.is_present("fetch");

    let list_of_branches = get_list_of_branches(&br, fetch);

    if list_of_branches.len() > 1 {
        for i in list_of_branches {
            if i.len() > 0 {
                if i != "master" || i != "develop" {
                    delete_branch(&i)
                }
            }
        }
    }
}

fn delete_branch(name: &String) {
    println!("start delete local branch {}", name);
    let mut output = Command::new("git")
        .args(&["branch", "-d", name.trim()])
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    println!("{}", String::from(String::from_utf8_lossy(&output.stdout)));

    println!("start delete remote branch {}", name);
    output = Command::new("git")
        .args(&["push", "origin", "--delete", name.trim()])
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    println!("{}", String::from(String::from_utf8_lossy(&output.stdout)));
}

fn get_list_of_branches(br_name: &String, is_fetch: bool) -> Vec<String> {
    if is_fetch {
        println!("fetching remote branches");
        let _ = Command::new("git")
            .args(&["fetch", "origin"])
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    }
    let output = Command::new("git")
        .args(&["branch", "--merged", br_name.trim()])
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
