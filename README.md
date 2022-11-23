# Simple Compiler in Rust

Tested on Ubuntu.

## Requirements

- nasm
- ld

## Examples
See `spec/*` for more examples

```shell
$ bin/exec "1+1"
2
```
```shell
$ bin/exec 'print("Hello, World!")'
Hello, world!
```

## Usage

```shell
$ res [input_file?] [options]
```

Quote the input to escape shell expansion, 
e.g. `res -e "(1+1)*2"` instead of `./res -e (1+1)*2`

### Options

| Command          | Description                            | Default     |
|------------------|----------------------------------------|-------------|
| `-o`, `--output` | Output assembly file path              | `'out.asm'` |
| `-e`, `--eval`   | Evaluate and print a single expression |             |
| `-x`, `--exec`   | Prints final expression                | `false`     |

## Commands

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