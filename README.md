This is a pure rust library for parsing osu! .osr replay files. It is designed to be used anywhere without the need of any system dependencies.

## Usage
```rust
use std::fs;
use osu_replay_parser::Replay;

let file = fs::read("assets/replay.osr").unwrap();
let replay = Replay::parse(&file).unwrap();
println!("{}", replay);
```