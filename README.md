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

### Notes: Structs
1. Structs, Tuple Structs
2. Can have methods and associated functions that aren't methods (kinda like constructors) in impl


### Notes: Enums and Patterrn Matching
- A way of defining custom data types in a different way than you do with structs
- Option enum -> allows null & non-null type whereas other types 
- `match` -> contrrol flow construct that allows you to compare a value against a series of patterns and then execute code based on which pattern matches
- `other` -> like default in switch statemetn
- `_` -> use when we don't want to use the catch all 
1. `if let` -> less verbose way to handle values that match one pattern while ignoring the restk
```
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
      fn call(&self) {
          // method body would be defined here
      }
  }

  let m = Message::Write(String::from("hello"));
  m.call();
```

### Notes: Collections
- vector, string, hash map
- Stored on the heap
1. vectors -> `Vec<T>`
- store more than one value in a single data structure with values next to each other in memory
- `let v: Vec<i32> = Vec::new();`
- `let v = vec![1,2,3];`
- dynamic vectors need to be mutable
- vectors are freed when it goes out of scope
- reading elements of vectors
  - ```
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    ```
- `get()` returns None if out-of-bound, `&v[100]` will crash if out-of-bound

2. Strings
- Rust has one string type in the coer language which is the strign slice `str` (borowed `&str`)
- `String` type is provided by Rust's standard library rather than coded into the core language
  - it's a growable, mutable, owned, UTF-8 encoded string type
  - string concatenation -> `s.push_str("hello")` -> takes a string slice so no ownership of the parameter
  ```
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    let s = format!("{}-{}-{}", s1, s2, s3);
  ```
  - indexing -> indexing into a string is often a bad idea because it's not clear what the return type of the string-indexing operation should be: a byte value, character, grapheme cluster, or str slice

3. Hash Maps
- `let mut scores = HashMap::new();`
```
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
```
- overwriting a value
  ```
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
  ```
- Only inserting a value if the key has no value
  ```
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
  ```
- Updating a value based on the old value
  ```
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
  ```






### Terminologies
- Crates: packages of code
- Associated function: a function that's implemented on aa type(i.e. `new` in `String::new()`)
- Double free error: two variables pointing to the same data on the heap. If one of these variables go out-of-scope, the heap data will be drop
  - Note: I believe this opens in other languages, but not Rust
