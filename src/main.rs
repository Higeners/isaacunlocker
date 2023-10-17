#![windows_subsystem = "windows"]
mod savedata;
mod unlocker;

use std::{rc::Rc, fs};

use find_all::FindAll;
use lazy_static::lazy_static;
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

slint::slint! {
	import { GridBox , ScrollView, GroupBox, ListView, HorizontalBox, CheckBox, Button, ComboBox, TabWidget} from "std-widgets.slint";
	import "./src/upheavtt.ttf";
	struct IsaacIcon {
		image: image,
		name: string,
	}
	struct Achievement {
		unlocked: bool,
		id: int,

	}
	struct Item {
		unlocked: bool,
		id: int,

	}


	export global Search {
		in property <[string]> saves: [1,2,3];
		in-out property <[int]> indexes: [
			0,1,2,3,4,5,6,7,8,9,10,11,12,13
		];
		in-out property <[int]> items_indexes: [
			0,1,2,3,4,5
		];
		in property <[IsaacIcon]> icons: [
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
			{image: @image-url("images/achievements/1.png")},
		];
		in property <[IsaacIcon]> items_icons: [
			{image: @image-url("images/items/0.png")},
			{image: @image-url("images/items/1.png")},
			{image: @image-url("images/items/2.png")},
			{image: @image-url("images/items/3.png")},
			{image: @image-url("images/items/4.png")},
			{image: @image-url("images/items/5.png")},
		];
		in-out property <[Achievement]> achievements;
		in-out property <[Achievement]> items;
		in-out property <int> Savefile: 1;
		callback range_change(int, int);
		callback search_change(string);
		callback select_save(int);
	}
	export global UnlockAchievements {
		callback unlock();
	}

	export component Icon inherits Image {
		width: 60px;
		height: 60px;
		in-out property <bool> has-unlocked: Search.achievements[id].unlocked;
		in property <int> id;
		rect := Rectangle {
			callback pressed;
			ta:= TouchArea { 
				clicked() => {
					parent.pressed();
				}
			}
			pressed => {
				Search.achievements[id].unlocked = !Search.achievements[id].unlocked;
				
			}
			border-width: 0px;
			border-radius: 2px;
			border-color: #b44160;

			animate border-width {duration: 100ms ; easing: ease-in;}
			states [ 
				unlocked when root.has-unlocked : {
					border-width: 3px;
					border-color: #4188b4;
				}
				inactive when !ta.has-hover && !root.has-unlocked : {
					border-width: 0px;
				
				}
				active when !root.has-unlocked && ta.has-hover: {
					border-width: 3px;
					border-color: #b44160;
				
				}
			]
		}
	} 
	component InputField {
		in property <length> font-size;
		in property <length> border-width;
		in property <string> input-title;
		in property <color> background-color;
		in property <color> font-color;
		in property <string> font-family;
		in-out property <string> text;
		callback edited;
		callback clear;
		clear => {
			textin.text = "";
			root.text = "";
		}
		HorizontalBox {
			alignment: start;
			spacing: 10px;
			Text{
				font-size: root.font-size;
				text: input-title;
				font-family: root.font-family;
			}
			textin:= TextInput {
				wrap: no-wrap;
				single-line: true;
				font-size: root.font-size;
				font-family: root.font-family;
				color: font-color;
				edited => {
					text = self.text;
					root.edited()
				}
			}
			
		}
		Rectangle {
			z: -10;
			x: textin.x;
			background: background-color;
			clip : true;
			width: textin.width + 10px;
			height: textin.height + 12px;
			border-radius: 4px;
			border-width: root.border-width;
			border-color: gray.darker(10%);
		}
		
	}
	export component App inherits Window {
		title: "Isaac Achievement Unlocker";
		min-width: 200px;
		preferred-width: 600px;
		preferred-height: 800px;
		background: #202325;

		property <int> list-width: 16;//Math.max(Math.ceil(self.width / 100px), 2);
		property <int> item-list-height: Math.ceil(Search.items-indexes.length / list-width);
		property <int> list-height: Math.ceil(Search.indexes.length / list-width);
		Rectangle {
			background: transparent;
			width: input-tab.width;
			height: input-tab.height + self.border-width;
			x: input-tab.x;
			y: input-tab.y;
			border-width: 4px;
			border-radius: 16px;
			border-color: #454242;
		}
		VerticalLayout {
			input-tab:= VerticalLayout {
				padding-left: 6px;
				spacing: 0px;
				search:= InputField {
					input-title: "Search:";
					font-size: 40px;
					font-family: "Upheaval TT (BRK)";
					border-width: 2px;
					background-color: gray.darker(40%);
					font-color: white;
					edited => {
						range-from.clear();
						range-to.clear();
						Search.search-change(self.text);
					}
				}
				HorizontalBox {
					padding: 0px;
					spacing: 0px;
					alignment: start;
					range-from:= InputField {
						input-title: tabs.current-index == 0 ? "Range of achievements:" : "Range of items:";
						
						font-size: 40px;
						font-family: "Upheaval TT (BRK)";
						border-width: 2px;
						background-color: gray.darker(40%);
						font-color: white;
						edited => {
							search.clear();
							Search.range-change(range-from.text.to-float(), range-to.text.is-float() ? range-to.text.to-float(): 637);
						}
					}
					range-to:= InputField {
						input-title: "-";
						font-size: 40px;
						font-family: "Upheaval TT (BRK)";
						
						border-width: 2px;
						background-color: gray.darker(40%);
						font-color: white;
						edited => {
							search.clear();
							Search.range-change(range-from.text.to-float(), range-to.text.is-float() ? range-to.text.to-float(): 637);
						}
					}
				}
				saves:= HorizontalBox {
					alignment: start;
					Text {
						font-size: 40px;
						text: "Savefile: ";
						font-family: "Upheaval TT (BRK)";
					}
					ComboBox { 
						padding-left: 10px;
						width: 100px;
						
						model: Search.saves;
						current-value: Search.Savefile;
						selected(ind) => {
							Search.Savefile = ind.to-float();
							Search.select-save(ind.to-float());
						}
					}	
				}
				HorizontalBox {
					alignment: center;
					Button { 
						text: "Unlock";
						width: 250px;
						clicked => {
							UnlockAchievements.unlock();
						}
					}
				}
				
			}
		tabs:= TabWidget {
			Tab {
				title: "Achievements";
				
				ListView  {
					for i in list-height : HorizontalLayout{
						padding: 4px;
						spacing: 4px;
						property <int> list_actual_width: Math.min(Search.indexes.length - i * list-width, list-width);
						
						for t in list_actual_width : VerticalLayout {
							property <int> index: Search.indexes[t + i * list-width];
							width: 100px;
							Icon {
								
								source: Search.icons[index].image;
								has-unlocked: Search.achievements[index].unlocked;
								id: Search.achievements[index].id;
							}
							Text {
								text: Search.achievements[index].id + 1;
								font-weight: 500;
								font-size: 16px;
								font-family: "Upheaval TT (BRK)";
							}Text {
								text: Search.icons[index].name;
								font-weight: 500;
								font-size: 16px;
								font-family: "Upheaval TT (BRK)";
								wrap: word-wrap;
							}
						}
					}
				}
			}
			Tab {
			title: "Items";
			ListView {
				property <int> list-height: Math.ceil(Search.items-icons.length / list-width);

				for i in list-height : HorizontalLayout{
					padding: 2px;
					spacing: 2px;
					property <int> list_actual_width: Math.min(Search.items-indexes.length - i * list-width, list-width);
					
					for t in list_actual_width : VerticalLayout {
						property <int> index: Search.items-indexes[t + i * list-width];
						width: 100px;
						
						Icon {
							source: Search.items-icons[index].image;
							has-unlocked: Search.items[index].unlocked;
							id: Search.items[index].id;
						}
						Text {
							text: Search.items[index].id + 1;
							font-weight: 500;
							font-size: 16px;
							font-family: "Upheaval TT (BRK)";
						}Text {
							text: Search.items-icons[index].name;
							font-weight: 500;
							font-size: 16px;
							font-family: "Upheaval TT (BRK)";
							wrap: word-wrap;
						}
						}
					}
			}
			}
		}	
	}
	}
}

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
