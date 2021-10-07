This crate is a WIP.

## Usage

Create a new binary crate and reference this crate wherever it is:

```toml
[package]
name = "test-file-copycat"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
file-copycat = { path = "relative/path/to/the/crate" }
```

Create two files, one is the input file you want to watch changes for
(`./testfile.txt`). The other is the output file you want to write on
(`./outputfile.txt`).

Then paste the following code into the `main.rs` file

```rust
use file_copycat::watch;
use std::str::from_utf8;

fn main() {
    let replacer = |input: Vec<u8>| {
        let mut utf8 = from_utf8(&input).unwrap();
        let replaced = utf8.replace("hello", "goodbye");

        replaced.as_bytes().to_vec()
    };

    if let Err(err) = watch("./testfile.txt", "./outputfile.txt", Box::new(replacer)) {
        println!("{:?}", err);
    }
}
```
