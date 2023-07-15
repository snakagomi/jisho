use std::io::{BufRead, BufReader};
use std::{env, fs::File};

fn main() {
    let dic_file = "./ejdic-hand-txt/ejdict-hand-utf8.txt";
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("[Usage] jisho word");
        return;
    }

    let word = &args[1];

    let fp = File::open(dic_file).unwrap();
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.find(word) == None {
            continue;
        }
        println!("{}", line);
    }
}
