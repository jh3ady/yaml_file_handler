use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use yaml_rust::yaml::Yaml;
use std::path::Path;
use std::collections::HashMap;

pub struct FileHandler {
    files: Vec<(String, String)>,
    files_contents: HashMap<String, Yaml>,
}

impl FileHandler {
    pub fn new() -> FileHandler {
        FileHandler {
            files: Vec::new(),
            files_contents: HashMap::new(),
        }
    }

    pub fn add_files(&mut self, files: Vec<&str>) -> bool {
        for file in files {
            let mut fd = match File::open(file) {
                Ok(fd) => fd,
                Err(..) => {
                    println!("[YamlFileHandler Error]: Cannot open the following destination: '{}'", file);
                    return false;
                }
            };
            let mut data = String::new();
            let _ = fd.read_to_string(&mut data);
            self.files.push((file.to_string(), data));
        }
        true
    }

    pub fn read_all_files(&mut self) -> Option<HashMap<String, Yaml>> {
        for file_content in &self.files {
            match file_content {
                &(ref destination, ref content) => {
                    let docs = match YamlLoader::load_from_str(&content) {
                        Ok(docs) => docs,
                        Err(e) => {
                            println!("[YamlFileHandler Error]: {}", e);
                            return None;
                        },
                    };
                    let path = Path::new(destination);
                    self.files_contents.insert(path.file_stem().unwrap().to_str().unwrap().to_string(), docs[0].clone());
                },
            }
        }
        Some(self.files_contents.clone())
    }
}
