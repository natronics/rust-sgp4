Simplified Perturbations Models
===============================

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
[![Build Status](https://travis-ci.org/natronics/rust-sgp4.svg?branch=master)](https://travis-ci.org/natronics/rust-sgp4)


_Simplified perturbations models_ are a set of models used for satellites and objects relative to an Earth-centered inertial coordinate system. These are often referred to collectively as **SGP4** because of how popular that particular code is and how it's used with nearly all low Earth orbit satellites.

The SGP4 and SDP4 models were published as FORTRAN IV in 1988. It has also been ported to C. This is a port to [Rust][rustlang].


Documentation
-------------

[Module documentation](https://natronics.github.io/rust-sgp4/)


Usage
-----

This port of SGP4 is still in progress.

A minimal example:

```rust
extern crate sgp4;

fn main() {

    let tle = sgp4::tle::load_from_str(
        "ISS (ZARYA)",
        "1 25544U 98067A   16210.59822142  .00000812  00000-0  11901-4 0  9990",
        "2 25544  51.6406 211.4156 0001780  85.8307 274.3426 15.54888439 11433",
    );
    println!("Satellite {}:", tle.int_designator);

    let time = 0.0;
    println!("TEME position at time t={}:", time);

    let location = sgp4::propagate(tle, time);
    println!("    X: {}", location.X);
    println!("    Y: {}", location.Y);
    println!("    Z: {}", location.Z);
}
```


Developing
----------

See [Rust's directions for getting Rust installed on your computer](https://www.rust-lang.org/en-US/downloads.html). If you're using OSX or any flavor of linux this should probably work:

    $ curl -sSf https://static.rust-lang.org/rustup.sh | sh

If you have the Rust toolchain installed, compile the program with cargo:

    $ cargo build

Be sure to test after making changes

    $ cargo test

Build the module documentation locally:

    $ make doc


License
-------

Copyright (c) 2016 Nathan Bergey

This project is licensed under the terms of the MIT license


[rustlang]: https://www.rust-lang.org/
