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

#![deny(
	clippy::all,
	clippy::pedantic,
	clippy::nursery,
	clippy::cargo,
	non_ascii_idents,
	unsafe_code,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_results
)]
#![allow(
	clippy::multiple_crate_versions,
	clippy::semicolon_if_nothing_returned,
	clippy::cargo_common_metadata
)]

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader as json_from_reader, to_string_pretty as json_to_string};
use serenity::{http::client::HttpBuilder, utils::read_image};
use std::fs::{read_dir, write as write_to_file, File};
use std::io::BufReader;
use std::path::Path;

const CONFIG_FILE_NAME: &str = "config.json";
const DATA_FILE_NAME: &str = "data.json";

#[derive(Serialize, Deserialize)]
struct Avatars {
	/// Vec of pathes to avatars
	avatars: Vec<String>,
	/// Path to avatar currently being in use
	current: String,
}

#[derive(Deserialize)]
struct Config {
	/// Token of your Discord bot
	token: String,
	/// Path to directory with avatars in it
	avatars_dir: String,
}

#[tokio::main]
async fn main() {
	let config = get_config();
	let mut avatars = get_current_state(&config);

	avatars.current = avatars.avatars.pop().unwrap();
	save_current_state(&avatars);

	println!("New avatar will be {}", avatars.current);
	change_avatar(&config.token, &avatars.current).await;
}

#[inline]
fn save_current_state(avatars: &Avatars) {
	write_to_file(
		DATA_FILE_NAME,
		json_to_string(avatars)
			.unwrap_or_else(|_| panic!("Couldn't convert {:?} into proper json", avatars.avatars)),
	)
	.unwrap_or_else(|_| panic!("Couldn't write data file to {}", DATA_FILE_NAME));
}

#[inline]
fn get_current_state(config: &Config) -> Avatars {
	if Path::new(DATA_FILE_NAME).exists() {
		let avatars: Avatars = json_from_file(DATA_FILE_NAME);
		if avatars.avatars.is_empty() {
			get_avatars(&config.avatars_dir, avatars.current)
		} else {
			avatars
		}
	} else {
		get_avatars(&config.avatars_dir, String::from(""))
	}
}

#[inline]
fn get_config() -> Config {
	json_from_file(CONFIG_FILE_NAME)
}

async fn change_avatar(token: &str, path_to_new_avatar: &str) {
	let http = HttpBuilder::new(&token)
		.await
		.expect("Couldn't' build http");
	let base64 = read_image(&path_to_new_avatar).expect("Couldn't get image");
	let mut current_user = http
		.get_current_user()
		.await
		.expect("Couldn't get current user");
	current_user
		.edit(http, |p| p.avatar(Some(&base64)))
		.await
		.expect("Couldn't update profile picture");
}

fn get_avatars(path: &str, current: String) -> Avatars {
	let mut avatars: Vec<String> = read_dir(path)
		.unwrap_or_else(|_| panic!("Couldn't read files from {} directory", path))
		.filter(|x| x.as_ref().unwrap().file_type().unwrap().is_file())
		.map(|x| x.as_ref().unwrap().path())
		.filter(|x| {
			matches!(
				x.extension().unwrap_or_default().to_str().unwrap(),
				"jpg" | "png"
			)
		})
		.map(|y| String::from(y.to_str().unwrap()))
		.collect();
	if avatars.len() < 2 {
		panic!(
			"There must be 2 or more jpg/png files in {} directory to make use of discac utility",
			path
		);
	}
	let mut rng = thread_rng();
	loop {
		avatars.shuffle(&mut rng);
		if !avatars.first().unwrap().eq(&current) {
			break;
		}
	}
	Avatars { avatars, current }
}

fn json_from_file<T>(path: &str) -> T
where
	T: serde::de::DeserializeOwned,
{
	json_from_reader(BufReader::new(
		File::open(path).unwrap_or_else(|_| panic!("Couldn't open {} file", path)),
	))
	.unwrap_or_else(|_| panic!("Couldn't parse {} as json", path))
}
