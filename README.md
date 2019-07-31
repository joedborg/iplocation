# iplocation

Simple Rust library that uses [http://ip-api.com/json](http://ip-api.com/json)
to gain location data based on your public IP address.

## Example

```rust
use iplocation;

fn main() {
    let result = iplocation::get().unwrap();

    println!("{:?}", result);
}
```
