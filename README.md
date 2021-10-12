# alpha_engine

- Platform tested: Manjaro (Linux).
- Language: Rust (https://www.rust-lang.org/).
- For these instructions cargo is also needed (https://doc.rust-lang.org/cargo/getting-started/installation.html).

  In the future the engine will be uploaded as a crate to be downloaded in new projects. For now, the library has to be downloaded manually from here and it's directory added to the cargo.toml of the new project.

## Compilation instructions

- Clone this repository.
- Add the directory of the engine to the cargo.toml file of the new project. (Not necessary when compiling examples inside this repository as the directory has already been specified).
- As the engine is a Rust library, it doesn't produce a binary and thus can't be ran by itself. We need to compile the project that's using the engine.
- For that we run the command `cargo build` if we only want to compile the project or `cargo run` if we want to also run it.

## Add engine to new project

To create a new project with the engine, we must create a new rust project with `cargo new` (new folder) or `cargo init` (current folder) and specify the directory in which the engine resides.
For that we edit the cargo.toml file of the project and add `alpha_engine = { path = "path/to/engine/folder" }` under `[dependencies]`
After that we need to add `extern crate alpha_engine;` to the main.rs file of the new project and we can start using the engine.
