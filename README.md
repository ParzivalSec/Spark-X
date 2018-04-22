#Spark-X - A small tool to measure an executable's execution time

## Why Spark-X and not just using a simple clock?

Spark-X was created out of the need for a possibility to run cross-language benchmarks. Due to the need of reliable and comparable times a way has to be found to
measure and compare the runtime costs of applications created in different languages. To fulfill this purpose, all executables participating in the benchmarks need
to be measured with the exact same clock. To ensure this and to provide reliable results, Spark-X allows to specify which executable to measure and how the time
shall be taken for its execution duration. 

## How does it work?

Spark-X receives the executable to measure as a command-line parameter and cann take a list of parameters that it will feed to the executable upon calling it. This is 
needed to enable the measured binary to accept parameters (in the context of the Spark project, both benchmark applications need the id of the test scenario as a command-line parameter).
Spark-X then uses a high-performance clock inernally to measure the time the executable needs till termination. the user can also specify the number of iterations, which will then
call the binary N times and returning the average, min and max measured time.

## How are the results formatted?

Spark-X can format the result in the following styles:

- console output:   logging the results to stdout in a visually well readable format
- csv output:       logging the results to stdout in the format: <AVG_TIME>;<MINIMAL_TIME>;<MAXIMUM_TIME>;