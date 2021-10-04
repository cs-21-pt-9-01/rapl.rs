# RAPL rs

## Installation

### Setup

```
# set exec permissions on scripts
$ chmod -R +x benchmark/
```

### Build & run
```
# build
$ cargo build --release

# run
$ sudo ./target/release/raplrs
```

## Other stuff

### Scripts
Scripts are located in `./benchmarks/`. We discern between three types:

- `interactive`: an application that expects input, e.g., a GUI. `stdout` and `stderr` should be piped to `/dev/null`, see `./benchmark/interactive/cura.sh` for an example.
- `macro`: larger benchmarks that perform several tasks. TBA, `./benchmark/macro`
- `micro`: smaller benchmarks that perform a single task. `./benchmark/micro`

### CSV output
`raplrs` outputs data from each poll in `csv` format. Sample output:

```
zone        time elapsed (s)    power used (joules)     avg watt usage          avg watt usage since last poll  start power (joules)    previous power          previous power reading
package-0,  28.030436537,       517.9944529999993,      18.479764219175102,     17.634610487045965,             14719.149051,           500.34430299999985,     15237.143504
core,       28.030548686,       364.91492099999596,     13.01855967970924,      11.588462316530928,             55951.045459,           353.3162470000025,      56315.96038
uncore,     28.030660372,       39.756702999999106,     1.4183443342235091,     2.070532459219232,              4492.606319,            37.684346000000005,     4532.363022
```

## Usage
```
$ sudo ./raplrs 
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
    inline           Inline output of a given metric
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

If using a tiling WM, fullscreen your IDE/terminal to avoid botched output.

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
$ sudo ./raplrs benchmark-int benchmark/interactive/cura.sh 
Running application "benchmark/interactive/cura.sh". Ctrl+C to exit. Exiting will kill "benchmark/interactive/cura.sh" as well
time (s)                 J since start            avg w since start        avg w since last poll    w/h                      kw/h
27                       156.017                  5.776                    0.000                    0.04334                  0.00004
```

### `inline`
Inline measurement output for different metrics.

Eligible metrics: 
- `joules`: joule consumption
- `avg_watt`: average watt usage, accumulated since start of execution
- `avg_watt_curr`: average watt usage, at current instant for each update
- `watt_h`: watt hours consumed, accumulated since start of execution
- `kwatt_h`: kilowatt hours consumed, accumulated since start of execution

```
raplrs-inline 0.1.0

USAGE:
    raplrs inline [OPTIONS] <metric>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delay <delay>    Delay between polls (ms) [default: 1000]

ARGS:
    <metric>    What to measure
```

```
$ sudo ./raplrs inline watt_h
0.23560
```

### `list`
List utility for various information.

Eligible input:
- `zones`: list all available power zones in CPU

```
raplrs-list 0.1.0
list

USAGE:
    raplrs list <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>    What to list
```

```
$ sudo ./raplrs list zones
RAPLZone { path: "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0", name: "package-0" }
RAPLZone { path: "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/intel-rapl:0:0", name: "core" }
RAPLZone { path: "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/intel-rapl:0:1", name: "uncore" }
```