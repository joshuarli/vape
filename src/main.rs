extern crate getopts;
extern crate rand;

use std::{char, env, process};
use std::io::{self, Read};
use rand::{FromEntropy, Rng};
use rand::rngs::SmallRng;
use getopts::Options;

const VERSION: &str = "0.1.3";
const KANA_LO: u32 = 0x30A0;
const KANA_HI: u32 = 0x30FF;

fn print_usage(program: &str, opts: &Options) {
    let usage = format!("Usage: {} [OPTIONS]", program);
    println!("{}", opts.usage(&usage));
}

fn to_fw(c: char) -> Option<char> {
    let c = c as u32;
    match c {
        0x0020 => Some(char::from_u32(0x3000).unwrap()),
        0x0021...0x007e => Some(char::from_u32(c + 0xfee0).unwrap()),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version");
    opts.optopt(
        "k",
        "kana",
        "append N random katakana characters, up to 255",
        "N",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}\nFor usage, try `{} -h`", e, &args[0]);
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
        return;
    }

    if matches.opt_present("v") {
        println!("{} {}", &args[0], &VERSION);
        return;
    }

    let kata_opt = matches.opt_str("k");
    let mut num_kata: u8 = match kata_opt {
        Some(x) => {
            let k = match x.parse::<u8>() {
                Ok(p) => p,
                Err(_) => {
                    eprintln!("Option -k, --kana must be an integer from 0 to 255.");
                    process::exit(1);
                }
            };
            k
        }
        None => 0,
    };

    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let mut output: String = input.chars().map(|c| to_fw(c).unwrap_or(c)).collect();

    if num_kata > 0 {
        // if a trailing newline exists (e.g. echo stdout is piped to vape)
        // then we want to insert the kana before it.
        let reserve_trailing_newline = output.ends_with('\n');
        if reserve_trailing_newline {
            output.pop();
        }
        // in any case, add a fw space to make appended kana look better
        output.push(char::from_u32(0x3000).unwrap());
        let mut rng = SmallRng::from_entropy();
        while num_kata > 0 {
            output.push(char::from_u32(rng.gen_range(KANA_LO, KANA_HI + 1)).unwrap());
            num_kata -= 1;
        }
        if reserve_trailing_newline {
            output.push('\n');
        }
    }

    print!("{}", output);
}

#[cfg(test)]
mod tests {
    use to_fw;
    #[test]
    fn test_supported_fw() {
        let orig = " 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    !\"#$%&()*+,-./:;<=>?@[\\]^_`{|}~";
        let fw = "　０１２３４５６７８９ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ\
            ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ\
            ！＂＃＄％＆（）＊＋，－．／：；＜＝＞？＠［＼］＾＿｀｛｜｝～";
        let orig_fw: String = orig.chars().map(|c| to_fw(c).unwrap_or(c)).collect();
        assert_eq!(orig_fw, fw);
    }
    #[test]
    fn test_no_fw() {
        let orig = "😍😍😍🙏🙏🙏🍆🍆🍆";
        let fw = "😍😍😍🙏🙏🙏🍆🍆🍆";
        let orig_fw: String = orig.chars().map(|c| to_fw(c).unwrap_or(c)).collect();
        assert_eq!(orig_fw, fw);
    }
}
