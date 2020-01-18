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

But what if the method is a natural member of the type itself, not just the
trait? Shouldn't we be able to just access the `length()` of any `Vector`
without having to explicitly tell Rust that we are using `Length`?

Enter the `#[inherent_pub]` procedural macro attribute!

## Usage

With `#[inherent_pub]`, we can make it so `length()` can be used without
importing the `Length` trait - just annotated the `impl` block and mark the
method as `pub`:

```rust
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

## How the desugaring works?

`#[inherent_pub]` removes the `pub` from the methods and adds another `impl`
block with inherent methods that get redirected to the trait ones. So:

```rust
impl Foo for Bar {
    #[inherent_pub]
    pub fn foo(self) {
        // Some code
    }
}
```

Gets desugared to:

```rust
impl Foo for Bar {
    fn foo(self) {
        // Some code
    }
}

impl Bar {
    #[doc(hidden)]
    #[inline(always)]
    pub fn foo(self) {
        <Self as Foo>::(self)
    }
}
```

Rather than handling special arguments like ignored arguments that start with
(or are) a single underscore `_` or pattern arguments, `#[inherent_pub]` simply
replaces all arguments (other than `self`) with generic names. So:

```rust
impl Foo for Bar {
    #[inherent_pub]
    pub fn foo(self, a: i32, (b, c): (i32, i32), _: i32) {
        // Some code
    }
}
```

Gets desugared to:

```rust
impl Foo for Bar {
    fn foo(self, a: i32, (b, c): (i32, i32), _: i32) {
        // Some code
    }
}

impl Bar {
    #[doc(hidden)]
    #[inline(always)]
    pub fn foo(self, arg1: i32, arg2: i32, arg3: i32) {
        <Self as Foo>::(self, arg1, arg2, arg3)
    }
}
```
