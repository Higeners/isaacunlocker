#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod savedata;
mod unlocker;

slint::include_modules!();

// CONFIG_PATH is PathBuf because Path can't be static
lazy_static::lazy_static! {
	pub static ref CONFIG_PATH: PathBuf = dirs_next::config_dir().unwrap().as_path().join("IsaacUnlocker\\config.ini");
}


use std::path::PathBuf;

use savedata::ISAAC_FOLDER;

use crate::unlocker::Unlocker;
fn main() {
	let conf = ini::Ini::load_from_file(CONFIG_PATH.as_path());
	let is_cloud = {
		let path = dirs_next::document_dir().unwrap().as_path().join("My Games\\Binding of Isaac Repentance\\options.ini");
		let options = ini::Ini::load_from_file(path).expect("Failed to open options file");
		options.section(Some("Options")).unwrap().get("SteamCloud").unwrap() == "1"
	};
	let config: ini::Ini = {
		let mut c;
		match conf {
			Ok(config) => {
				c = config;
			},
			Err(_) => {
				let conf = ini::Ini::new();
				std::fs::create_dir(CONFIG_PATH.parent().unwrap()).expect("Failed to create directory");
				std::fs::File::create(CONFIG_PATH.as_path()).expect("Failed to create file");
				c = conf;

			}
		};
		c.with_section(Some("Init"))
			.set("CloudEnabled", (is_cloud).to_string());
		c.write_to_file(CONFIG_PATH.as_path()).expect("Failed to create config file");
		c
	};
	let unlock = Unlocker::new(config);
	match unlock {
		Err(er) => {
			let title: Vec<u16> = "Error\0".encode_utf16().collect();
			let text: Vec<u16> = (er.to_string()+ "\0").encode_utf16().collect();
			unsafe {
				use std::ptr::null_mut as NULL;
				use winapi::um::winuser;
				winuser::MessageBoxW(NULL(), text.as_ptr(), title.as_ptr(), winuser::MB_OK | winuser::MB_ICONERROR);
			}
			return;
		}
		Ok(mut unlock) => {
			unlock.init();
			unlock.run();
		}
	}

}
