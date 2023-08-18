use std::collections::HashMap;

use find_all::FindAll;

//#![windows_subsystem = "windows"]
slint::slint! {
	import { GridBox , ScrollView, GroupBox, ListView, HorizontalBox, CheckBox, Button} from "std-widgets.slint";
	import "./src/upheavtt.ttf";
	struct AchievementIcon {
		image: image,
		name: string,
		unlocked: bool,
		id: int,
	}

	export global Search {
		callback range_change(int, int);
		callback search_change(string);

	}

	export component Icon inherits Image {
		width: 60px;
		height: 60px;
		in-out property <bool> has-unlocked;
		rect := Rectangle {
			callback pressed;
			ta:= TouchArea { 
				clicked() => {
					parent.pressed();
				}
			}
			pressed => {
				root.has-unlocked = !root.has-unlocked;
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
		out property <string> text;
		callback edited;
		HorizontalBox {
			alignment: start;
			spacing: 10px;
			Text{
				font-size: root.font-size;
				text: input-title;
			}
			textin:= TextInput {
				wrap: word-wrap;
				single-line: false;
				font-size: root.font-size;
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
		property <int> list-width: 16;//Math.max(Math.ceil(self.width / 100px), 2);
		property <int> list-height: Math.ceil(icons.length / list-width);
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
						border-width: 2px;
						background-color: gray.darker(40%);
						font-color: white;
						edited => {
							Search.range-change(range-from.text.to-float(), range-to.text.is-float() ? range-to.text.to-float(): 637);
						}
					}
				}
				CheckBox { 
					text: "Unlocked";
				}
				Button { 
					text: "Apply";
					preferred-width: 100px;
				 }
		}
			ListView  {
				for i in list-height : HorizontalLayout{
					padding: 4px;
					spacing: 4px;
					property <int> list_actual_width: Math.min(icons.length - i * list-width, list-width);
					for t in list_actual_width : VerticalLayout {
						property <int> index: t + i * list-width;
						width: 100px;
						Icon {
							
							source: icons[index].image;
							has-unlocked: icons[index].unlocked;
						}
						Text {
							text: icons[index].id;
							font-weight: 500;
							font-size: 16px;
							font-family: "Upheaval TT (BRK)";
						}Text {
							text: icons[index].name;
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

fn load_achievement_data() -> Vec<bool> {
	use std::{
		fs,
		path,
	};
	let bytes = fs::read(path::Path::new(r"E:\Steam\userdata\140201072\250900\remote\rep_persistentgamedata3.dat")).expect("Couldn't open file");
	bytes[33..637+33].iter().fold(Vec::<bool>::new(), |mut acc, x| {
		acc.push( *x != 0);
		acc
	})
}

fn change_icons(app: &App, vec: Vec<AchievementIcon>) {
	let vec = slint::VecModel::from(vec);

	let icons = std::rc::Rc::new(vec);
	
	app.set_icons(icons.into());
}

fn main() {
	use slint::Model;
	
	let achievements = {
		let images = imbed_images();
		let achievements = load_achievement_data();
		let mut arr = vec![];
		for (i, ((im, s), b)) in images.iter().zip(achievements.iter()).enumerate(){
			arr.push(AchievementIcon {image: im.clone(), name: s.into(), unlocked: *b, id: i as i32});
		}
		arr
	};
	let names = {
		let mut hash: HashMap<String, i32> = HashMap::new();
		for a in &achievements {
			hash.insert(a.name.clone().into(), a.id);
		}
		hash
	};
	let app = App::new().unwrap();
	let weak1 = app.as_weak();
	let weak2 = app.as_weak();
	let ra = achievements.clone();
	change_icons(&app, achievements.clone());
	app.global::<Search>().on_range_change(move |x, y| {
		let app = weak1.upgrade().unwrap();
		if x > y || x < 0 || y > 637 {
			return;
		}
		let vec = ra[x as usize..y as usize].to_vec();
		change_icons(&app, vec);
	});
	app.global::<Search>().on_search_change(move |s| {
		let app = weak2.upgrade().unwrap();
		let sa: String = s.into();
		let n = NAMES.lines().into_iter().find_all( |st| st.contains(sa.as_str()));
		let ac = {
			let mut v = vec![];
			if let Some(num) = n {
				for i in num {
					v.push(achievements.clone()[i as usize].clone());
				}
			}
			v
		};
		change_icons(&app, ac);
	});

	app.run().unwrap();
}
