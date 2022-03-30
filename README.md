# Rust
- https://doc.rust-lang.org/book/title-page.html

## Notes
1. Rust compiler plays a gatekeeper role by refusing to compile code with elusive bugs(i.e. concurrency bugs)
- For example, low-level code is prrone to a variety of subtle bugs, which in most other languages can be caught only through extensive testing and careful code review by experienced developers

2. Tool system
- Cargo -> build system/dependency manager
- Rustfmt -> consistent code sytling
- Rust Language Server -> code completion and inline error msg

3. Use cases
- Systems programming, CLI, web services, DevOps tooling, embedded devices, audio and video analysis aand transcoding, cryptocurrencies, bioinformatics, search engines, IoT, machine learning

4. Compiling
- rustc filename.rs
- ./filename

5. Ahead-of-time compiled language
- You can compile a Rust program and give the executable to someone else, and they can run it without having Rust installed
- If you give someone a .py or .js file, they need too have Python/JavaScript installed

6. Cargo
- Creating a new Cargo project -> cargo new project_name
- Building a Cargo project -> cargo build  (this creates a executabale file in target/debug/)
- Compiling and running executabale -> cargo run
- Cargo check: checks your code to make sure it compiles but doesn't produce an executable
- Building for release -> cargo build --release  (compiles with optimization in target/release/)




### Terminologies
- Crates: packages of code