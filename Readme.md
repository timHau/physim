# Physim

A toy *phy*sics *sim*ulation written in Rust, heavily inspired by [the awesome johnBuffer](https://www.youtube.com/watch?v=lS_qeBy3aQI).



https://user-images.githubusercontent.com/12029285/218090302-91d7dc29-3922-4a9f-8ea4-d0913ce97485.mp4



## Requirements
To run this simulation you need to use [nightly rust](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html). To install use the following instructions
```
rustup default nightly
rustup update
```

We need nightly rust, because we make use of the `get_many_mut` feature, which is currently not yet in stable rust.

## Running
To run the simulation you should use the release version
```
cargo run --release
```

### Note
This repo currently uses a very naive way to handle collisions. I hope to improve upon this.
