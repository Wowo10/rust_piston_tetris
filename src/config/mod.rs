use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

pub struct Config {
    file: File,
    buffer: HashMap<String, String>,
}

impl Config {
    pub fn create(name: &'static str) -> Self {
        Config {
            file: File::open(name).unwrap(),
            buffer: HashMap::new(),
        }
    }

    pub fn read(&mut self, key: &'static str) -> String {

        self.file.seek(SeekFrom::Start(0)).unwrap();

        let match_value = self.buffer.get(key).cloned();

        match match_value {
            Some(review) => review.to_string(),
            None => {
                let mut reader = BufReader::new(&self.file);

                for line in reader.lines() {
                    let [data_key, data_value] = Config::split_data(line.unwrap());

                    if data_key == key.to_string() {
                        self.buffer.insert(data_key.clone(), data_value.clone());

                        return data_value;
                    }
                }

                "".to_string()
            }
        }
    }

    fn split_data(data: String) -> [String; 2] {
        let temp: Vec<String> = data.split(';').map(String::from).collect();

        [temp[0].clone(), temp[1].clone()]
    }
}
