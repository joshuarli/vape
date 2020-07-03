extern crate getopts;

use getopts::Options;
use std::io::{self, Read};
use std::{char, env, process};

const VERSION: &str = "0.3.1";

fn print_usage(program: &str, opts: &Options) {
    let usage = format!("Usage: {} [OPTIONS]", program);
    println!("{}", opts.usage(&usage));
}

fn to_fw(c: char) -> Option<char> {
    let c = c as u32;
    match c {
        0x0020 => Some(char::from_u32(0x3000).unwrap()),
        0x0021..=0x007e => Some(char::from_u32(c + 0xfee0).unwrap()),
        _ => None,
    }
}

fn rand_kana() -> u32 {
    // Highest kana is 0x30FF. 0x30FF - 0x30A0 (lowest kana) = 95.
    let ret = 0x30A0 + (fastrand::u32(..) % 95);
    ret
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

    let matches = opts.parse(&args[1..]).unwrap_or_else(|e| {
        eprintln!("{}\nFor usage, try `{} -h`", e, &args[0]);
        process::exit(1);
    });

    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
        return;
    }

    if matches.opt_present("v") {
        println!("{}", &VERSION);
        return;
    }

    let kata_opt = matches.opt_str("k").unwrap_or_else(|| "0".to_string());
    let mut num_kata: u8 = kata_opt.parse::<u8>().unwrap_or_else(|_| {
        eprintln!("Option -k, --kana must be an integer from 0 to 255.");
        process::exit(1);
    });

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });

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
        while num_kata > 0 {
            output.push(char::from_u32(rand_kana()).unwrap());
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
