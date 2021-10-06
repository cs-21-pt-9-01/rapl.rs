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

A full sample log can be found in `./logs/`.

## Usage
```
RAPL.rs 0.1.0
cs-21-pt-9-01
RAPL measurement tool

USAGE:
    raplrs [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delay <delay>    Delay between polls (ms) [default: 1000]

SUBCOMMANDS:
    benchmark        Measure power consumption of a oneshot script
    benchmark-int    Measure power consumption of an interactive application
    help             Prints this message or the help of the given subcommand(s)
    inline           Inline output of a given metric
    list             List utility for various RAPL-related information
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
```

```
$ sudo ./raplrs live
Press 'q' to quit
zone                        time(s)                     J                           avg watt                    avg watt curr               w/h                         kw/h
package-0                   21.01861                    401.94149                   19.12318                    19.61201                    0.11165                     0.00011
core                        21.01864                    298.07888                   14.18170                    13.46358                    0.08280                     0.00008
uncore                      21.01866                    20.68407                    0.98409                     2.15952                     0.00575                     0.00001
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
zone                        time(s)                     J                           avg watt                    avg watt curr               w/h                         kw/h
package-0                   5.76740                     116.93481                   20.27543                    0.00000                     0.03248                     0.00003
core                        5.76744                     89.69783                    15.55278                    0.00000                     0.02492                     0.00002
uncore                      5.76750                     4.55624                     0.79001                     0.00000                     0.00127                     0.00000
```

### `benchmark-int`
Benchmark an interactive program.

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
Running application "benchmark/interactive/cura.sh"
'q' or ctrl+c to exit. Ctrl+c will kill "benchmark/interactive/cura.sh" as well
zone                        time(s)                     J                           avg watt                    avg watt curr               w/h                         kw/h
package-0                   24.02615                    473.20264                   19.69537                    20.30874                    0.13145                     0.00013
core                        24.02617                    346.57723                   14.42504                    14.78349                    0.09627                     0.00010
uncore                      24.02619                    28.36818                    1.18072                     1.53713                     0.00788                     0.00001
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