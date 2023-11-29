// discac - small program to change your Discord bot's avatar
// Copyright (C) 2021-2022 N0D4N
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
	clippy::wildcard_dependencies,
	clippy::cargo_common_metadata
)]

use std::collections::VecDeque;
use std::env;
use std::fs::{canonicalize as to_absolute_path, read_dir, write as write_to_file};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader as json_from_reader, to_string_pretty as json_to_string};
use serenity::{builder::CreateAttachment, builder::EditProfile, http::Http};

const CONFIG_FILE_NAME: &str = "config.json";
const DATA_FILE_NAME: &str = "data.json";
const FOLDER_WITH_PROFILES_ENV_VAR_NAME: &str = "DISCAC_PROFILES_DIR";

#[derive(Serialize, Deserialize)]
struct Avatars {
	/// Vec of pathes to avatars
	avatars: Vec<String>,
	/// Path to avatar currently being in use
	current: Option<String>,
}

#[derive(Deserialize)]
struct Config {
	/// Token of your Discord bot
	token: String,
	/// Path to directories with avatars in it
	avatars_dirs: Vec<String>,
	/// Should avatars be fetched from subdirectories of directories specified in `avatars_dirs`
	#[serde(default = "bool::default")]
	should_get_avatars_from_subdirectories: bool,
}

impl Config {
	fn from_file(path_to_config: &Path) -> Self {
		json_from_file(path_to_config)
	}
}

struct Pathes {
	path_to_config: PathBuf,
	path_to_data: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let pathes = get_config_and_data_path();
	let config = Config::from_file(&pathes.path_to_config);

	let mut avatars = get_current_state(&config, &pathes.path_to_data);

	avatars.current = Option::Some(avatars.avatars.remove(0));

	println!("New avatar will be {}", avatars.current.as_ref().unwrap());
	change_avatar(&config.token, avatars.current.as_ref().unwrap()).await;
	save_current_state(&avatars, &pathes.path_to_data);
}

fn save_current_state(avatars: &Avatars, path_to_data: &Path) {
	write_to_file(
		path_to_data,
		json_to_string(avatars).unwrap_or_else(|e| {
			panic!(
				"Couldn't convert {:?} into proper json. Error message: {e}",
				avatars.avatars
			)
		}),
	)
	.unwrap_or_else(|e| panic!("Couldn't write data file to {path_to_data:?}. Error message: {e}"));
}

fn get_config_and_data_path() -> Pathes {
	let dir_with_data_and_config = get_dir_with_data_and_config();
	let path_to_data = dir_with_data_and_config.join(DATA_FILE_NAME);

	let path_to_config = dir_with_data_and_config.join(CONFIG_FILE_NAME);
	assert!(
		path_to_config.is_file(),
		"{}",
		format!("{path_to_config:?} isn't a file")
	);
	Pathes {
		path_to_config,
		path_to_data,
	}
}

fn get_dir_with_data_and_config() -> PathBuf {
	env::var(FOLDER_WITH_PROFILES_ENV_VAR_NAME).map_or_else(|_| {
		println!("Environment variable \"{FOLDER_WITH_PROFILES_ENV_VAR_NAME}\" not set, assuming single profile mode, where \"{DATA_FILE_NAME}\" and \"{CONFIG_FILE_NAME}\" are located in the same directory as \"discac\" executable");
		env::current_dir().expect("Couldn't get current dir")
	}, |val| {
		let dir_with_profiles = Path::new(&val);
		assert!(
			dir_with_profiles.is_dir(),
			"{}",
			format!(
				"Value of {FOLDER_WITH_PROFILES_ENV_VAR_NAME} environment variable isn't a directory"
			)
		);
		let profile_name = &env::args().nth(1).expect("Can't get name of profile");

		let path_to_dir_with_data_and_config = dir_with_profiles.join(profile_name);
		assert!(
			&path_to_dir_with_data_and_config.is_dir(),
			"{}",
			format!("{path_to_dir_with_data_and_config:?} isn't a directory")
		);
		path_to_dir_with_data_and_config
	} )
}

fn get_current_state(config: &Config, path_to_data: &Path) -> Avatars {
	if path_to_data.exists() {
		let mut avatars: Avatars = json_from_file(path_to_data);
		if avatars.avatars.is_empty() {
			avatars.avatars = get_avatars(
				&config.avatars_dirs,
				config.should_get_avatars_from_subdirectories,
			);
			let mut rng = thread_rng();
			let default = &String::default();
			let current = avatars.current.as_deref().unwrap_or(default);
			loop {
				avatars.avatars.shuffle(&mut rng);
				if !avatars.avatars.first().unwrap().eq(current) {
					break;
				}
			}
		}
		avatars
	} else {
		println!("File with data doesn't exist");
		Avatars {
			avatars: get_avatars(
				&config.avatars_dirs,
				config.should_get_avatars_from_subdirectories,
			),
			current: Option::None,
		}
	}
}

async fn change_avatar(token: &str, path_to_new_avatar: &str) {
	let http = Http::new(token);
	let base64 = CreateAttachment::path(path_to_new_avatar)
		.await
		.expect("Couldn't get image");
	let mut current_user = http
		.get_current_user()
		.await
		.expect("Couldn't get current user");
	current_user
		.edit(&http, EditProfile::new().avatar(&base64))
		.await
		.expect("Couldn't update profile picture");
}

fn get_avatars(pathes: &[String], should_read_from_subdirs: bool) -> Vec<String> {
	assert!(
		!pathes.is_empty(),
		"There must be more than 0 pathes to directory/ies with avatars"
	);
	let mut pathes_to_traverse: VecDeque<String> = VecDeque::with_capacity(pathes.len());
	pathes_to_traverse.extend(pathes.to_owned());

	let mut avatars: Vec<String> = Vec::default();
	loop {
		let option = pathes_to_traverse.pop_front();
		if option.is_none() {
			break;
		}
		let path = option.unwrap();
		for path in read_dir(path)
			.expect("Couldn't read files from directory")
			.filter_map(|x| {
				let val = x.as_ref().unwrap().file_type().unwrap();
				if val.is_dir() {
					Some((x, true))
				} else if val.is_file() {
					Some((x, false))
				} else {
					None
				}
			})
			.map(|x| (x.0.as_ref().unwrap().path(), x.1))
			.filter(|x| {
				if x.1 {
					true
				} else {
					matches!(
						x.0.extension().unwrap_or_default().to_str().unwrap(),
						"jpg" | "png"
					)
				}
			})
			.map(|y| {
				(
					String::from(
						to_absolute_path(&y.0)
							.unwrap_or_else(|e| {
								panic!(
									"Couldn't convert \"{}\" to absolute path. Error message: {e}",
									y.0.to_str().unwrap()
								)
							})
							.to_str()
							.unwrap(),
					),
					y.1,
				)
			}) {
			if path.1 && should_read_from_subdirs {
				pathes_to_traverse.push_back(path.0);
			} else {
				avatars.push(path.0);
			}
		}
	}
	assert!(
		avatars.len() >= 2,
		"There must be 2 or more jpg/png files in {} directory/ies to make use of discac utility",
		pathes.join(",")
	);
	avatars
}

fn json_from_file<T>(path: &Path) -> T
where
	T: serde::de::DeserializeOwned,
{
	json_from_reader(BufReader::new(std::fs::File::open(path).unwrap_or_else(
		|e| {
			panic!(
				"Couldn't open {:?} file. Error message: {e}",
				to_absolute_path(path).unwrap()
			)
		},
	)))
	.unwrap_or_else(|e| panic!("Couldn't parse {path:?} as json.  Error message: {e}"))
}
