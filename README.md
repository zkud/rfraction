# rfraction
[![CI](https://github.com/zkud/rfraction/actions/workflows/ci.yaml/badge.svg)](https://github.com/zkud/rfraction/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/zkud/rfraction/branch/main/graph/badge.svg?token=YJ9UE1UFBJ)](https://codecov.io/gh/zkud/rfraction)
[![Hits-of-Code](https://hitsofcode.com/github/zkud/rfraction?branch=main)](https://hitsofcode.com/github/zkud/rfraction/view?branch=main)

Rapid and clear to use fractional numbers crate for the Rust programming language

## Installing

Add this to your Cargo.toml:

```
[dependencies]
rfraction = "0.1.0"
```

## Usage Example:

Here is an example program of a simple calculation using this crate:
```
use rfraction::{Fraction, Sign};

fn main() {
  let a = Fraction::<u128>::new(Sign::Positive, 10, 4);
  let b = Fraction::<u128>::new(Sign::Negative, 7, 2);
  let c = Fraction::<u128>::new_natural(10);
  
  println!("{} + {}*{} = {}", a, b, c, a + b*c);
}
```

## License

Rfraction is distributed under the terms the MIT license, please check the [LICENSE](./LICENSE) for details.
