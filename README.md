# Spark-X - A small tool to measurement tool

## Why Spark-X?

In order to automate the benchmark process and to run benchmarks multiple times, Spark-X was created to fullfill this purpose. Spark-X is able to collect timestamps emitted by run executables and accumulate the results to create averaged/min/max benchmark results.

## How does it work?

Spark-X receives the executable to measure as a command-line parameter and can take a list of parameters that it will feed to the executable upon calling it. This is needed to enable the measured binary to accept parameters (in the context of the Spark project, both benchmark applications need the id of the test scenario as a command-line parameter). The executed binary has to emit the measurement results in mircoseconds on the standard output. Spark-X will collect these times and accumulate them, creating averaged, min and max benchmark results. By providing the *--i* parameter the user can specify how often Spark-X runs the executable before emitting results. 

## How are the results formatted?

Spark-X can format the result in the following styles:

- console output:   logging the results to stdout in a visually well readable format
- csv output:       logging the results to stdout in the format: <AVG_TIME>;<MINIMAL_TIME>;<MAXIMUM_TIME>;
