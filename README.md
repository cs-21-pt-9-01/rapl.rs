# RAPL rs

```
# build
$ cargo build --release

# run
$ sudo ./target/release/raplrs
```

## Usage
```
$ sudo ./target/release/raplrs 
RAPL.rs 0.1.0
PT10xE21
RAPL measurement tool

USAGE:
    raplrs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    benchmark        Measure power consumption of a oneshot script
    benchmark-int    Measure power consumption of an interactive application
    help             Prints this message or the help of the given subcommand(s)
    live             Live measurements
```

### `live`
Perform live, continuous measurements of power consumption. 

```
raplrs-live 0.1.0
Live measurements

USAGE:
    raplrs live [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delay <delay>    Delay between polls (ms) [default: 1000]
```

```
$ sudo ./raplrs live
time (s)                 J since start            avg w since start        avg w since last poll    w/h                      kw/h
39                       824.494                  21.137                   21.565                   0.22903                  0.00023
```

### `benchmark`
Benchmark a single, oneshot program, optionally `n` times.

```
raplrs-benchmark 0.1.0
Measure power consumption of a oneshot script

USAGE:
    raplrs benchmark [OPTIONS] <runner> <program> [args]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n <n>        Amount of times to run benchmark [default: 1]

ARGS:
    <runner>     Benchmark runner application, e.g., python
    <program>    Benchmark program
    <args>...    Args for <program>
```

```
$ sudo ./raplrs benchmark /usr/bin/bash benchmark/micro/fib.sh -n 3
Running benchmark iteration 1
Running benchmark iteration 2
Running benchmark iteration 3
time (s)                 J since start            avg w since start        avg w since last poll    w/h                      kw/h
8                        206.469                  27.236                   0.000                    0.05735                  0.00006
```

### `benchmark-int`
Benchmark an interactive program.

***Note:** if you exit this benchmark with ctrl+c you discard the results. Make sure to exit the application manually.*

```
raplrs-benchmark-int 0.1.0
Measure power consumption of an interactive application

USAGE:
    raplrs benchmark-int <program>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <program>    Benchmark program
```

```
$ sudo ./raplrs benchmark-int cura
Running application "cura". Exit the application to stop; Ctrl+C will discard results
time (s)                 J since start            avg w since start        avg w since last poll    w/h                      kw/h
24                       561.650                  23.449                   0.000                    0.15601                  0.00016
```