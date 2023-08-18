//#![windows_subsystem = "windows"]
use std::{collections::HashMap, rc::Rc, path::Path, fs, borrow::BorrowMut};

use find_all::FindAll;
use lazy_static::lazy_static;
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

slint::slint! {
	import { GridBox , ScrollView, GroupBox, ListView, HorizontalBox, CheckBox, Button, ComboBox} from "std-widgets.slint";
	import "./src/upheavtt.ttf";
	struct AchievementIcon {
		image: image,
		name: string,
	}
	struct Achievement {
		unlocked: bool,
		id: int,

	}

	export global Search {
		in-out property <[int]> indexes;
		in property <[AchievementIcon]> icons: [
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
			{image: @image-url("images/1.png")},
		];
		in-out property <[Achievement]> achievements;
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
		out property <string> text;
		callback edited;
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
		preferred-height: 200px;
		background: #202325;

		property <int> list-width: 16;//Math.max(Math.ceil(self.width / 100px), 2);
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
				
				search:= InputField {
					input-title: "Search:";
					font-size: 40px;
					font-family: "Upheaval TT (BRK)";
					border-width: 2px;
					background-color: gray.darker(40%);
					font-color: white;
					edited => {
						Search.search-change(self.text);
					}
				}
				HorizontalBox {
					padding: 0px;
					spacing: 0px;
					alignment: start;
					range-from:= InputField {
						input-title: "Range of achievements:";
						font-size: 40px;
						font-family: "Upheaval TT (BRK)";
						border-width: 2px;
						background-color: gray.darker(40%);
						font-color: white;
						edited => {
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
							Search.range-change(range-from.text.to-float(), range-to.text.is-float() ? range-to.text.to-float(): 637);
						}
					}
				}
				HorizontalLayout {
					alignment: start;
					Text {
						font-size: 40px;
						text: "Savefile: ";
						font-family: "Upheaval TT (BRK)";
					}
					ComboBox { 
						padding-left: 10px;
						width: 100px;
						
						model: [1,2,3];
						current-value: Search.Savefile;
						selected(ind) => {
							Search.Savefile = ind.to-float();
							Search.select-save(ind.to-float());
						}
					}	
				}
				Button { 
					text: "Apply";
					preferred-width: 100px;
					clicked => {
						UnlockAchievements.unlock();
					}
				 }
		}
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
	}
}
const NAMES: &str = include_str!("Achievements.txt");

fn imbed_images() -> Vec<(slint::Image, String)>{
	use include_dir::*;
	use slint::*;
	const IMAGES: Dir = include_dir!("./images");
	let mut files: Vec<&File> = IMAGES.files().collect();
	files.sort_by(|a, b| {

		a.path().to_str().unwrap()
		.strip_suffix(".png")
		.unwrap().parse::<i16>().unwrap()
		.cmp(&b.path().to_str().unwrap()
		.strip_suffix(".png")
		.unwrap().parse::<i16>().unwrap())
	});
	let mut images: Vec<(slint::Image, String)> = vec![];
	for (file, name) in files.iter().zip(NAMES.lines()) {
		let image = image::load_from_memory(file.contents()).unwrap().into_rgb8();
		let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(image.as_raw(), image.width(), image.height());
		let i = Image::from_rgb8(buffer);
		images.push((i, name.to_string()));
	}
	images
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

fn load_achievement_data(savefile: i32) -> Vec<bool> {
	let path = ISAAC_FOLDER.to_string() + format!("\\rep_persistentgamedata{savefile}.dat").as_str();
	let bytes = fs::read(Path::new(path.as_str())).expect("Couldn't open file");
	bytes[33..637+33].iter().fold(Vec::<bool>::new(), |mut acc, x| {
		acc.push( *x != 0);
		acc
	})
	
}

fn unlock_achievements(vec: Vec<u8>, savefile: i32) {
	let path = ISAAC_FOLDER.to_string() + format!("\\rep_persistentgamedata{savefile}.dat").as_str();
	
	let mut bytes = fs::read(Path::new(path.as_str())).expect("Couldn't open file");
	bytes[33..637+33].copy_from_slice(&vec);
	let check = check_sum(bytes[0x10..(bytes.len()-4) as usize].to_vec());
	let check: [u8; 4] = unsafe { std::mem::transmute(check.to_le()) };
	let len = bytes.len();
	bytes[(len-4)..len].copy_from_slice(&check);
	fs::write(Path::new(path.as_str()), bytes).expect("Failed to write to file");
}

fn check_sum(buf: Vec<u8>) -> u32 {
	
	const CRC_TABLE:[u32; 256] = [
		0x00000000, 0x09073096, 0x120E612C, 0x1B0951BA, 0xFF6DC419, 0xF66AF48F, 0xED63A535, 0xE46495A3, 
		0xFEDB8832, 0xF7DCB8A4, 0xECD5E91E, 0xE5D2D988, 0x01B64C2B, 0x08B17CBD, 0x13B82D07, 0x1ABF1D91, 
		0xFDB71064, 0xF4B020F2, 0xEFB97148, 0xE6BE41DE, 0x02DAD47D, 0x0BDDE4EB, 0x10D4B551, 0x19D385C7, 
		0x036C9856, 0x0A6BA8C0, 0x1162F97A, 0x1865C9EC, 0xFC015C4F, 0xF5066CD9, 0xEE0F3D63, 0xE7080DF5, 
		0xFB6E20C8, 0xF269105E, 0xE96041E4, 0xE0677172, 0x0403E4D1, 0x0D04D447, 0x160D85FD, 0x1F0AB56B, 
		0x05B5A8FA, 0x0CB2986C, 0x17BBC9D6, 0x1EBCF940, 0xFAD86CE3, 0xF3DF5C75, 0xE8D60DCF, 0xE1D13D59, 
		0x06D930AC, 0x0FDE003A, 0x14D75180, 0x1DD06116, 0xF9B4F4B5, 0xF0B3C423, 0xEBBA9599, 0xE2BDA50F, 
		0xF802B89E, 0xF1058808, 0xEA0CD9B2, 0xE30BE924, 0x076F7C87, 0x0E684C11, 0x15611DAB, 0x1C662D3D, 
		0xF6DC4190, 0xFFDB7106, 0xE4D220BC, 0xEDD5102A, 0x09B18589, 0x00B6B51F, 0x1BBFE4A5, 0x12B8D433, 
		0x0807C9A2, 0x0100F934, 0x1A09A88E, 0x130E9818, 0xF76A0DBB, 0xFE6D3D2D, 0xE5646C97, 0xEC635C01,
		0x0B6B51F4, 0x026C6162, 0x196530D8, 0x1062004E, 0xF40695ED, 0xFD01A57B, 0xE608F4C1, 0xEF0FC457, 
		0xF5B0D9C6, 0xFCB7E950, 0xE7BEB8EA, 0xEEB9887C, 0x0ADD1DDF, 0x03DA2D49, 0x18D37CF3, 0x11D44C65, 
		0x0DB26158, 0x04B551CE, 0x1FBC0074, 0x16BB30E2, 0xF2DFA541, 0xFBD895D7, 0xE0D1C46D, 0xE9D6F4FB, 
		0xF369E96A, 0xFA6ED9FC, 0xE1678846, 0xE860B8D0, 0x0C042D73, 0x05031DE5, 0x1E0A4C5F, 0x170D7CC9, 
		0xF005713C, 0xF90241AA, 0xE20B1010, 0xEB0C2086, 0x0F68B525, 0x066F85B3, 0x1D66D409, 0x1461E49F, 
		0x0EDEF90E, 0x07D9C998, 0x1CD09822, 0x15D7A8B4, 0xF1B33D17, 0xF8B40D81, 0xE3BD5C3B, 0xEABA6CAD, 
		0xEDB88320, 0xE4BFB3B6, 0xFFB6E20C, 0xF6B1D29A, 0x12D54739, 0x1BD277AF, 0x00DB2615, 0x09DC1683, 
		0x13630B12, 0x1A643B84, 0x016D6A3E, 0x086A5AA8, 0xEC0ECF0B, 0xE509FF9D, 0xFE00AE27, 0xF7079EB1, 
		0x100F9344, 0x1908A3D2, 0x0201F268, 0x0B06C2FE, 0xEF62575D, 0xE66567CB, 0xFD6C3671, 0xF46B06E7, 
		0xEED41B76, 0xE7D32BE0, 0xFCDA7A5A, 0xF5DD4ACC, 0x11B9DF6F, 0x18BEEFF9, 0x03B7BE43, 0x0AB08ED5, 
		0x16D6A3E8, 0x1FD1937E, 0x04D8C2C4, 0x0DDFF252, 0xE9BB67F1, 0xE0BC5767, 0xFBB506DD, 0xF2B2364B, 
		0xE80D2BDA, 0xE10A1B4C, 0xFA034AF6, 0xF3047A60, 0x1760EFC3, 0x1E67DF55, 0x056E8EEF, 0x0C69BE79, 
		0xEB61B38C, 0xE266831A, 0xF96FD2A0, 0xF068E236, 0x140C7795, 0x1D0B4703, 0x060216B9, 0x0F05262F, 
		0x15BA3BBE, 0x1CBD0B28, 0x07B45A92, 0x0EB36A04, 0xEAD7FFA7, 0xE3D0CF31, 0xF8D99E8B, 0xF1DEAE1D, 
		0x1B64C2B0, 0x1263F226, 0x096AA39C, 0x006D930A, 0xE40906A9, 0xED0E363F, 0xF6076785, 0xFF005713, 
		0xE5BF4A82, 0xECB87A14, 0xF7B12BAE, 0xFEB61B38, 0x1AD28E9B, 0x13D5BE0D, 0x08DCEFB7, 0x01DBDF21, 
		0xE6D3D2D4, 0xEFD4E242, 0xF4DDB3F8, 0xFDDA836E, 0x19BE16CD, 0x10B9265B, 0x0BB077E1, 0x02B74777, 
		0x18085AE6, 0x110F6A70, 0x0A063BCA, 0x03010B5C, 0xE7659EFF, 0xEE62AE69, 0xF56BFFD3, 0xFC6CCF45, 
		0xE00AE278, 0xE90DD2EE, 0xF2048354, 0xFB03B3C2, 0x1F672661, 0x166016F7, 0x0D69474D, 0x046E77DB, 
		0x1ED16A4A, 0x17D65ADC, 0x0CDF0B66, 0x05D83BF0, 0xE1BCAE53, 0xE8BB9EC5, 0xF3B2CF7F, 0xFAB5FFE9, 
		0x1DBDF21C, 0x14BAC28A, 0x0FB39330, 0x06B4A3A6, 0xE2D03605, 0xEBD70693, 0xF0DE5729, 0xF9D967BF, 
		0xE3667A2E, 0xEA614AB8, 0xF1681B02, 0xF86F2B94, 0x1C0BBE37, 0x150C8EA1, 0x0E05DF1B, 0x0702EF8D
	];
	let mut check: u32 = !0xFEDCBA76;
	for v in buf {
		check = CRC_TABLE[(((check as u8 & 0xFF)) ^ v) as usize] ^ (check >> 8);
	}
	return !check;
	
}

fn main() {
	use slint::Model;
	let app = App::new().unwrap();
	
	let icons = {
		let images = imbed_images();
		let mut arr = vec![];
		for (im, s) in images.iter(){
			arr.push(AchievementIcon {image: im.clone(), name: s.into()});
		}
		arr
	};
	let achievements = load_achievement_data(app.global::<Search>().get_Savefile()).iter().enumerate().fold( Vec::<Achievement>::new(), |mut acc, (x, i)| {
		acc.push(Achievement {id: x as i32, unlocked: *i});
		acc
	});
	let vec = slint::VecModel::from(icons);
	
	let icons = std::rc::Rc::new(vec);
	let vec = slint::VecModel::from(achievements);
	let achievements = std::rc::Rc::new(vec);
	let weak1 = app.as_weak();
	let weak2 = app.as_weak();
	let saveapp = app.as_weak();
	app.global::<Search>().set_icons(icons.clone().into());
	app.global::<Search>().set_achievements(achievements.clone().into());
	let saveachievements = Rc::downgrade(&achievements);
	app.global::<Search>().set_indexes(Rc::new(slint::VecModel::from((0..637).collect::<Vec<i32>>())).into());
	app.global::<Search>().on_select_save(move |save| {
		let achievements = saveachievements.upgrade().unwrap();
		let save_data = load_achievement_data(save);
		achievements.set_vec(save_data.iter().enumerate().fold( Vec::<Achievement>::new(), |mut acc, (x, i)| {
			acc.push(Achievement {id: x as i32, unlocked: *i});
			acc
		}));
		
		
		
	});

	app.global::<Search>().on_range_change(move |x, y| {
		let app = weak1.upgrade().unwrap();
		if x > y || x < 0 || y > 637 {
			return;
		}
		app.global::<Search>().set_indexes(Rc::new(slint::VecModel::from(((x-1).max(0)..y).collect::<Vec<i32>>())).into());
	});
	app.global::<Search>().on_search_change(move |s| {
		let app = weak2.upgrade().unwrap();
		let sa: String = s.into();
		let n = NAMES.lines().into_iter().find_all( |st| st.contains(sa.as_str()));
		if let Some(ns) = n {
			app.global::<Search>().set_indexes(Rc::new(slint::VecModel::from(ns.iter().map(|x| *x as i32).collect::<Vec<i32>>())).into());

		}
	});
	let unlockweak = app.as_weak();
	app.global::<UnlockAchievements>().on_unlock( move || {
		let app = unlockweak.upgrade().unwrap();
		unlock_achievements(achievements.iter().fold( Vec::<u8>::new(), |mut acc, x| {
			acc.push(x.unlocked as u8);
			acc
		}), app.global::<Search>().get_Savefile());
	});


	app.run().unwrap();
}
