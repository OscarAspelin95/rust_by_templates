# Problem
You need to create an axum HTTP server with custom extractors for e.g., struct validation.

# Solution
Implement a custom extractor.

# Notes
In this example, we implement a custom extractor (heavily inspired by axums own `Json<T>` extractor) for structs that implement the `Validate` trait.

Run `cargo run` and use your favorite API tool to send post requests to `http://localhost:8080/user`.
