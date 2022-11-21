# Simple Compiler in Rust

Tested on Ubuntu.

## Requirements

- Rust
- nasm
- ld

## Examples
See `spec/*` for more examples

```shell
$ bin/exec "1+1"
2
```
```shell
$ bin/exec "hello_world()"
Hello, world!
```

## Usage

```shell
$ ./res [input] [options]
```

Quote the input to escape shell expansion, 
e.g. `./res "(1+1)*2"` instead of `./res (1+1)*2`

### Options

| Command            | Description               |
|--------------------|---------------------------|
| `-o`, `--output`   | Output assembly file path |

## Commands

```shell
# Build
$ bin/build

# Run
$ bin/exec "1+1"

# Test
$ bin/test

# Assemble a file
$ bin/asm <file>
```