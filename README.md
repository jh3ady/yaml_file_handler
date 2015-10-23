# Yaml File handler

A Yaml File Handler written in Rust. Based on Yaml implementation of [Chyh1990]
(https://github.com/chyh1990/yaml-rust)

## How to use it

Cargo.toml

```toml
yaml_file_handler = "0.1.3"
```

Simple example:

```rust
extern crate yaml_rust;

pub mod yaml_handler;
use yaml_handler::FileHandler;

#[test]
fn it_works() {
    let mut handler = FileHandler::new();

    handler.add_files(vec![
        "parameters.yml",
        "routing.yml"
    ]);

    let config = match handler.read_all_files() {
        Some(data) => data,
        None => return,
    };

    println!("config['parameters']['server']['hostname'] = {}", config["parameters"]["server"]["hostname"].as_str().unwrap());
}
```
