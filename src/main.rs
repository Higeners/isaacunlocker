#![windows_subsystem = "windows"]
mod savedata;
mod unlocker;

slint::include_modules!();

use crate::unlocker::Unlocker;
fn main() {
	let unlock = Unlocker::new();
	unlock.init();
	unlock.run();
}
