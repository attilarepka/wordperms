# wordperms

Generate all unique permutations of words from an input list, with optional capitalization variants.

![wordperms demo](vhs/demo.gif)

## Features
- Generates permutations up to a configurable maximum length.
- Supports multiple capitalization styles:
  - `all` (original, uppercase, first letter capitalized)
  - `none` (keep as-is)
  - `first` (capitalize first letter only)
  - `upper` (all uppercase)
- Limits the total number of results.
- Limit maximum number of words to include in each permutation combination.
- Outputs to a file or stdout.

## Installation

**[Archives of precompiled binaries for wordperms are available for 
macOS and Linux.](https://github.com/attilarepka/wordperms/releases)**

Linux binaries are static executables.

If you're a **Debian** user (or a user of a Debian derivative like **Ubuntu**),
then wordperms can be installed using a binary `.deb` file provided in each
[wordperms release](https://github.com/attilarepka/wordperms/releases).

```
$ curl -LO https://github.com/attilarepka/wordperms/releases/download/0.1.0/wordperms_0.1.0_amd64.deb
$ sudo dpkg -i wordperms_0.1.0_amd64.deb
```

## Building

wordperms is written in Rust, so you'll need [Rust installation](https://www.rust-lang.org/) in order to compile it.
wordperms compiles with Rust 1.70.0 (stable) or newer. In general, it tracks
the latest stable release of the Rust compiler.

```shell
$ git clone https://github.com/attilarepka/wordperms.git
$ cd wordperms
$ cargo build --release
```
## Usage

Wordperms provides a command-line interface with the following options:

```shell
Usage: wordperms [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>          Input file (one word per line)
  -m, --max-len <MAX_LEN>      Max number of words per combination [default: 4]
  -c, --cap-style <CAP_STYLE>  Capitalization style [default: all] [possible values: all, none, first, upper]
  -l, --limit <LIMIT>          Limit number of generated results
  -o, --output <OUTPUT>        Output file (default: stdout)
  -h, --help                   Print help
  -V, --version                Print version
```

## Example
Given `words.txt`:
```
apple
banana
123
```

Run:
```bash
wordperms -i words.txt -m 2 -c all 
```

Produces permutations like:
```
appleBANANA
APPLEbanana123
apple123
APPLEBanana
applebanana
123appleBanana
BANANAApple123
...
```

## Contributing

Contributions are welcome! Open a GitHub issue or pull request.

## License

This project is licensed under the [MIT license](LICENSE)
