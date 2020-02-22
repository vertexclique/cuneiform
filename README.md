<h1 align="center">
    <img src="https://github.com/vertexclique/cuneiform/raw/master/img/cuneiform-logo.png" width="200" height="200"/>
</h1>
<div align="center">
 <strong>
   In memory optimizations for Rust, revived from the slabs of Sumer.
 </strong>
<hr>

[![Build Status](https://github.com/vertexclique/cuneiform/workflows/CI/badge.svg)](https://github.com/vertexclique/cuneiform/actions)
[![Latest Version](https://img.shields.io/crates/v/cuneiform.svg)](https://crates.io/crates/cuneiform)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cuneiform/)
</div>

This crate provides proc macro attributes to improve in memory data access times.

Cuneiform's main macro can take various arguments at attribute position:
* `hermetic = true|false` (default is `true` when `#[cuneiform]`)
    * Hermetic enables cuneiform to detect cache sizes from OSes which have API to fetch.
    * Currently, hermetic argument works only Linux kernel 2.6.32 and above.
    * If system is different than supported systems it falls back to `slab`s.
* `slab = "board_or_architecture_name` (e.g. `#[cuneiform(slab = "powerpc_mpc8xx")]`)
    * Slabs are either embedded system boards or other specific architecture.
    * Slab checking have two stages:
        * First, it checks the given board/architecture if exist.
        * If not slabs fall back to Rust supported architectures.
        * Still architecture is not detected, it will fall back to default values.
* `force = u8` (e.g. `#[cuneiform(force = 16)]`)
    * Forces a given cache alignment. Overrides all other systems mentioned above.

```toml
[dependencies]
cuneiform = "0.1"
```

## Examples
Basic usage can be:
```rust
// Defaults to `hermetic = true`
#[cuneiform]
pub struct Varying {
    data: u8,
    data_2: u16,
}
```

Targeting specific architecture:
```rust
#[cuneiform(slab = "powerpc_mpc8xx")]
pub struct SlabBased {
    data: u8,
    data_2: u16,
}
```

Overriding the default cache alignment:
```rust
#[cuneiform(force = 16)]
pub struct Forced {
    data: u8,
    data_2: u16,
}
```

## Field level cache optimizations
Check out [cuneiform-fields](https://github.com/vertexclique/cuneiform-fields) for field level optimizations.

## Design choices
* Cuneiform doesn't have specific instruction or architecture specific code.
* Works with crates like `#![no_std]`. Ease your pain for cache optimizations. With allocator you can compile on the board too.
* Not based on assumptions. Based on Linux tree, OS checks, provider manuals and related documentation.

## Before opening a PR
* If it is big.LITTLE architecture, separate both parts in slabs. Apply the naming conventions.
* Check existing slabs before opening a PR. Please update it when you add one.
* If you come up with instructionless detection for hermetic alignment. Be sure that tests are included and not breaking existing platforms.

## Existing Slabs

* powerpc_mpc8xx
* powerpc64bridge
* powerpc_e500mc
* power_7
* power_8
* power_9
* exynos_big
* exynos_LITTLE
* krait
* neoverse_n1
