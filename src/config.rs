use std::fs;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub(crate) place_key: String,
    pub(crate) canvas_width: u32,
    pub(crate) canvas_height: u32,
    pub(crate) ads: Vec<String>,
    pub(crate) motds: Vec<String>
}

pub fn load() -> AppConfig {
    return match fs::read_to_string("./config.json") {
        Ok(s) => {
            return match serde_json::from_str(s.as_str()) {
                Ok(a) => a,
                Err(_e) => {
                    create_default_config();
                    load()
                }
            }
        }
        Err(_) => {
            create_default_config();
            load()
        }
    }
}

fn create_default_config() {
    let mut f = File::create("./config.json").unwrap();

    let str: String = serde_json::to_string_pretty(&AppConfig {
        place_key: String::from("key"),
        canvas_width: 256,
        canvas_height: 256,
        ads: Vec::new(),
        motds: Vec::new()
    }).unwrap();

    f.write_all(str.as_bytes()).unwrap();
}
