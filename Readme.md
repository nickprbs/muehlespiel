# Run

## AI

To run an ai, just run:
```bash
cargo run
```
or run it with the provided UI.

To get the full speed (it's very slow in debug mode), run:
```bash
cargo build --release
```
and then point the UI to the release binary (in `./target/release/muehlespiel` instead of `./target/debug/muehlespiel`)

## Other modes

To run enumeration (Übung 4), run:
```bash
cargo run -- enumerate
```

To run a player vs player game (Übung 3), run:
```bash
cargo run -- player-vs-player
```

To print a game board from a string, run:
```bash
cargo run -- print-board WWEWBEBWBEWEBEBEWWBEWBEB
```

To debug an ai output, run:
```bash
carog run -- ai-debug "M B WWWEBEBWEWEEBEBBWWBEWBBE"
```
This will print, what the AI has determined as possible moves and which move it'd like to take.