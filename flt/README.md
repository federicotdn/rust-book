# flt - Fede's Log Tool

## Building
`cd` into the `flt` directory, then run:
```bash
$ cargo build --release
```

The resulting binary will be placed at `target/release/flt`.

## Usage

Pipe a command producing live log output into `flt`. For example:
```bash
$ docker-compose up | flt
```

Command-line options:
```
flt - Fede's Log Tool 1.0
Federico Tedin <federicotedin@gmail.com>
Colorize and separate log output.

USAGE:
    flt [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --separator <SEPARATOR>    Enable log lines separation by timestamp. [default: -]
    -w, --wait-time <TIME (MS)>    Time (ms) to wait before a new log separator is printed. [default: 1000]
```

## To do:
- [ ] Implement log colorizing using regular expressions.
- [ ] Read color options from a config file.
