![example workflow](https://github.com/revers3ntropy/oxynium/actions/workflows/tests.yml/badge.svg)

# Oxynium Compiler in Rust

Tested on Ubuntu.

## Requirements

- nasm
- gcc

# Installation

`curl https://raw.githubusercontent.com/revers3ntropy/oxynium/master/scripts/install | sh`

## Dev Requirements

- cargo
- rustc

## Examples
See `spec/*` for more examples

```shell
$ oxy -e 'print("Hello, World!")' && ./oxy-out
Hello, world!
```

```shell
$ oxy hello_world.oxy && ./oxy-out
Hello, world!
```

## Usage

```shell
$ oxy [input_file?] [options]
```

Quote the input to escape shell expansion, 
e.g. `oxy -e "(1+1)*2"` instead of `oxy -e (1+1)*2`

### Options

| Command             | Type | Description                          | Default                      |
|---------------------|------|--------------------------------------|------------------------------|
| `-o`, `--output`    | Str  | Output assembly file path            | `'out.asm'`                  |
| `-e`, `--eval`      | Str  | Pass the program on the CLI          |                              |
| `-s`, `--std`       | Str  | Path to STD asm file                 | `/usr/local/bin/oxy-std.asm` |
| `-k`, `--keep`      | Bool | Keep outputted `.asm` and `.o` files | `0`                          |
| `-x`, `--exec_mode` | Int  | Exec mode                            | `0`                          |

#### Exec Mode
`0` Compile to application

`1` Compile as library

## Dev Commands

```shell
# Build
$ bin/build

# Compile, assemble, link and run file
$ bin/run <file>

# Compile, assemble, link and print expression
$ bin/exec "1+1"

# Run test suite
$ bin/test

# Assemble a file
$ bin/asm <file>
```