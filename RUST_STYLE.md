# A note defining how Rust should be written in this project

### Functions that return Self
Functions that return the same struct as the implementation, or whatever, should always be annotated as `fn some() -> Self`

And the return statement should always be
```rust
Self {
    value: 1
}
```

#### Here is an example from the `math` crate
```rust
    pub fn translation(trans_x: f32, trans_y: f32) -> Self {
        Self { matrix: [
            [1.0, 0.0, trans_x],
            [0.0, 1.0, trans_y],
            [0.0, 0.0, 1.0]
        ] }
    }
```

