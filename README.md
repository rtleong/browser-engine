Rusty Renderer
========

A Rust web rendering engine by Riley Leong
(rileyleong24@g.ucla.edu).

Currently implemented:

* Parse a small subset of HTML and build a DOM tree.
* Parse a small subset of CSS.
* Perform selector matching to apply styles to elements.
* Basic block layout.

Instructions
------------

1. [Install Rust 1.0 beta or newer.](http://www.rust-lang.org/install.html)

2. Clone the Rusty Renderer source code from https://github.com/rtleong/browser-engine

3. Run `cargo build` to build Rusty Renderer, and `cargo run` to run it.

To build and run with optimizations enabled, use `cargo build --release` and
`cargo run --release`.

By default, Rusty Renderer will load test.html and test.css from the `examples`
directory.  You can use the `--html` and `--css` arguments to the cearch
executable to change the input files:

    ./target/debug/browser-engine --html examples/test.html --css examples/test.css

The rendered page will be saved to a file named `output.png`.  To change the
output filename, use the `-o` option.  To switch to PDF output, use add
`--format pdf`.
