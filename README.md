# Heapo - Heap implementations in Rust

Currently contains:

- [PairingHeap](./src/pairing_heap/mod.rs)

## Usage

```bash
cargo add heapo
```

## Examples

<details>

<summary>PairingHeap</summary>

```rust
    let mut heap = PairingHeap::new();
    heap.insert(253);
    heap.insert(1231);
    heap.insert(65);
    assert_eq!(heap.peek(), Some(&65));
    assert_eq!(heap.pop(), Some(65));
    heap.delete();
    assert_eq!(heap.peek(), Some(&1231));
    assert_eq!(heap.pop(), Some(1231));
    assert_eq!(heap.peek(), None);
    assert_eq!(heap.pop(), None);
    assert_eq!(heap.is_empty(), true);
```

</details>

### License

This project is dual-licensed under the MIT and Apache-2.0 licenses. You can choose either license for your use.
