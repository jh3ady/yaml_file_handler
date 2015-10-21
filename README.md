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
      Some(data) => data,
      None => return,
  };

  // parameters.yml
  // server:
  //   hostname: localhost
  //   port: 3000
  // client:
  //   hostname: 00.00.00.00
  //   port: 5642
  //
  // to access to "localhost" : variable["filename+extension"]["server"]["hostname"]
  println!("{}", data["parameters.yml"]["server"]["hostname"].as_str().unwrap());
}
```
