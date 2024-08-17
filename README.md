# Things I have learned.

## Rust

### Avoid unnecessary loops.

Each update cycle, I was using a loop to find the neighbors of a given cell and
then check if that cell was alive or not. This meant that each update cycle,
I was throwing away the work to calculate the neighbors. Doing the math ahead of
time and keeping the results stored in an array that I could use to look up the
neighbors of a given cell turned out to be much faster.

### Dealing with the borrow checker can be a pain.

Feels like black magic to me still. I need to go back and spend more time trying
to understand how it actually works and best practices.
