// discac - small program to change your Discord bot's avatar
// Copyright (C) 2021 N0D4N
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::{from_str as json_from_string, to_string_pretty as json_to_string};
use serenity::{http::client::HttpBuilder, utils::read_image};
use std::fs::{read_dir, read_to_string as read_file_to_string, write as write_to_file};

const CONFIG_FILE_NAME: &'static str = "config.json";
const DATA_FILE_NAME: &'static str = "data.json";

#[derive(Serialize, Deserialize)]
struct Avatars {
    avatars: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    token: String,
    avatars_dir: String,
}

#[tokio::main]
async fn main() {
    let config = get_config();
    let mut pathes_to_avatars = match read_file_to_string(DATA_FILE_NAME) {
        Ok(v) => {
            let avatars: Avatars = json_from_string(&v)
                .expect(format!("Couldn't parse {} into proper json", DATA_FILE_NAME).as_str());
            if !avatars.avatars.is_empty() {
                avatars.avatars
            } else {
                get_avatars(&config.avatars_dir)
            }
        }
        _ => get_avatars(&config.avatars_dir),
    };

    let pth = pathes_to_avatars.pop().unwrap();
    save_current_state(pathes_to_avatars);

    println!("New avatar will be {}", pth);
    change_avatar(&config.token, &pth).await;
}

fn save_current_state(pathes_to_avatars: Vec<String>) {
    let avatars = Avatars {
        avatars: pathes_to_avatars,
    };
    write_to_file(
        DATA_FILE_NAME,
        json_to_string(&avatars)
            .expect(format!("Couldn't convert {:?} into proper json", &avatars.avatars).as_str()),
    )
    .expect(format!("Couldn't write data file to {}", DATA_FILE_NAME).as_str());
}

fn get_config() -> Config {
    let config_str = read_file_to_string(CONFIG_FILE_NAME)
        .expect(format!("Couldn't read {} file to string", CONFIG_FILE_NAME).as_str());
    json_from_string(&config_str)
        .expect(format!("Couldn't parse {} into proper json", CONFIG_FILE_NAME).as_str())
}

async fn change_avatar(token: &String, path_to_new_avatar: &String) {
    let http = HttpBuilder::new(&token).await
        .expect("Couldn't' build http");
    let base64 = read_image(&path_to_new_avatar).expect("Couldn't get image");
    let mut current_user = http.get_current_user().await.expect("Couldn't get current user");
    current_user.edit(http, |p| p.avatar(Some(&base64))).await
        .expect("Couldn't update profile picture");
}

fn get_avatars(path: &str) -> Vec<String> {
    let mut avatars: Vec<String> = read_dir(path)
        .expect(format!("Couldn't read files from {} directory", path).as_str())
        .filter(|x| x.as_ref().unwrap().file_type().unwrap().is_file())
        .filter(|x| {
            match x
                .as_ref()
                .unwrap()
                .path()
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
            {
                "jpg" | "png" => true,
                _ => false,
            }
        })
        .map(|y| String::from(y.unwrap().path().to_str().unwrap()))
        .collect();
    if avatars.is_empty() {
        panic!("There is no jpg/png files in {} directory", path);
    }
    avatars.shuffle(&mut rand::thread_rng());
    avatars
}
