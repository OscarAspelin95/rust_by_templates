pub struct Fib {
    iterations: usize,
    prev_prev: usize,
    prev: usize,
    pub max_iterations: usize,
}

impl Fib {
    pub fn new(max_iterations: usize) -> Self {
        Self {
            iterations: 0,
            prev_prev: 0,
            prev: 1,
            max_iterations,
        }
    }
}

impl Iterator for Fib {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterations += 1;

        if self.iterations > self.max_iterations {
            return None;
        }

        let current = self.prev_prev;
        let prev = self.prev_prev + self.prev;

        self.prev_prev = self.prev;
        self.prev = prev;

        Some(current)
    }
}
