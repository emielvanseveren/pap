# Simple profiler

In the previous section, we explored the use of `RDTSC` (`Read Time Stamp Counter`).
This is an 8086 assembly operation available on nearly all Intel architectures. Within the `timed_haversine` crate, we initially employed a rudimentary method where one had to manually compute the duration between two time stamps. This approach can be both tedious and susceptible to errors.

To enhance this, this crate introduces a straightforward profiler.
This profiler automatically tracks the duration of a specific scope. It records a start time stamp upon creation and automatically logs an end time stamp when it is dropped.

## Problem 1

The current implementation is suboptimal. It employs a multithreaded approach in a single-threaded context, leading to unnecessary overhead from frequent locking and unlocking. This inefficiency is evident in the
profiling results, with parsing consuming only 83% of the time and virtually no time spent on other tasks.

## Problem 2

If you look at the output of `profiled_haversine` (**Listing 76**) of e.g.

```
cargo run --release --bin profiled_haversine ../haversine_gen/data_10000000_flex.json
```

You might notice that the cumulative time recorded exceeds 100%. This discrepancy arises because some profiles may represent subsets of others. For instance, if we profile a function that occupies 90 of the total time, and then separately profile a section within that function
which takes up 20%, the total will appear inflated. To clarify this overlap, we should present the results hierarchically, perhaps using indentation for better visualization.

**The best method to solve this will be explained in the next homework.**
