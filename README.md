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

```
fn main() {
    let y = 6;
}
```

- Exressions evaluate to a resulting value (NOTE: NO SEMILCOLON AT THE END!!!)
```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

### Notes: Rust Ownership
1. Ownership
  - Enables Rust to make memory safety guarantees without needing a garbage collector
  - Ownership: set of rules that governs how a Rust program manages memory
  - Other languages use a garbage collector that scans for unused memory or make programmers manage their own memory
  - In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks
  - Ownership rules:
    - Each value in rust has aa variable that's called its called
    - There can only be one owner at a time
    - When the owner goes out of scope, the value will be dropped 

2. References and Borrowing
  - Since variables are dropped if it goes out-of-scope when passed into a function, we can't use the variables again unless if we return it as a tuple in the function
  - We use a reference instead
  - Reference: is like pointer in that it's an address we can follow to access data stored at that address that is owned by some other variable
    - Unlike a pointer, reference is guaranteed to point to a valid value of a particular type
  - Borrrowing: action of creating a reference  
    - Note: can't modify when borrowing since references are immutable
    - Mutable references -> add `&mut`
      - Can only have one mutable reference to a paarticular piece of data at a time (to prevent data races at compile time)
        - Data race:
          1. Two or more pointers access the same data at the same time
          2. At least one of the pointers is being used to write to the data
          3. There's no mechanism being used to synchronize access to the data (concurrency)
  - Cannot have a mutable reference while having an immutable one to the same value
  - Dangling pointer: a pointer that references a location in memory that may have been given to someone else - by freeing some memory while preserving a pointer to that memory
    - Rust guarantees that references will never be dangling references

3. The Slice Type
 - Lets you reference a contiguous sequence of elements in a collection rather than the whole collection
 - A kind of reference, so it does not have ownership 



### Terminologies
- Crates: packages of code
- Associated function: a function that's implemented on aa type(i.e. `new` in `String::new()`)
- Double free error: two variables pointing to the same data on the heap. If one of these variables go out-of-scope, the heap data will be drop
  - Note: I believe this opens in other languages, but not Rust
