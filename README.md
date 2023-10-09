# base-to-base

`base-to-base` is a little Rust program to convert numbers between bases.

It aims to be as quick to use as possibe.

## Install

You'll need [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) for this.

```
cargo install btb
```

This will place `btb` in `~/.cargo/bin`. Make sure it's in your `PATH`.

## Usage

```
btb <Base> <Number> [...args]`
```

`Base` is the base of the input `Number`. It can be any of the following values:

```
hex x
dec d
oct o
bin b
```

### Args

```
--to <Base>: output just one base in plaintext
```
