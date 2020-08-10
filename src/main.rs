extern crate csv;
extern crate getopts;

use getopts::Options;
use std::env;

fn parse_args() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("l", "list", "list header");
    opts.optopt("d", "delimiter", "specify the delimiter", "DELIM");
    opts.optopt("n", "top", "select top N record", "TOPN");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("error: {}", f);
            print_usage(&program);
            std::process::exit(1);
        }
    };

    if matches.free.is_empty() {
        print_usage(&program);
        std::process::exit(1);
    }

    let delim = matches.opt_str("d");
    let top_n = matches.opt_str("n");

    if matches.opt_present("l") {
        csv::list_headers(&matches.free[0], delim);
    } else {
        csv::list_columns(&matches.free[0], &matches.free[1..], delim, top_n);
    }
}

fn print_usage(program: &str) {
    println!("\nusage: {} [-l] <file> [column1] [column2] [...]", program);
}

fn main() {
    parse_args();
}
