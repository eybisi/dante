# dobby-rs

Dobby is a lightweight, multi-platform, multi-architecture exploit hook framework.

This crate is a rusty binding of [Dobby](https://github.com/jmpews/Dobby).

Caution: Hooking is NOT SAFE! Use at your own risk.

## Quickstart

```rust
use dobby_rs::{resolve_symbol, hook, Address};
use std::mem::transmute;

#[inline(never)]
#[no_mangle]
extern "C" fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[inline(never)]
#[no_mangle]
extern "C" fn sub(a: u64, b: u64) -> u64 {
    a - b
}

unsafe {
    let addr = add as usize as Address;
    let replace = sub as usize as Address;

    let origin = hook(addr, replace).unwrap();
    let origin: extern "C" fn(u64, u64) -> u64 = transmute(origin);

    assert_eq!(origin(2, 1), 2 + 1);
    assert_eq!(add(2, 1), 2 - 1);
}
```

## Supported Target

- Android
    - x86
    - x86_64
    - armv7
    - aarch64

- MacOS
    - x86_64
    - aarch64

- Linux
    - (WIP) x86
    - x86_64
