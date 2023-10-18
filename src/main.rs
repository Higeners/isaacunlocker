//#![windows_subsystem = "windows"]
mod savedata;
mod unlocker;

use std::{rc::Rc, fs};

use find_all::FindAll;
use lazy_static::lazy_static;
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

slint::include_modules!();

lazy_static! {
	pub static ref ISAAC_FOLDER: String = {
		let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
		let steam = hklm.open_subkey(r"SOFTWARE\WOW6432Node\Valve\Steam").expect("Steam is not installed");
		let path: String = steam.get_value("InstallPath").expect("Failed to find Steam folder");
		for p in fs::read_dir(path + r"\userdata").unwrap() {
			for ps in fs::read_dir(p.unwrap().path()).unwrap() {
				if ps.as_ref().unwrap().file_name() == "250900"{
					return ps.unwrap().path().to_str().unwrap().to_string() + r"\remote";
				}
	
			}
		};
		String::new()
	};
}

use crate::unlocker::Unlocker;
fn main() {
	let unlock = Unlocker::new();
	unlock.init();
	unlock.run();
}
