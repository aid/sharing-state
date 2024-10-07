# Shared State

This repository includes some test code based upon the `Arc<RwLock>` example in the following Medium article by Sai Praveen Polimera:

* [Sharing State in Rust: Exploring Different Approaches](https://medium.com/@contactomyna/sharing-state-in-rust-exploring-different-approaches-73f30f969bff)

The code as presented in the article did not compile - indeed it seemed to be a mash-up between threaded and async code.

In this repository, we include two source files, both based upon the original code with as few changes as necessary applied for it to operate properly:

* `src/main-async.rs` - Operating under the tokio async runtime.
* `src/main-thread.rs` - Operating using the operating system's standard threads through std::thread.

The `Cargo.toml` file is configured to produce two binaries:

* `sharing-state-async`
* `sharing-state-thread`

The standard `cargo build` command will build both binaries.

We can use `cargo` to run each of these binaries as follows:

* `cargo run -bin sharing-state-async`
* `cargo run -bin sharing-state-thread`


## Dependencies:

The following crates were added to support this code:

```
cargo add tokio@="1.40" --features rt-multi-thread,net,macros,time,sync
cargo add rand@="0.8.2"
```
