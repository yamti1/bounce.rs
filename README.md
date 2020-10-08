# Bouncers
Bouncing balls game written in Rust with the ggez library.

## Build & Run
Install Rust, then clone the repository and run `cargo run`.
It should download the dependecies, compile and run the game.
For better performance compile with `cargo build --release` and then run the resulting executable under `target/release`.

## Gamplay
Click the screen to add more Bouncers and have fun!

## Configuration
The file `src/conf.rs` contains const values that allow you to control the game behavior.
You can play around with those and see their effect (just remember to re-compile and re-run every time you change a value).
