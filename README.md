func_metadata
---

Rust library for getting metadata about a function

# Usage

```rust
use func_metadata::func_metadata;

#[func_metadata]
fn my_func(a: i32, b: &str ) -> String {
    format!("All works a: {a}, b: {b}")
}

fn main() {
    println!("{}", my_func(1, "foo"));
    // This constant is created by a macro.
    println!("{}", MY_FUNC_JSON)
}
```

Output

```
All works a: 1, b: foo
{"name":"my_func","input":[{"name":"a","type":"i32"},{"name":"b","type":"& str"}],"output":"String"}
```