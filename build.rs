extern crate lalrpop;
use std::env;

fn main() {
    env::set_var("IN_DIR", "./src/assembler");
    env::set_var("OUT_DIR", "./src/assembler");
    lalrpop::Configuration::new().use_cargo_dir_conventions().process().unwrap();

    // lalrpop::process_root().unwrap();
}
