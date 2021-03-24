This crate entroduces `is_false!` macro and `is_false()` function which checks if argument is false

```toml
[dependencies] 
is_false = "0.1"
```

## Usage

```rust
use is_false::is_false;

fn main() {
    assert_eq!(is_false!(4 % 2 == 0, 5 % 2 == 0), false);        
    assert_eq!(is_false!(5 % 2 == 0, 1 == 2), true);
}
```

```rust
use is_false::is_false;

fn main() {
    assert_eq!(is_false(5 % 2 == 0), true);
}
```
