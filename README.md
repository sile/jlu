jlu
===

[![jlu](https://img.shields.io/crates/v/jlu.svg)](https://crates.io/crates/jlu)
[![Actions Status](https://github.com/sile/jlu/workflows/CI/badge.svg)](https://github.com/sile/jlu/actions)
![License](https://img.shields.io/crates/l/jlu)

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

### `$ jlu flatten`

```console
$ jlu flatten --help
Read JSON values from stdin and convert each value into a flattened JSON object

Usage: jlu flatten

Options:
  -h, --help  Print help

$ jq . example0.json
{
  "aaa": 1,
  "bbb": [
    "a",
    "b",
    "c"
  ],
  "ccc": {
    "x": 10,
    "y": 20
  }
}

$ cat example0.json | jlu flatten | jq .
{
  "aaa": 1,
  "bbb[0]": "a",
  "bbb[1]": "b",
  "bbb[2]": "c",
  "ccc.x": 10,
  "ccc.y": 20
}
```

Note that the following commands assume that the input JSON values are flat JSON objects.

### `$ jlu names`

```console
$ jlu names --help
Read JSON objects from stdin and output the unique member names for all top-level objects

Usage: jlu names

Options:
  -h, --help  Print help

$ cat example0.json | jlu flatten | jlu names
"aaa"
"bbb[0]"
"bbb[1]"
"bbb[2]"
"ccc.x"
"ccc.y"
```

### `$ jlu rename`

```console
$ jlu rename --help
Read JSON objects from stdin and rename top-level member names that match a regular expression with a replacement string.

For details about regular expressions and replacement strings, please refer to the documentation of the regex crate: https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace_all

Usage: jlu rename <REGEX> <REPLACEMENT>

Arguments:
  <REGEX>
          Regular expression to match top-level member names

  <REPLACEMENT>
          String to replace the matched segment of the member names

Options:
  -h, --help
          Print help (see a summary with '-h')

$ cat example0.json | jlu flatten | jlu rename '(.+)\.' '' | jq .
{
  "aaa": 1,
  "bbb[0]": "a",
  "bbb[1]": "b",
  "bbb[2]": "c",
  "x": 10,
  "y": 20
}
```

### `$ jlu count`

```console
$ jlu count --help
Read JSON objects from stdin and count the occurrences of the values associated with the specified top-level member names

Usage: jlu count [NAMES]...

Arguments:
  [NAMES]...  Names of the top-level members to count

Options:
  -h, --help  Print help

$ cat example1.json
{"level":"info","msg":"Hello World!"}
{"level":"error","msg":"Hello Rust!"}
{"level":"info","msg":"Hello JSON!"}

$ cat example1.json | jlu count level | jq .
{
  "error": 1,
  "info": 2
}

$ cat example1.json | jlu count level msg | jq .info
{
  "Hello JSON!": 1,
  "Hello World!": 1
}
```

### `$ jlu table`

```console
$ jlu table --help
Read JSON objects from stdin and create a markdown table

Usage: jlu table [OPTIONS] [COLUMN_NAMES]...

Arguments:
  [COLUMN_NAMES]...  Names of object members to be included in the table

Options:
  -s, --sort <SORT>
          If specified, the table rows are sorted based on the member value associated with this name
  -m, --max-column-chars <MAX_COLUMN_CHARS>
          Maximum number of characters to display in a column [default: 50]
  -h, --help
          Print help
```
