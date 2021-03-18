An example of using addcomb_comp for fun and profit.

To get working, download rust on your computer and restart your terminal (to check it's correct, run `which cargo`). Then, download this repository into a directory (`git clone https://github.com/Torrencem/addcomb_kyle.git`) and `cd` into it.

IMPORTANT: Install rust nightly by typing `rustup default nightly`!

Helpful commands:

To check your code for errors:
```
cargo check
```

To run with optimizations:
```
cargo run --release
```

Unfortunately, `addcomb-comp` is poorly documented right now, but if you want to check through the generated docs for different method signatures, use:
```
cargo doc --open
```
and use the side panel to open `addcomb-comp`, and click around to find definitions.
