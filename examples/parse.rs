use osu_replay_parser::parse_replay;
use std::fs;

fn main() {
    // This is a sample replay file
    let input = fs::read("assets/replay.osr").expect("Error reading file");
    let replay = parse_replay(input).unwrap();

    println!("{}", replay);
}
