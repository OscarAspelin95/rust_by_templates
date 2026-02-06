# Problem
You have a custom struct for which you would like to implement iteration to enable e.g., `my_data_structure.into_iter().map()....`.

# Solution
Implement the `Iterator` trait for this struct.

# Notes
In this example, we implement an iterator over the fibonacci sequence.
