use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use yaml_rust::YamlLoader;
use yaml_rust::yaml::Yaml;

pub struct YamlHandler {
    files_content: Vec<(String, String)>,
    docs_content: HashMap<String, Yaml>,
}

impl YamlHandler {
    pub fn new() -> YamlHandler {
        YamlHandler {
            files_content: Vec::new(),
            docs_content: HashMap::new(),
        }
    }

    pub fn add_files(&mut self, files: Vec<&str>) -> bool {
        for file in files {
            let mut fd = match File::open(file) {
                Ok(fd)      => fd,
                _           => {
                    println!("[YamlHandler Error]: Cannot open the following destination: '{}'", file);
                    return false;
                }
            };

            let mut data = String::new();
            let _ = fd.read_to_string(&mut data);
            self.files_content.push((file.to_string(), data));
        }
        true
    }

    pub fn read_all_files(&mut self) -> Option<HashMap<String, Yaml>> {
        for data_file in self.files_content.clone() {
            match data_file {
                (destination, content) => {
                    let docs = match YamlLoader::load_from_str(&content) {
                        Ok(docs)    => docs,
                        _           => {
                            println!("[YamlHandler Error]: YamlLoader cannot \
                            load the following destination: '{}'", destination);
                            return None;
                        },
                    };

                    let path = Path::new(&destination);
		            self.docs_content.insert(
                        path
                            .file_name().unwrap()
                            .to_str().unwrap()
                            .to_string(),
                        docs[0].clone()
                    );

                    return Some(self.docs_content.clone());
                },
            }
        }
        None
    }

    pub fn read_file(&mut self, file: &str) -> Option<HashMap<String, Yaml>> {
        let mut fd = match File::open(file) {
            Ok(fd)      => fd,
            _           => {
                println!("[YamlHandler Error]: Cannot open the following destination: '{}'", file);
                return None;
            }
        };

        let mut data = String::new();
        let _ = fd.read_to_string(&mut data);

        let docs = match YamlLoader::load_from_str(&data) {
            Ok(docs)    => docs,
            _           => {
                println!("[YamlHandler Error]: YamlLoader cannot \
                load the following destination: '{}'", file);
                return None;
            },
        };

        let path = Path::new(&file);
        let mut hashmap = HashMap::new();
        hashmap.insert(
            path
                .file_name().unwrap()
                .to_str().unwrap()
                .to_string(),
            docs[0].clone()
        );

        Some(hashmap)
    }
}
