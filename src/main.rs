extern crate clap;

use clap::App;

use std::process::Command;

fn main() {
    let _matches = App::new("bd")
        .version("0.1")
        .author("huynguyen <hoanghuy2908@gmail.com>")
        .about("Do some stupid stuff noone care to do")
        .get_matches();

    let repo_name = get_git_hub_repo();

    if repo_name != "" {
        println!("{}", repo_name);
    }
}

fn get_git_hub_repo() -> String {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let output: String = String::from(Strin g::from_utf8_lossy(&output.stdout));
    if output.contains("github.com") {
        let mut v: Vec<&str> = output.split("/").collect();
        v = v[v.len() - 1].split(".").collect();
        return String::from(v[0]);
    }

    let s = String::new();
    s
}
