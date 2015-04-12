use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::exit;

static TEMPLATE: &'static str = "\
#![feature(exit_status)]
extern crate @UTIL_CRATE@ as uu@UTIL_CRATE@;

use std::env;
use uu@UTIL_CRATE@::uumain;

fn main() {
    env::set_exit_status(uumain(env::args().collect()));
}
";

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: mkbuild <crate> <outfile>");
        exit(1);
    }

    let crat    : &str = args[1].as_ref();
    let outfile : &str = args[2].as_ref();

    let main = TEMPLATE.replace("@UTIL_CRATE@", crat);
    let out = OpenOptions::new().create(true).read(true).write(true).truncate(true)
        .open(&Path::new(outfile));
    match out {
        Err(e) => panic!("{}", e),
        Ok(mut f) => match f.write_all(main.as_bytes()) {
            Err(e) => panic!("{}", e),
            Ok(_) => ()
        }
    }
}
