use std::{fs, path::PathBuf};

use homedir::my_home;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub person_name: String,
    pub task_status: String,
    pub api_key: String,
    pub font: FontSettings,
    pub width_offset_perc: f32,
    pub start_height: i32,
    pub height_increment: i32,
    pub output_image: OutputImageSettings,
    pub todos_only: bool,
    pub todos_path: String,
    pub bg_set_command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputImageSettings {
    pub path: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontSettings {
    pub path: String,
    pub size: f32,
}

impl Config {
    pub fn get_config() -> anyhow::Result<Config> {
        // Create the directory if it doesn't exist
        // Create the config dir
        let Some(home) = my_home()? else {
            return Err(anyhow::anyhow!("Couldn't find a home directory"));
        };

        let config_path = home.join(".config/tw/");

        let _ = fs::create_dir_all(config_path.clone())?;

        let config_path = if cfg!(debug_assertions) {
            PathBuf::from("config.json")
        } else {
            config_path.join("config.json")
        };

        // Check if config exists
        if !config_path.exists() {
            println!("Config file not found, creating...");

            let config = json!({
                "personName": "Suryansh",
                "apiKey": "lol",
                "font": {
                    "path": "smd.ttf",
                    "size": 30
                },
                "widthOffsetPerc": 50,
                "startHeight": 170,
                "heightIncrement": 50,
                "outputImage": {
                    "path": "~/.config/tw/lmao.png",
                    "width": 1920,
                    "height": 1080
                },
                "todosPath": "~/.config/tw/todos",
                "bgSetCommand": "swaymsg output HDMI-A-1 bg ~/dev/projects/tw/output.png fill"
            });

            let _ = fs::File::create_new(config_path.clone()).expect("couldn't create all the things");
            let _ = fs::write(config_path.clone(), serde_json::to_string(&config).expect("couldn't serialize things"));

            println!("Created");
        }

        let mut config = serde_json::from_str::<Config>(&fs::read_to_string(config_path)?)?;

        // Canonicalize the personal path
        config.todos_path = canonicalize(config.todos_path, home.clone());
        config.font.path = canonicalize(config.font.path, home.clone());
        config.output_image.path = config.output_image.path.replace(
            "~",
            home.to_str()
                .expect("Couldn't get the home directory string"),
        );

        Ok(config)
    }
}

fn canonicalize(path: String, home: PathBuf) -> String {
    String::from(
        fs::canonicalize(PathBuf::from(
            path.replace(
                "~",
                home.to_str().expect(
                    format!("Couldn't get the home directory string for: {}", path).as_str(),
                ),
            ),
        ))
        .expect(format!("Couldn't canonicalize the path of the string: {}", path).as_str())
        .to_str()
        .unwrap(),
    )
}
