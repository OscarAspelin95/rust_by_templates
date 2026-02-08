# Problem
You want to run an async background task that can check for new messages at a defined tick interval. 

# Solution
Use `tokio::spawn`, `tokio::sync::mpsc` and `tokio::time::interval`.

# Notes
In this basic example, we trigger a background process that periodically checks for new messages send through a `mpsc::unbounded_channel`.
