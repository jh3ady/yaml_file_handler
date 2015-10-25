extern crate yaml_rust;
pub use yaml_rust::*;

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
