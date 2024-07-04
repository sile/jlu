jlu
===

[![jlu](https://img.shields.io/crates/v/jlu.svg)](https://crates.io/crates/jlu)
[![Actions Status](https://github.com/sile/jlu/workflows/CI/badge.svg)](https://github.com/sile/jlu/actions)
![License](https://img.shields.io/crates/l/jlu)

Command-line utilities for on-the-fly investigation of JSON Lines.

```console
$ jlu
Command-line utilities for on-the-fly investigation of JSON Lines

Usage: jlu <COMMAND>

Commands:
  count    Read JSON objects from stdin and count the occurrences of the values associated with the specified top-level member names
  flatten  Read JSON values from stdin and convert each value into a flattened JSON object
  names    Read JSON objects from stdin and output the unique member names for all top-level objects
  rename   Read JSON objects from stdin and rename top-level member names that match a regular expression with a replacement string
  table    Read JSON objects from stdin and create a markdown table
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Installation
------------

Please execute the following command:
```console
$ cargo install jlu
```

In addition, pre-built binaries for Linux and MacOS are available in [the releases page](https://github.com/sile/jlu/releases).

Command Examples
----------------

### `$ jlu latten`

### `$ jlu names`

### `$ jlu rename`

### `$ jlu count`

### `$ jlu table`
