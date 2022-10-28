#![allow(unused)]

use crate::util::{LegacyRandomNumberGenerator, RandomNumberGenerator, ModernRandomNumberGenerator};

mod config;
mod util;
mod grid;
mod graphics;



fn main() {
    let config = config::Config::load();
    
}
