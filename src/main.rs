#![windows_subsystem = "windows"]
mod savedata;
mod unlocker;

slint::include_modules!();

use savedata::ISAAC_FOLDER;

use crate::unlocker::Unlocker;
fn main() {
	if let Err(er) = ISAAC_FOLDER.as_ref() {
		let title: Vec<u16> = "Error\0".encode_utf16().collect();
		let text: Vec<u16> = (er.to_string()+ "\0").encode_utf16().collect();
		unsafe {
			use std::ptr::null_mut as NULL;
			use winapi::um::winuser;
			winuser::MessageBoxW(NULL(), text.as_ptr(), title.as_ptr(), winuser::MB_OK | winuser::MB_ICONERROR);
		}
		return;
	}
	let unlock = Unlocker::new();
	unlock.init();
	unlock.run();
}
