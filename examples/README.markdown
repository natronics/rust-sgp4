# Rust SGP4 Example

Basic example:

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

## Running This Example Code

The above code is in `src/main.rs`. Run it using cargo:

    $ cargo run
    Satellite 98067A  :
    TEME position at time t=0:
        X: 0
        Y: 0
        Z: 0
    
