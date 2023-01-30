# 16bit

Based of [16-Bit Virtual Machine](https://github.com/lowbyteproductions/16-Bit-Virtual-Machine) series by [@LowByteProductions](https://www.youtube.com/@LowByteProductions)

## Core

Main vm written in c++ (2z).

Supports original instruction set. Has debug mode.

prerequirments (built with):
```
Meson 0.61.2
Ninja 1.10.1
Clang 14
```

build:
```sh
cd core
meson build
cd build
ninja
```

tests (ut test microframework):
```sh
ninja test
```

## Asm

Asm written purely in rust.

Trying to be close to series as possible.

build:
```sh
cd asm
cargo build
```

tests:
```sh
cargo test
```

### Current state

Currently support expression parsing thrue Pratt parsing method.

Supported some move/add/stack/call instructions.

TODO:
 - [ ] Remove NOM dep from `Cargo.toml` (actually unused)
 - [ ] Add codegen
 - [ ] (Optional) add typechecker
