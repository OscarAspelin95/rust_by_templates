# Problem
You have custom data structures (which share some trait) for which you'd like comparisons like `==`, `>=`, `<=`, etc.

# Solution
Implement the `PartialOrd` and `PartialEq` traits.

# Notes
In this example, we define two structs `Triangle` and `Square` for which we implement a trait `Area`. This in turn enables us to compare the structs with `PartialOrd` and `PartialEq`.
