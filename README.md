# MIDIFUC

MIDI For Use in Coding (MIDIFUC) is a MIDI-based programming language inspired by [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) that emphasizes musicality over functionality.

## Running the interpreter

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
  - The default installer should also install the `cargo` crate manager.

### Instructions

In the root folder of the repository, the project can be built and run using `cargo`.

```bash
cargo run -- [LOCATION] [TRACK NUMBER]
```

Alternatively, a binary can be built with

```bash
cargo build --release
```

and can be executed like so:

```bash
# Linux
./target/release/midifuc [LOCATION] [TRACK NUMBER]

# Windows
./target/release/midifuc.exe [LOCATION] [TRACK NUMBER]
```

### Running test MIDI files

In the root folder of the repository, each test can be run with their respective commands:

Hello World:

```bash
cargo run -- ./tests/helloworld.mid 1
```

Read/write 1 character from stdin to stdout:

```bash
cargo run -- ./tests/readwrite.mid 0
```

`cat` equivalent:

```bash
cargo run -- ./tests/cat.mid 0
```
