# Yaml File handler

rust```
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
