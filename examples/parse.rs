use osu_replay_parser::{Replay};
use std::fs;

fn main() {
    // This is a sample replay file
    let input = fs::read("assets/replay.osr").expect("Error reading file");
    let replay = Replay::parse(&input).expect("Error parsing replay");
    println!("{}", replay);
}
