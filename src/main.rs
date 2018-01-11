extern crate getopts;

use std::env;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let usage = format!("Usage: {} [OPTIONS]", program);
    print!("{}", opts.usage(&usage));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optopt("k", "kata", "append N random katakana characters, up to 255", "N");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let kata_opt = matches.opt_str("k");
    let num_kata: u8 = match kata_opt {
        Some(x) => { x.parse().unwrap() }
        None => { 0 }
    };

    println!("{}", num_kata);

    return;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
