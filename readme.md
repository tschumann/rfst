rfst
====

Rust file system tools

Build
-----

```
cargo build
```

A binary gets output in target/debug/

Usage
-----

List files in path:
```
target/debug/rfst test_data/ -l
```

Find duplicate files in path:
```
target/debug/rfst test_data/ -d
```

Test
----

```
./test.sh
```
