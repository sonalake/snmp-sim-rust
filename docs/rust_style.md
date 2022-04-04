# Contributing Rust code to Sonalake's SNMP Simulator

We don't maintain a separate style guide but in general try to follow [common good practice](https://aturon.github.io/), write readable and idiomatic code and aim for full test coverage. In addition, this document lists a few decisions we've reached in discussions about specific topics.

## Rust version

We currently always use the latest Rust stable toolchain. Please install it using `rustup`:

https://www.rust-lang.org/tools/install

To update stable:

```
rustup update stable
```

## Rustfmt

Apply Rustfmt to new code before committing, using the default configuration or, if present, the repository's `rustfmt.toml` file. We run Rustfmt on the stable toolchain.

To install Rustfmt:

```
rustup component add rustfmt
```

To run Rustfmt:

```
cargo fmt
```

## Clippy

Crates are tested using cargo-clippy; make sure your code does not produce any new errors when running Clippy. If you don't agree with a [Clippy lint](https://github.com/Manishearth/rust-clippy#lints), discuss it with the team before explicitly adding a `#[allow(clippy::<lint>)]` attribute. We run Clippy on the stable toolchain.

To install Clippy:

```
rustup component add clippy
```

To run Clippy:

```
cargo clippy
cargo clippy --all-targets
```

If the crate being tested also defines features, these two Clippy commands should also be run with each feature enabled.

## Unwrap

Don't unwrap [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)s or [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)s, except possibly when:

1. locking a mutex,
1. spawning a thread,
1. joining a thread,
1. writing tests or examples

or in other patterns where using them makes the code _much simpler_ and it is _obvious at first glance_ to the reader (even one unfamiliar with the code) that the value cannot be `None`/`Err`.

In these cases, prefer to use the macro from the [`unwrap` crate](https://crates.io/crates/unwrap).

## Threads

Generally avoid detached threads. Give child threads meaningful names.

## Function ordering

In `impl`s, always put public functions before private ones.

## Bringing names into scope (`use` statements)

Generally `use` statements should be employed to bring names from different modules into scope. However, functions from other modules should not be brought fully into scope. Instead their module should be brought into scope meaning that subsequent usage of the function requires one level of qualification. For example, if we have:

```rust
pub mod a {
    pub mod b {
        pub struct Harbour {}
        pub fn bar() {}
    }
}
```

then the normal `use` statement to bring these into scope would be:

```rust
use a::b::{self, Harbour};
```

Requiring functions to be module-qualified allows generically-named functions to be disambiguated, particularly given that [stuttering is discouraged](https://github.com/rust-lang-nursery/rust-clippy/wiki#stutter). For example, `encode()` could exist as a function in modules `hex`, `base32` and `base64`. That function shouldn't be named e.g. `hex::hex_encode()`, so when we use it, it's clearer to write `hex::encode()` rather than just `encode()`.

This policy on imports applies to all repositories apart from safe_client_libs, where functions are also fully brought into scope. This is because the safe_client_libs workspace has many instances of functions which if partially qualified would make the code unnecessarily verbose.

## Cargo

Use `cargo-edit` to update dependencies or keep the `Cargo.toml` in the formatting that `cargo-edit` uses.

## Other crates

Adding new dependencies to Sonalake crates in general should be discussed in the team first, except if other Sonalake crates already have the same dependency. E.g. [quick-error](https://crates.io/crates/quick-error) and [unwrap](https://crates.io/crates/unwrap) are fine to use.

## Git Commit Messages

The first line of the commit message should have the format `<type>/<scope>: <subject>`. For details see the [Leaf project's guidelines](https://github.com/autumnai/leaf/blob/master/CONTRIBUTING.md#git-commit-guidelines).