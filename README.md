![example workflow](https://github.com/revers3ntropy/oxynium/actions/workflows/tests.yml/badge.svg)

# Oxynium Compiler in Rust

Linux/MacOS x86-64 support only so far.

## Requirements

- nasm
- gcc

# Installation

`curl -sSL https://oxynium.org/scripts/install | bash`

### Unstable

*Can also install from development branch, which will include the latest features*

`curl -sSL https://oxynium.org/scripts/install | bash -s -- "latest"`

## Dev Requirements

- cargo
- rustc

#### For running full testing suite:

- python3
- Docker (and cli)

## Examples

See `test/spec/*` for more examples

```shell
$ oxy -e 'print("Hello, World!")' && ./oxy-out
Hello, world!
```

```shell
$ more hello_world.oxy
print("Hello, World!")
$ oxy hello_world.oxy && ./oxy-out
Hello, world!
```

## Usage

```shell
$ oxy [input_file?] [options]
```

### Options

| Command            | Type   | Description                                    | Default        |
|--------------------|--------|------------------------------------------------|----------------|
| `-o`, `--output`   | Str    | Output assembly file path                      | `'out.asm'`    |
| `-e`, `--eval`     | Str    | Pass the program on the CLI                    |                |
| `-k`, `--keep`     | Bool   | Keep outputted `.asm` and `.o` files           | `0`            |
| `-t`, `--target`   | String | Compilation target ('x86_64-linux' or 'macos') | current system |
| `-O`, `--optimise` | Int    | Optimisation level (0 or 1)                    | `1`            |
| `-E`, `--enable`   | String | Enable an optimisation                         |                |
| `-D`, `--disable`  | String | Disable an optimisation                        |                |

#### Note for --eval

Quote the input to escape shell expansion,
e.g. `oxy -e "(1+1)*2"` instead of `oxy -e (1+1)*2`