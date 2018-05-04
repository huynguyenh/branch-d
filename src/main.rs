extern crate clap;

use clap::App;

fn main() {
    let _matches = App::new("bd")
        .version("0.1")
        .author("huynguyen <hoanghuy2908@gmail.com>")
        .about("Do some stupid stuff noone care to do")
        .get_matches();

    println!("hello world");
}
