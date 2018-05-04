extern crate clap;

use clap::App;

use std::process::Command;

fn main() {
    let _matches = App::new("bd")
        .version("0.1")
        .author("huynguyen <hoanghuy2908@gmail.com>")
        .about("Do some stupid stuff noone care to do")
        .get_matches();

    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        println!("{}", s);
    } else {
        println!("Not a github repository");
    }
}
