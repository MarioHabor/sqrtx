# SqrtCalculator

![Crates.io](https://img.shields.io/crates/v/sqrtx) ![Docs.rs](https://docs.rs/sqrtx/badge.svg)

A Rust library for calculating square roots asynchronously and synchronously, optimized for high-performance workloads. 
Includes robust error handling and support for parallel processing.

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Examples](#examples)
- [License](#license)

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
sqrtx = "0.1.2"

#### 5. **Usage**

**Example:**
```markdown
## Usage

### Synchronous Example
```rust
use sqrtx::square_root;

fn main() {
    let result = square_root(144.0).unwrap();
    println!("The square root is {}", result);
}

### Asynchronous Example
```rust
use sqrtx::square_root_async;

#[tokio::main]
async fn main() {
    let result = square_root_async(144.0).await.unwrap();
    println!("The square root is {}", result);
}



#### 6. **Features**

## Features

- Supports synchronous and asynchronous computations.
- Parallel processing for heavy workloads.
- Comprehensive error handling.
- Lightweight and fast.


## Examples

### Parallel Processing

```rust
use sqrtx::square_roots_parallel;

#[tokio::main]
async fn main() {
    let numbers = vec![4.0, 16.0, 25.0];
    let results = square_roots_parallel(numbers).await.unwrap();
    println!("The square roots are: {:?}", results);
}


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
