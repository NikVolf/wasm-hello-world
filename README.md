# wasm-hello-world

it's just returns "hello world" whenver called

```rust
#[no_mangle]
pub unsafe extern "C" fn call() {
    ext::ret(b"Hello world");
}
```

To build, run

```
cargo install pwasm-utils --force
./build.sh
```
