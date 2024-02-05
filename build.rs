extern crate chrono;

use chrono::prelude::*;

fn set_build_date() {
    let date = Local::now().format("%B %Y");
    println!("cargo:rustc-env=BUILD_DATE={}", date);
}

fn main() {
    set_build_date();
}
