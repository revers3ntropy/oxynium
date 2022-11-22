# Simple Compiler in Rust

Tested on Ubuntu.

## Requirements

- Rust
- nasm
- ld

## Examples
See `spec/*` for more examples

```shell
$ bin/run "1+1"
2
```
```shell
$ bin/run "hello_world()"
Hello, world!
```

## Usage

```shell
$ ./res [input_file?] [options]
```

Quote the input to escape shell expansion, 
e.g. `./res -e "(1+1)*2"` instead of `./res -e (1+1)*2`

### Options

| Command          | Description                            | Default   |
|------------------|----------------------------------------|-----------|
| `-o`, `--output` | Output assembly file path              | `out.asm` |
| `-e`, `--eval`   | Evaluate and print a single expression |           |

## Commands

```shell
# Build
$ bin/build

# Run
$ bin/run "1+1"

# Run, showing Rust's compilation logs
$ bin/exec "1+1"

# Test
$ bin/test

# Assemble a file
$ bin/asm <file>
```