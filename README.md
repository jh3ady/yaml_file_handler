# Yaml File handler

A Yaml File Handler written in Rust. Based on Yaml implementation of [Chyh1990]
(https://github.com/chyh1990/yaml-rust)

## How to use it

Cargo.toml

```toml
yaml_file_handler = "0.1.0"
```

Simple example:

```rust
extern crate yaml_file_handler;
use yaml_file_handler::YamlHandler;

fn main() {
  let mut handler = YamlHandler::new();

  handler.add_files(vec![
      "app/ressources/config/parameters.yml"
  ]);

  let data = match handler.read_all_files() {
      Some(data)  => data,
          println!("none here test!!");
      None           => {
          return;
      }
  };

  println!("{}", data["parameters.yml"]["server"]["hostname"].as_str().unwrap());
}
```
