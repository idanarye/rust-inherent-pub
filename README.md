[![Build Status](https://api.travis-ci.org/idanarye/rust-inherent-pub.svg?branch=master)](https://travis-ci.org/idanarye/rust-inherent-pub)
[![Latest Version](https://img.shields.io/crates/v/inherent-pub.svg)](https://crates.io/crates/inherent-pub)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://idanarye.github.io/rust-inherent-pub/)

# Rust Inherent Pub

## Motivation

If you `impl` a trait on a type, you'll only be able to use the methods in that
`impl` block if you import the trait:

```rust
mod geometry {
    pub trait Length {
        fn length(&self) -> f64;
    }

    pub struct Vector(pub f64, pub f64);

    impl Length for Vector {
        fn length(&self) -> f64 {
            let Vector(x, y) = self;
            (x.powi(2) + y.powi(2)).sqrt()
        }
    }
}

fn main() {
	// Compilation error: we did not `use geometry::Length` so we can't access `length()`
    assert!(geometry::Vector(3.0, 4.0).length() == 5.0);
}
```

With `#[inherent_pub]`, we can make it so `length()` can be used without
importing the `Length` trait - just annotated the `impl` block and mark the
method as `pub`:

```rust
extern crate inherent_pub;

mod geometry {
    use inherent_pub::inherent_pub;

    pub trait Length {
        fn length(&self) -> f64;
    }

    pub struct Vector(pub f64, pub f64);

    #[inherent_pub]
    impl Length for Vector {
        pub fn length(&self) -> f64 {
            let Vector(x, y) = self;
            (x.powi(2) + y.powi(2)).sqrt()
        }
    }
}

fn main() {
    assert!(geometry::Vector(3.0, 4.0).length() == 5.0);
}
```
