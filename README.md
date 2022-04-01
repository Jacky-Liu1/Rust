# Rust
- https://doc.rust-lang.org/book/title-page.html

## Notes: Rust Tool System/Attributes
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
- Updating a Crate -> cargo update
- Build documentation -> cargo doc --open

## Notes: Rust Language/Syntax
1. Immutable/mutable
- Variables are immutable by default(can't reassign variable)
- To make a variable mutable, we add `mut` before the variable name

2. Overflow/underflow(panic in debug mode -> wrap around in release mode)

3. Out-of-bound array access -> not possible

4. Statements and Expressions
- Statement: instructions that perform some action and do not return a value

`code`
fn main() {
    let y = 6;
}
`code`

- Exressions evaluate to a resulting value (NOTE: NO SEMILCOLON AT THE END!!!)
`code`
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
`code`




### Terminologies
- Crates: packages of code
- Associated function: a function that's implemented on aa type(i.e. `new` in `String::new()`)
