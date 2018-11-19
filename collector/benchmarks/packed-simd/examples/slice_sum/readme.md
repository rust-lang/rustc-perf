# Computes the sum of a slice of floating-point numbers

This example show-cases the performance difference of computing the sum of a
`&[f32]` slice using horizontal or vertical operations. 

To run it:

```
RUSTFLAGS="-C target-cpu=native" cargo run --release
```

On my machine it prints:

```
vertical: 155 ms
horizontal: 424 ms
```

that is, on my particular the slice sum algorithm using horizontal vector
additions operation is ~2.7x slower than the one using vertical vector
operations.
