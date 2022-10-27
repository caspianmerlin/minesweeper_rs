#![allow(unused)]

mod config;
mod util;



fn main() {
    let config = config::Config::load();
    println!("{:#?}", config);
    config.save_to_ini().unwrap();
}
