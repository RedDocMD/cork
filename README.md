# Cork

![version](https://img.shields.io/crates/v/cork)
![license](https://img.shields.io/crates/l/cork)
[![Rust](https://github.com/RedDocMD/cork/actions/workflows/cargo-test.yml/badge.svg)](https://github.com/RedDocMD/cork/actions/workflows/cargo-test.yml)

![Usage](assets/usage.svg)

Cork is a simple command-line calculator, mainly targeted towards people who deal with hex numbers. It deals only with **integer** arithmetic. Expressions may involve mixed bases (limited to *decimal*, *hexadecimal*, *octal* and *binary* numbers). The global output format may be set to a particular radix - by default it is hex.

## History

Cork is something that I wrote over a weekend, when I was getting annoyed at having to punch in 16 digit hex numbers on my calculator. I wanted something on my screen, and naturally on the terminal. But all the calculator programs that I found online (including a REPL of Python and Octave) had a glaring problem - they could work on hex numbers, but the _output was always in decimal_. So I hit to `cargo new` ... 

### Naming

Cork is a rather odd name for a calculator. I wanted something on the lines of _kernel calculator_, but that's way too long. So *kernel* became *core* (technically, that's what it means in English) and calculator, well that can be *C*. So we have *CoreC* ... maybe *CorC* ... ah right, *Cork*.

## Installation

### Prebuilt binary

For Linux, you can download a pre-built binary [here](https://github.com/RedDocMD/cork/releases/latest).

### Build from source

If you have `cargo` installed, then you can build this from source:

```shell
git clone https://github.com/RedDocMD/cork
cd cork
cargo build --release
```

The binary produced will be `target/release/cork`.

### From crates.io

To install from [crates.io](https://crates.io/crates/cork), run `cargo install cork`. Then, Cork should be executable with the `cork` command.

## Usage

Cork is a REPL calculator, so you can put in expressions and it displays the answer. A sample run goes like:

```
Welcome to cork - a calculator for hex-lovers!
Press Ctrl + D to exit.
cork> 0xCAFE
0xcafe
cork> 0xCAFE + 2 * 8 * 0x20
0xccfe
cork> set of dec
cork> ans
52478
cork> 0xCAFE
51966
Exiting ...

```

Oh, btw, Cork uses [rustyline](https://github.com/kkawakam/rustyline). So you get the whole `readline` goodness (including a history).

### Numbers

Cork accepts four types of numbers:

- Decimal: These are regular numbers (10, 23245, 4124, etc).
- Hexadecimal: These are numbers prefixed by `0x` (0xaA 0x5acd, 0x101c, etc).
- Octal: These are numbers prefixed by `0o` (0o12, 0o55315, 0o10034, etc).
- Binary: These are numbers prefixed by `0b` (0b1010, 0b101101011001101, 0b1000000011100, etc).

In addition, `ans` holds the answer of the last computation. It is initialized to `0` on startup.

### Set directives

Cork has something called set directives, which basically set some global property. They are of the form
```
set <key> <value>
```

As of now, Cork has the following keys:

Key | Possible Values | Purpose
--- | --------------- | --------
of | hex, dec, oct, bin | Sets the output format

## LICENSE

Cork is released under [GNU General Public License, v2](https://github.com/RedDocMD/cork/blob/main/LICENSE).
