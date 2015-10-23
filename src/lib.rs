extern crate yaml_rust;

pub mod yaml_handler;
use yaml_handler::YamlHandler;

#[test]
fn it_works() {
    let mut handler = YamlHandler::new();

    handler.add_files(vec![
        "app/ressources/config/parameters.yml"
    ]);

    let config = match handler.read_all_files() {
        Some(data) => data,
        None => return,
    };

    println!("config['parameters.yml']['server']['hostname'] = {}", config["parameters.yml"]["server"]["hostname"].as_str().unwrap());

}
