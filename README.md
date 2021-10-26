# RAPLrs
A Rust wrapper for Intel's Running Average Power Limit (RAPL) present in Intel CPUs since the Sandy Bridge (2nd) generation.

This wrapper functions mostly as a benchmarking tool, however, it also allows for performing live measurements with terminal output.
By default the entire system consumption is measured, however, it is also possible to estimate pure software consumption by offsetting the benchmark consumption based on idle system consumption.

## Table of Contents

- [Installatoin](#installation)
  - [Setup](#setup)
  - [Build & run](#build--run)
  - [Troubleshooting notes](#troubleshooting-notes)
- [Misc](#misc)
  - [Scripts](#scripts)
  - [CSV output](#csv-output)
  - [Isolation data](#isolation-data)
- [Usage](#usage)
  - [`live`](#live)
  - [`benchmark`](#benchmark)
  - [`benchmark-int`](#benchmark-int)
  - [`list`](#list)
  - [`pretty-print`](#pretty-print)
  - [`isolate`](#isolate)
    - [Steps](#steps)

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

# for ease of access you can symlink raplrs to /usr/bin - preferably with absolute paths
$ sudo ln -s /path/to/target/release/raplrs /usr/bin/raplrs
# and then run with
$ sudo raplrs
```

### Troubleshooting notes
If you are running a shell script that calls some command you would usually run from your command line, you need to make sure that said command is present in your `/usr/bin` dir.
For example, [`crispy-doom`](http://manpages.ubuntu.com/manpages/cosmic/man6/crispy-doom.6.html) on Ubuntu by default installs to `/usr/games`, which is not recognized when running `raplrs` as root.

As `raplrs` mostly needs to be run with root access, you need to ensure that any configuration for the benchmark is present where it needs to be.
That is, under `/` rather than your user directory.

## Misc

### Scripts
Scripts are located in `./benchmarks/`. We discern between three types:

- `interactive`: an application that expects input, e.g., a GUI. `stdout` and `stderr` should be piped to `/dev/null`, see `./benchmark/interactive/cura.sh` for an example.
- `macro`: larger benchmarks that perform several tasks. `./benchmark/macro`
- `micro`: smaller benchmarks that perform a single task. `./benchmark/micro`

Micro- and macrobenchmarks should ideally terminate on their own, or be easily killable without interacting with the terminal `raplrs` is being run from.

### CSV output
`raplrs` outputs data from each poll in `csv` format.
For each measurement, rows according to the available zones in your CPU are appended.
Sample output:

```
zone        time elapsed (s)    power used (joules)     avg watt usage          avg watt usage since last poll  start power (joules)    previous power          previous power reading
package-0,  28.030436537,       517.9944529999993,      18.479764219175102,     17.634610487045965,             14719.149051,           500.34430299999985,     15237.143504
core,       28.030548686,       364.91492099999596,     13.01855967970924,      11.588462316530928,             55951.045459,           353.3162470000025,      56315.96038
uncore,     28.030660372,       39.756702999999106,     1.4183443342235091,     2.070532459219232,              4492.606319,            37.684346000000005,     4532.363022
```

A full sample log can be found in `./logs/`.

### Isolation data
Using [`isolate`](#isolate) as a setup tool, `raplrs` can estimate pure software energy consumption by offsetting the measurements using previously measured idle data of the system consumption.
What is necessary for this is the output the `isolate` tool - that is, a JSON on the following format:

```json
[
  "package-0": {
    "power_j": {
      "min": 1.4401819999911822,
      "max": 3.130302000005031,
      "avg": 1.5812111187977085,
      "total": 3315.913507999998
    },
    "watts": { ... },
    "watts_since_last": { ... },
    "watt_h": { ... },
    "kwatt_h": { ... }
  },
  "core": { ... },
  ...
]
```

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
    -d, --delay <delay>                       Delay between polls (ms) [default: 1000]
    -i, --isolate-from <isolate-file>         Idle data to isolate measurements from - see README.md for details
    -n, --name <name>                         Benchmark name - to easily discern csv output
    -t, --terminate-after <run-time-limit>    Terminate after time limit (s)

SUBCOMMANDS:
    benchmark        Measure power consumption of a oneshot script
    benchmark-int    Measure power consumption of an interactive application
    help             Prints this message or the help of the given subcommand(s)
    isolate          Tools for measuring and generating isolation data
    list             List utility for various RAPL-related information
    live             Live measurements
    pretty-print     Pretty print last measurement of .csv file
```

The following system-wide options are available:
- `-d, --delay`: the delay between RAPL polls - that is, the delay between retrieving RAPL measurements from the system in milliseconds. Default is 1.000ms.
- `-n, --name`: an identifier for the benchmark, in addition to the mode that is being run - used to name the output `.csv` file(s). For example, `raplrs -n idle live` would name the `.csv` file something like `idle-live-420.69.csv`.
- `-t, --terminate-after`: the time limit for the benchmark in seconds. For example, `raplrs -t 30 live` would terminate the measurements after 30 seconds.
- `-i, --isolate-from`: idle data to use to isolate software consumption. This should be generated through [`isolate`](#isolate).

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

#### Examples

```
$ sudo raplrs live
Press 'q' to quit
zone                        time(s)                     J                           avg watt                    avg watt curr               w/h                         kw/h
package-0                   21.01861                    401.94149                   19.12318                    19.61201                    0.11165                     0.00011
core                        21.01864                    298.07888                   14.18170                    13.46358                    0.08280                     0.00008
uncore                      21.01866                    20.68407                    0.98409                     2.15952                     0.00575                     0.00001
```

### `benchmark`
Benchmark a single, oneshot program, optionally `n` times. If `-n` is passed, `n` `.csv` files will be generated as well.
By default, `benchmark` expects `<program>` to be executable - alternatively you can specify a runner, e.g., `bash`, with `-r, --runner`.
Additionally, `benchmark` expects `<program>` to terminate on its own - if this is not the case for your benchmark, see [`benchmark-int`](#benchmark-int).

```
raplrs-benchmark 0.1.0
Measure power consumption of a oneshot script

USAGE:
    raplrs benchmark [OPTIONS] <program> [args]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n <n>                   Amount of times to run benchmark [default: 1]
    -r, --runner <runner>    Benchmark requires <runner> to execute

ARGS:
    <program>    Benchmark program
    <args>...    Args for <program>
```

#### Examples

```
$ sudo raplrs benchmark benchmark/micro/fib.sh -n 3
Running benchmark iteration 1
# result omitted
Running benchmark iteration 2
# result omitted
Running benchmark iteration 3
zone                        time(s)                     J                           avg watt                    avg watt curr               w/h                         kw/h
package-0                   5.76740                     116.93481                   20.27543                    0.00000                     0.03248                     0.00003
core                        5.76744                     89.69783                    15.55278                    0.00000                     0.02492                     0.00002
uncore                      5.76750                     4.55624                     0.79001                     0.00000                     0.00127                     0.00000
```

### `benchmark-int`
Benchmark an interactive program.
By default, `benchmark-int` expects `<program>` to be executable - alternatively you can specify a runner, e.g., `bash`, with `-r, --runner`.
Additionally, `benchmark-int` will log results in terminal in an `ncurses` window. To retain availability of the terminal and only log in the background, pass `-b, --bg-log`.

```
raplrs-benchmark-int 0.1.0
Measure power consumption of an interactive application

USAGE:
    raplrs benchmark-int [FLAGS] [OPTIONS] <program>

FLAGS:
    -b, --bg-log     Log in background and post a summary on exit
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --runner <runner>    Benchmark requires <runner> to execute

ARGS:
    <program>    Benchmark program
```

#### Examples

```
$ sudo raplrs benchmark-int benchmark/interactive/cura.sh 
Running application "benchmark/interactive/cura.sh"
'q' or ctrl+c to exit. Ctrl+c will kill "benchmark/interactive/cura.sh" as well
zone                        time(s)                     J                           avg watt                    avg watt curr               w/h                         kw/h
package-0                   24.02615                    473.20264                   19.69537                    20.30874                    0.13145                     0.00013
core                        24.02617                    346.57723                   14.42504                    14.78349                    0.09627                     0.00010
uncore                      24.02619                    28.36818                    1.18072                     1.53713                     0.00788                     0.00001
```

### `list`
List utility for various information.

Eligible input:
- `zones`: list all available power zones in CPU

```
raplrs-list 0.1.0
List utility for various RAPL-related information

USAGE:
    raplrs list <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>    What to list
```

#### Examples

```
$ sudo raplrs list zones
RAPLZone { path: "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0", name: "package-0" }
RAPLZone { path: "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/intel-rapl:0:0", name: "core" }
RAPLZone { path: "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/intel-rapl:0:1", name: "uncore" }
```

### `pretty-print`
Pretty print (like the output measurements from measurement tools) the last measurement of a `.csv` file.

```
raplrs-pretty-print 0.1.0
Pretty print last measurement of .csv file

USAGE:
    raplrs pretty-print <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file>    File to print from
```

#### Examples

```
$ raplrs pretty-print some-log.csv
zone                        time(s)                     J                           avg watt                    avg watt curr               w/h                         kw/h
package-0                   100.09665                   2180.14650                  21.78043                    21.83721                    0.60560                     0.00061
core                        100.09672                   1707.28910                  17.05642                    17.26086                    0.47425                     0.00047
uncore                      100.09676                   87.50553                    0.87421                     0.73465                     0.02431                     0.00002
```

### `isolate`
Measure or generate isolated system consumption data to be used in benchmarks when `-i, --isolate-from` is passed.

By default, `isolate` will measure the system consumption for 30 minutes. 
Alternatively, a different value (in minutes) can be specified with `-m, --measure`.
This data will be logged in a file `idle-isolate-420.69.csv` - an example can be seen in `./logs/`.

To generate the necessary data for isolating the software consumption, pass `-g, --generate` with the file generated from `-m, --measure`.
This will create a JSON file containing `min`, `max`, `avg`, and `total` of the relevant fields used to isolate software consumption.

```
raplrs-isolate 0.1.0
Tools for measuring and generating isolation data

USAGE:
    raplrs isolate [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --generate <file>      Generate isolation data based on input .csv file
    -m, --measure <measure>    Measure data as a basis for isolation for n minutes - make sure your system is as idle as
                               possible [default: 30]
```

#### Steps
```
# first, ensure a stable, idle environment
# then measure the system consumption for however long you want with -m <minutes>
#   this will output a .csv file as "idle-isolate-STAMP.csv"
$ sudo raplrs isolate -m 30

# then convert the previously measured data to json with relevant information with -g <csv_file>
#   this will output a .json file as "isolate-data-STAMP.json"
$ raplrs isolate -g idle-isolate-STAMP.csv

# now use the .json file with the base argument -i, for example
$ sudo raplrs -i isolate-data-STAMP.json live
```