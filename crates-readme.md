# urng_core

A minimal, `no_std` random number generator core where the internal state is expressed as a type.

## Usage

```rust
use urng_core::Rng;

struct XorShift32 {
    state: u32,
}

impl Rng for XorShift32 {
    type Word = u32;

    fn nextu(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
}

fn main() {
    let mut rng = XorShift32 { state: 2463534242 };

    let _: u32 = rng.nextu();             // [0, 2^32-1]
    let _: f32 = rng.nextf();             // [0, 1)
    let _: i32 = rng.randi(-10, 10);      // [-10, 10]
    let _: f32 = rng.randf(-1.0, 1.0);    // [-10.0, 10.0)
}
```
