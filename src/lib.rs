extern crate yaml_rust;

mod yaml_handler;
use yaml_handler::YamlHandler;

#[test]
fn it_works() {
    let mut handler = YamlHandler::new();

    handler.add_files(vec![
        "app/ressources/config/parameters.yml"
    ]);

    let data = match handler.read_all_files() {
        Some(data)  => data,
        None           => {
            println!("none here test!!");
            return;
        }
    };

    println!("{}", data["parameters.yml"]["server"]["hostname"].as_str().unwrap());
}