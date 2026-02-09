# Problem
You need to add middleware(s) for handlers in an HTTP server.

# Solution
Use `axum::middleware` and `tower-governor`.

# Notes
Here, we apply two middlewares (log and rate limiting) to our `/health` route. Note that in a real case scenario, one would choose carefully what middlewares should be applied to what routes. 

Run `cargo run` and use your favorite API tool to send requests to `http://localhost:8080/health`. To try out the rate limiter, spam `curl "http://localhost:8080/health"`
