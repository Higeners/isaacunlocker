#![windows_subsystem = "windows"]
mod savedata;
mod unlocker;

slint::include_modules!();

lazy_static::lazy_static! {
	pub static ref CONFIG_PATH: PathBuf = {
		let mut con = dirs_next::config_dir().unwrap();
		con.push("IsaacUnlocker\\config.ini");
		con
	};
}


use std::path::PathBuf;

use savedata::ISAAC_FOLDER;

use crate::unlocker::Unlocker;
fn main() {
	let conf = {
		let mut con = dirs_next::config_dir().unwrap();
		con.push("IsaacUnlocker/config.ini");

		ini::Ini::load_from_file(con)
	};
	let config: ini::Ini = {
		let c;
		match conf {
			Ok(config) => {
				c = config;
			},
			Err(_) => {
				unsafe {
					use std::ptr::null_mut as NULL;
					use winapi::um::winuser;
					let title: Vec<u16> = "Isaac Achievement Unlocker\0".encode_utf16().collect();
					let text: Vec<u16> = "Is Steam cloud enabled for The Binding of isaac?\0".encode_utf16().collect();
					let res = winuser::MessageBoxW(NULL(), text.as_ptr(), title.as_ptr(), winuser::MB_YESNO | winuser::MB_ICONQUESTION);
					let mut conf = ini::Ini::new();
					std::fs::create_dir(CONFIG_PATH.parent().unwrap()).expect("Failed to create directory");
					std::fs::File::create(CONFIG_PATH.as_path()).expect("Failed to create file");
					conf.with_section(Some("Init"))
						.set("CloudEnabled", (res == winuser::IDYES).to_string());
					conf.write_to_file(CONFIG_PATH.as_path()).expect("Failed to create config file");
					c = conf;
				}

			}
		};
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
